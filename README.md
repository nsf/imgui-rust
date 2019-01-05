# nsf-imgui

Rust bindings for Dear ImGui 1.66b. Experimental personal project, created due to the following reasons:

1. Wanted to learn rust.
1. Mildly interested in game development.
1. Wasn't entirely happy with existing bindings.

## Goals

1. Generate most of FFI bindings automatically (helps with maintenance) using cimgui data files.
1. Generate most of native wrappers automatically as well.
1. Native wrappers should not be focused too much on safety, instead provide some level of convenience when it comes to types (refs to pointers, string management, etc.).
1. Closely follow original C++ API, hence avoid being paranoid about safety. E.g. as long as original lib allows you to break things with mismatched begin()/end() calls, allow that as well.

## Implementation details

1. A lot of code is generated using `generate.js` script. Generation is triggered manually, the repository includes generated files. To run this script you will need to install npm packages via `npm install`. In addition to `generate.js` script, see `gen.bash` script to understand how files are being generated.
1. Unsafe part uses "bitflags" crate to implement enums which act as bit flags.
1. Safe part is currently very limited. Only functions that are considered safe are being wrapped. E.g. ones which don't contain function pointers or `void*` or things like that.
1. `const char*` arguments become `&CStr`, there is also custom `cstr!` macro to help building those at compile-time. `const char*` return values are converted into `String`. Other types are propagated as-is, `*mut` becomes `&mut`, `*const` becomes `&`.
1. "Format" portion of the function arguments that normally consists of a format string and arguments is converted to a single string argument. Which is then passed in as `"%s"`.
1. Custom `cstr!` macro supports formatting, e.g. `cstr!("{}", myint)`. This variant uses `format!` macro underneath, which allocates a String.
1. All imgui functions are methods of `ImGui` struct. You can create one with `ImGui::new()`. Only one instance of this struct is allowed (protected via static mutex). Although the struct is Clone, Rc inside. `ImGui::new()` function will call `igCreateContext()`. When last `ImGui` instance is dropped, it will call `igDestroyContext(igGetCurrentContext())`.
1. Original library uses C types, such as int, unsigned int, float, double, size_t. I assume those are i32, u32, f32, f64, usize and so on. Basically implying they have fixed sizes. While it's not true in theory, in many cases it's true in practice. It will work for typical 64 bit desktop setups, it might not work for some esoteric embedded scenarios in case if you want to use imgui there.

## Q & A

Q: Where are the examples?

A: There are none. The goal for this lib is to stay as close as possible to C++ API, you can refer to original C++ examples. There are no safe wrappers around structs yet, hence if you want to integrate this lib into your renderer, you'll just fetch ImDrawData using FFI API and do it in a fully "unsafe" manner, just like you would do it in C++.

Q: Do you plan to maintain this library?

A: It depends. Currently I'm using it in my personal hobby gamedevy project. As long as I keep working on it, this library probably will be maintained to some degree. Then - who knows. Don't expect it to be well maintained.
