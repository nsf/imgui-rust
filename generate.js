const _ = require("lodash");
const changeCase = require('change-case');
const nunjucks = require("nunjucks");
const fs = require("fs");
const program = require("commander");

nunjucks.configure({
  autoescape: false
});

const definitions = JSON.parse(fs.readFileSync("data/definitions.json", "utf-8"));
const structsAndEnums = JSON.parse(fs.readFileSync("data/structs_and_enums.json", "utf-8"));
const typedefs = JSON.parse(fs.readFileSync("data/typedefs_dict.json", "utf-8"));

const blacklistedTypedefs = {
  // some C++ artifacts from templates
  'const_iterator': true,
  'iterator': true,
  'value_type': true,
}

const blacklistedStructs = {
  'Pair': true, // has a union inside, let's implement it manually
  'ImVector': true, // template type, let's implement it manually
}

const typedefsSorted = _.keys(typedefs).sort();

const typeMap = {
  'bool': 'bool',
  'char': 'c_char',
  'double': 'c_double',
  'float': 'c_float',
  'int': 'c_int',
  'int64_t': 'i64',
  'Pair': 'Pair',
  'short': 'c_short',
  'signed int': 'c_int', // iirc, "int == signed int" in C, "char != signed char" though
  'uint64_t': 'u64',
  'unsigned char': 'c_uchar',
  'unsigned int': 'c_uint',
  'unsigned short': 'c_ushort',
  'unsigned_char': 'c_uchar',
  'void': 'c_void',

  'bool(*)(void* data,int idx,const char** out_text)': 'extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char) -> bool',
  'const char*(*)(void* user_data)': 'Option<extern "C" fn(user_data: *mut c_void) -> *const c_char>',
  'float(*)(void* data,int idx)': 'extern "C" fn(data: *mut c_void, idx: c_int) -> c_float',
  'void(*)(int x,int y)': 'Option<extern "C" fn(x: c_int, y: c_int)>',
  'void(*)(void* ptr,void* user_data)': 'Option<extern "C" fn(ptr: *mut c_void, user_data: *mut c_void)>',
  'void(*)(void* user_data,const char* text)': 'Option<extern "C" fn(user_data: *mut c_void, text: *const c_char)>',
  'void*(*)(size_t sz,void* user_data)': 'Option<extern "C" fn(sz: usize, user_data: *mut c_void) -> *mut c_void>',

  'ImColor_Simple': 'ImColor_Simple',
  'ImDrawCallback': 'ImDrawCallback',
  'ImDrawListSharedData': 'ImDrawListSharedData',
  'ImFontPtr': 'ImFontPtr',
  'ImGuiContext': 'ImGuiContext',
  'ImGuiInputTextCallback': 'ImGuiInputTextCallback',
  'ImGuiSizeCallback': 'ImGuiSizeCallback',
  'ImVec2_Simple': 'ImVec2_Simple',
  'ImVec4_Simple': 'ImVec4_Simple',
  'size_t': 'size_t',
}

const snakeCaseManualCases = {
  'ColorConvertRGBtoHSV': 'color_convert_rgb_to_hsv',
  'ColorConvertHSVtoRGB': 'color_convert_hsv_to_rgb',
}

function snakeCase(s) {
  const v = snakeCaseManualCases[s];
  return v || changeCase.snakeCase(s);
}

function addKnownTypes() {
  const alreadyAdded = [];
  const addFrom = (collection, name) => {
    for (const v of collection) {
      if (typeMap[v]) {
        alreadyAdded.push({key: v, collection: name});
      } else {
        typeMap[v] = v;
      }
    }
  }
  addFrom(allEnumNames(), 'Enum');
  addFrom(allStructNames(), 'Struct');
  addFrom(allTypedefNames(), 'Typedef');
  if (alreadyAdded.length > 0) {
    throw new Error(`already added: ${_.join(_.map(alreadyAdded, JSON.stringify), ', ')}`);
  }
}

addKnownTypes();

function extractArity(name) {
  const re = /\[([^\]]+)\]/g;
  const m = re.exec(name);
  if (m) {
    const mm = /([^_]+)_(.+)/.exec(m[1]);
    if (mm) {
      return `${mm[1]}::${mm[2]} as usize`;
    }
    return m[1];
  }
  return '';
}

function stripArity(name) {
  return name.replace(/\[[^\]]+\]/g, '')
}

function mapType(t, opts) {
  const prefix = (opts && opts.prefix) || '';
  const arity = (opts && opts.arity) || '';

  // some manual overrides
  if (t === 'const char* const[]') {
    return '*const *const c_char';
  }

  if (t.indexOf("(*)") === -1) {
    // we mess with types, but only if they are not a function pointer, those are special

    if (_.startsWith(t, 'const ') && _.endsWith(t, '*')) {
      // it's not exactly a C syntax parser here, but for given cases it works,
      // converts "const Foo*" to "*const Foo"
      const nt = t.substr(0, t.length-1).substr('const '.length);
      return mapType(nt, { ...opts, prefix: `*const ${prefix}` });
    } else if (_.startsWith(t, 'const ')) {
      const nt = t.substr('const '.length);
      return mapType(nt, opts);
    }
    if (_.endsWith(t, '*') || _.endsWith(t, '&')) {
      // it's not exactly a C syntax parser here, but for given cases it works,
      // converts "Foo*" to "*mut Foo"
      return mapType(t.substr(0, t.length-1), { ...opts, prefix: `*mut ${prefix}` });
    }
    if (_.endsWith(t, ']')) {
      // arity in a type, happens in functions, in that case we need to convert it to a pointer to an array
      // C array types are just pointers
      const arity = extractArity(t);
      const nt = stripArity(t) + '*';
      return mapType(nt, { ...opts, arity });
    }
    if (_.startsWith(t, "ImVector_")) {
      return `${prefix}ImVector<${mapType(t.substr("ImVector_".length))}>`;
    }
  }
  const result = typeMap[t];
  if (!result) {
    throw new Error(`unknown type: ${prefix}${t}`);
  }
  if (arity) {
    return `${prefix}[${result}; ${arity}]`;
  } else {
    return `${prefix}${result}`;
  }
}

//=========================================================================================
// HEADER AND MANUALLY IMPLEMENTED THINGS
//=========================================================================================

function generateHeader() {
console.log(`
use bitflags::bitflags;
use std::os::raw::{c_char, c_double, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};
use libc::size_t;
use std::slice;

#[repr(C)]
pub struct ImVector<T> {
    size: c_int,
    capacity: c_int,
    data: *mut T,
}

impl<T> ImVector<T> {
    pub unsafe fn as_slice(&self) -> &[T] {
        slice::from_raw_parts(self.data, self.size as usize)
    }
}

#[repr(C)]
pub struct Pair {
    pub key: ImGuiID,
    pub value: PairValue,
}

#[repr(C)]
pub union PairValue {
    pub val_i: c_int,
    pub val_f: c_float,
    pub val_p: *mut c_void,
}

// opaque types
pub enum ImDrawListSharedData {}
pub enum ImGuiContext {}

pub type ImFontPtr = *mut ImFont;
pub type ImDrawCallback = Option<extern "C" fn(parent_list: *const ImDrawList, cmd: *const ImDrawCmd)>;
pub type ImGuiInputTextCallback = Option<extern "C" fn(data: *mut ImGuiInputTextCallbackData) -> c_int>;
pub type ImGuiSizeCallback = Option<extern "C" fn(data: *mut ImGuiSizeCallbackData)>;
`.substr(1));
}

//=========================================================================================
// TYPEDEFS
//=========================================================================================

function allTypedefNames() {
  return _.filter(typedefsSorted, typedef => !isTypedefBlacklisted(typedef, typedefs[typedef]));
}

function isTypedefBlacklisted(typedef, val) {
  if (_.startsWith(val, "struct ")) { // not interested in struct typedefs
    return true;
  }
  if (val.indexOf("(*)") !== -1) { // function pointer typedefs are done manually
    return true;
  }
  if (blacklistedTypedefs[typedef]) {
    return true;
  }
  if (structsAndEnums.enums[typedef+"_"]) { // not interested in enums either
    return true;
  }
  return false;
}

function generateTypedefs() {
  for (const typedef of typedefsSorted) {
    const val = typedefs[typedef];
    if (isTypedefBlacklisted(typedef, val)) {
      continue;
    }

    console.log(`pub type ${typedef} = ${mapType(val)};`);
  }
}

//=========================================================================================
// ENUMS
//=========================================================================================

const bitflagsTemplate = `
bitflags! {
  #[repr(C)]
  pub struct {{ name }}: {{ type }} {
    {%- for f in fields %}
      const {{ f.name }} = {{ f.value }};
    {%- endfor %}
  }
}
`;

const enumTemplate = `
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum {{ name }} {
  {%- for f in fields %}
  {{ f.name }} = {{ f.value }},
  {%- endfor %}
}
`;

function isBitFlags(fields) {
  return _.some(fields, f => _.isString(f.value) && f.value.indexOf("<<") !== -1);
}

function processBitFlagsValue(v, nameRaw) {
  if (!_.isString(v)) {
    return v;
  }
  const re = new RegExp(nameRaw+`([A-Za-z0-9_]+)`, 'g');
  return v.replace(re, (m, name) => {
    return `Self::${name}.bits`;
  });
}

function allEnumNames() {
  return _.map(_.keys(structsAndEnums.enums), e => _.trimEnd(e, '_'));
}

function generateEnums() {
  for (const nameRaw of _.keys(structsAndEnums.enums)) {
    const fields = structsAndEnums.enums[nameRaw];
    const name = _.trimEnd(nameRaw, '_');
    const data = {
      name,
      type: mapType(typedefs[name]),
      fields: _.map(fields, f => ({
        ...f,
        name: _.startsWith(f.name, nameRaw) ? f.name.substr(nameRaw.length) : f.name,
        value: processBitFlagsValue(f.value, nameRaw),
      })),
    };
    if (isBitFlags(fields)) {
      const result = nunjucks.renderString(bitflagsTemplate, data);
      console.log(result);
    } else {
      const valsMap = {};
      data.fields = _.filter(data.fields, f => {
        if (valsMap[f.calc_value]) {
          return false;
        } else {
          valsMap[f.calc_value] = true;
          return true;
        }
      });
      const result = nunjucks.renderString(enumTemplate, data);
      console.log(result);
    }
  }
}

//=========================================================================================
// STRUCTS
//=========================================================================================

const structTemplate = `
#[repr(C)]
{%- if simple %}
#[derive(Copy, Clone)]{% endif %}
pub struct {{ name }} {
  {%- for f in fields %}
  pub {{ f.name }}: {{ f.type }},
  {%- endfor %}
}
`;

const simpleStructs = {
  'ImVec2': true,
  'ImVec4': true,
}

function allStructNames() {
  return _.filter(_.keys(structsAndEnums.structs), s => !blacklistedStructs[s]);
}

function generateStructs() {
  for (const name of _.keys(structsAndEnums.structs)) {
    if (blacklistedStructs[name]) {
      continue;
    }

    const fields = structsAndEnums.structs[name];
    const result = nunjucks.renderString(structTemplate, {
      name,
      simple: !!simpleStructs[name],
      fields: _.map(fields, f => ({
        type: mapType(f.type, { arity: extractArity(f.name) }),
        name: snakeCase(stripArity(f.name)),
      })),
    });
    console.log(result);
  }
}

//=========================================================================================
// FUNCTIONS
//=========================================================================================

const funcTemplate = `
pub fn {{name}}(
  {%- for a in args -%}
    {%- if not loop.first %}, {% endif %}{{ a.name }}{% if a.type %}: {% endif %}{{ a.type -}}
  {% endfor -%}
)
{%- if ret %} -> {{ ret }}{% endif %};
`;

const safeArgNameMap = {
  'type': '_type',
  'ref': '_ref',
  'self': '_self',
  'in': '_in',
}

function safeArgName(v) {
  return safeArgNameMap[v] || v;
}

function generateFuncs() {
  console.log(`extern "C" {`);
  for (const name of _.keys(definitions)) {
    const funcs = definitions[name];
    for (const func of funcs) {
      if (func.nonUDT) { // rust works just fine here
        continue;
      }
      if (_.some(func.argsT, a => a.type === 'va_list')) {
        // skip va_list variants, we're fine with "..." notation, in most cases it's about string formatting
        // and we'll just use ("%s", str)
        continue;
      }
      const name = func.ov_cimguiname || func.cimguiname;
      const ret = func.ret && func.ret !== 'void' ? mapType(func.ret) : undefined;
      const args = _.map(func.argsT, a => ({
        name: safeArgName(a.name),
        type: a.name === '...' ? undefined : mapType(a.type),
      }));
      const result = nunjucks.renderString(funcTemplate, {
        name,
        ret,
        args,
      });
      console.log(_.trim(result));
    }
  }
  console.log(`} // extern "C"`);
}

//=========================================================================================
// SAFE WRAPPERS
//=========================================================================================

const blacklistedSafeFuncs = {
  'igCreateContext': true, // calls to it are made by ImGui object
  'igDestroyContext': true, // calls to it are made by ImGui object
  'igMemAlloc': true, // raw memory
  'igMemFree': true, // raw memory
  'igSetAllocatorFunctions': true, // use unsafe interface if you want to override those
  'igSetCurrentContext': true, // currently we assume only one context per lib usage
  'igTextUnformatted': true, // has "end" pointer, TODO: we could make &str interface for it
}

const safeArgTypeConv = {
  '*const c_char': v => `${v}.as_ptr()`,
};

const safeRetTypeConv = {
  '*const c_char': v => `CStr::from_ptr(${v}).to_string_lossy().into_owned()`,
};

const safeTypeMap = {
  '*const ImVec2': (g) => `&'${g.next()} ImVec2`,
  '*const c_char': (g) => `&'${g.next()} CStr`,
  '*const c_float': (g) => `&'${g.next()} f32`,
  '*mut [c_float; 2]': (g) => `&'${g.next()} mut [f32; 2]`,
  '*mut [c_float; 3]': (g) => `&'${g.next()} mut [f32; 3]`,
  '*mut [c_float; 4]': (g) => `&'${g.next()} mut [f32; 4]`,
  '*mut [c_int; 2]': (g) => `&'${g.next()} mut [i32; 2]`,
  '*mut [c_int; 3]': (g) => `&'${g.next()} mut [i32; 3]`,
  '*mut [c_int; 4]': (g) => `&'${g.next()} mut [i32; 4]`,
  '*mut bool': (g) => `&'${g.next()} mut bool`,
  '*mut c_double': (g) => `&'${g.next()} mut f64`,
  '*mut c_float': (g) => `&'${g.next()} mut f32`,
  '*mut c_int': (g) => `&'${g.next()} mut i32`,
  '*mut c_uint': (g) => `&'${g.next()} mut u32`,
  '*mut size_t': (g) => `&'${g.next()} mut usize`,
  'ImGuiCol': () => 'ImGuiCol',
  'ImGuiColorEditFlags': () => 'ImGuiColorEditFlags',
  'ImGuiComboFlags': () => 'ImGuiComboFlags',
  'ImGuiCond': () => 'ImGuiCond',
  'ImGuiDataType': () => 'ImGuiDataType',
  'ImGuiDir': () => 'ImGuiDir',
  'ImGuiDragDropFlags': () => 'ImGuiDragDropFlags',
  'ImGuiFocusedFlags': () => 'ImGuiFocusedFlags',
  'ImGuiHoveredFlags': () => 'ImGuiHoveredFlags',
  'ImGuiID': () => 'ImGuiID',
  'ImGuiInputTextCallback': () => 'ImGuiInputTextCallback',
  'ImGuiInputTextFlags': () => 'ImGuiInputTextFlags',
  'ImGuiKey': () => 'ImGuiKey',
  'ImGuiMouseCursor': () => 'ImGuiMouseCursor',
  'ImGuiSelectableFlags': () => 'ImGuiSelectableFlags',
  'ImGuiSizeCallback': () => 'ImGuiSizeCallback',
  'ImGuiStyleVar': () => 'ImGuiStyleVar',
  'ImGuiTreeNodeFlags': () => 'ImGuiTreeNodeFlags',
  'ImGuiWindowFlags': () => 'ImGuiWindowFlags',
  'ImTextureID': () => 'ImTextureID',
  'ImU32': () => 'u32',
  'ImVec2': () => 'ImVec2',
  'ImVec4': () => 'ImVec4',
  'bool': () => 'bool',
  'c_double': () => 'f64',
  'c_float': () => 'f32',
  'c_int': () => 'i32',
  'c_uint': () => 'u32',
  'size_t': () => 'usize',
};

const safeRetTypeMap = {
  ...safeTypeMap,
  '*const c_char': () => 'String',
};

const safeTemplate = `
#[inline]
pub fn {{ sname }}<{{ lifetimes }}>(&'a self
  {%- for a in args -%}
    , {{ a.name }}{% if a.stype %}: {% endif %}{{ a.stype -}}
  {% endfor -%}
)
{%- if sret %} -> {{ sret }}{% endif %} {
    {% set funccall %}
      {{- name }}(
        {%- for a in args -%}
          {%- if not loop.first %}, {% endif %}{{ a.aconv -}}
        {% endfor -%}
      )
    {%- endset -%}
    unsafe { {{ retconv(funccall) }} }{% if not sret %};{% endif %}
}
`;

const defaultValueMap = {
  '"%.0f deg"': t => 'cstr_ptr!("%.0f deg")',
  '"%.3f"': t => 'cstr_ptr!("%.3f")',
  '"%.6f"': t => 'cstr_ptr!("%.6f")',
  '"%d"': t => 'cstr_ptr!("%d")',
  '((void *)0)': t => _.startsWith(t, '*mut') ? '::std::ptr::null_mut()' : '::std::ptr::null()',
  '+360.0f': t => '360.0',
  '-1': t => '-1',
  '-1.0f': t => '-1.0',
  '-360.0f': t => '-360.0',
  '0': t => _.endsWith(t, 'Flags') || t === 'ImGuiCond' ? `${t}::empty()` : '0',
  '0.0f': t => '0.0',
  '0.5f': t => '0.5',
  '1': t => '1',
  '1.0f': t => '1.0',
  '100': t => '100',
  'FLT_MAX': t => '::std::f32::MAX',
  'ImVec2(-1,0)': t => 'ImVec2{x:-1.0,y:0.0}',
  'ImVec2(0,0)': t => 'ImVec2{x:0.0,y:0.0}',
  'ImVec2(1,1)': t => 'ImVec2{x:1.0,y:1.0}',
  'ImVec4(0,0,0,0)': t => 'ImVec4{x:0.0,y:0.0,z:0.0,w:0.0}',
  'ImVec4(1,1,1,1)': t => 'ImVec4{x:1.0,y:1.0,z:1.0,w:1.0}',
  'false': t => 'false',
  'sizeof(float)': t => '::std::mem::size_of::<f32>() as i32',
  'true': t => 'true',
};

function indentMultilineString(indent, str) {
  return _.join(_.map(_.split(str, '\n'), li => `${indent}${li}`), '\n');
}

function convertVarargs(args) {
  const n = args.length;
  if (n >= 2 && args[n-1].type === undefined && args[n-2].name === 'fmt') {
    args.splice(n-2, 2, {
      ...args[n-2],
      aconv: `cstr_ptr!("%s"), ${args[n-2].name}.as_ptr()`,
    });
  }
  return args;
}

const lifetimeNames = 'abcdefghijklmnopqrstuvwxyz';

class LifetimeGen {
  constructor() {
    this.i = 1;
  }
  next() {
    return lifetimeNames[this.i++];
  }
}

function generateSafe() {
  console.log("use super::{cstr_ptr, ImGui};");
  console.log("use nsf_imgui_raw::*;");
  console.log("use std::ffi::CStr;");
  console.log("impl ImGui {");
  const defaultConv = v => v;
  const indent = "    ";
  const typesUsedInAPI = {};
  for (const name of _.keys(definitions)) {
    const funcs = definitions[name];
    for (const func of funcs) {
      if (func.nonUDT) { // rust works fine with UDTs
        continue;
      }
      if (_.some(func.argsT, a => a.type === 'va_list')) { // not interesting in va_list funcs
        continue;
      }
      const rawName = func.ov_cimguiname || func.cimguiname;
      if (!_.startsWith(rawName, 'ig')) { // TODO: not interested in methods just yet
        continue;
      }
      if (blacklistedSafeFuncs[rawName]) { // these are not safe
        continue;
      }

      const name = snakeCase(rawName.substr(2)); // remove 'ig' prefix and convert_to_snake_case
      const ret = func.ret && func.ret !== 'void' ? mapType(func.ret) : undefined;
      const args = _.map(func.argsT, a => ({
        name: safeArgName(a.name),
        type: a.name === '...' ? undefined : mapType(a.type),
        def: func.defaults[a.name],
      }));
      if (_.every(args, a => a.type === undefined || !!safeTypeMap[a.type]) && (!ret || !!safeRetTypeMap[ret])) {
        const lifetimeGen = new LifetimeGen();
        const data = {
          name: rawName,
          sname: name,
          ret,
          retconv: safeRetTypeConv[ret] || defaultConv,
          sret: safeRetTypeMap[ret] ? safeRetTypeMap[ret]() : undefined,
          args: convertVarargs(_.map(args, a => {
            const stypef = safeTypeMap[a.type];
            const stype = stypef ? (a.def !== undefined ? `impl Into<Option<${stypef(lifetimeGen)}>>` : stypef(lifetimeGen)) : undefined;
            const convf = safeArgTypeConv[a.type] || defaultConv;
            return {
              ...a,
              stype,
              aconv: a.def ?
                `match ${a.name}.into() { Some(v) => ${convf("v")}, None => ${defaultValueMap[a.def](a.type)} }` :
                convf(a.name),
            };
          })),
        };
        data.lifetimes = _.join(_.map(_.range(lifetimeGen.i), i => `'${lifetimeNames[i]}`), ', ');
        for (const a of data.args) {
          const t = _.startsWith(a.stype, "impl Into<Option<") ?
            a.stype.substring("impl Into<Option<".length, a.stype.length-2) :
            a.stype;
          if (t && t[0] !== '&' && _.toUpper(t[0]) === t[0]) {
            typesUsedInAPI[t] = true;
          }
        }
        const result = nunjucks.renderString(safeTemplate, data);
        console.log(indentMultilineString(indent, _.trim(result)));
      } else {
        console.log(indent + `// pub fn ${name} : (${_.join(_.map(args, a => a.type), ', ')}) -> ${ret}`);
      }
    }
  }
  console.log("} // impl ImGui");
  console.log("pub mod types_used {");
  for (const t of _.keys(typesUsedInAPI).sort()) {
    console.log(indent + `pub use nsf_imgui_raw::${t};`);
  }
  console.log("} // mod types_used");
}

//=========================================================================================
//=========================================================================================
//=========================================================================================

program
  .command("raw")
  .action(() => {
    generateHeader();
    generateTypedefs();
    generateEnums();
    generateStructs();
    generateFuncs();
  });

program
  .command("safe")
  .action(() => {
    generateSafe();
  });

program.parse(process.argv);