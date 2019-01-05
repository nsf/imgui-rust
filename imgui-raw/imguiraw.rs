use bitflags::bitflags;
use libc::size_t;
use std::os::raw::{c_char, c_double, c_float, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};
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

pub type ImDrawIdx = c_ushort;
pub type ImGuiColumnsFlags = c_int;
pub type ImGuiID = c_uint;
pub type ImS32 = c_int;
pub type ImS64 = i64;
pub type ImTextureID = *mut c_void;
pub type ImU32 = c_uint;
pub type ImU64 = u64;
pub type ImWchar = c_ushort;

bitflags! {
  #[repr(C)]
  pub struct ImDrawCornerFlags: c_int {
      const TopLeft = 1 << 0;
      const TopRight = 1 << 1;
      const BotLeft = 1 << 2;
      const BotRight = 1 << 3;
      const Top = Self::TopLeft.bits | Self::TopRight.bits;
      const Bot = Self::BotLeft.bits | Self::BotRight.bits;
      const Left = Self::TopLeft.bits | Self::BotLeft.bits;
      const Right = Self::TopRight.bits | Self::BotRight.bits;
      const All = 0xF;
  }
}

bitflags! {
  #[repr(C)]
  pub struct ImDrawListFlags: c_int {
      const None = 0;
      const AntiAliasedLines = 1 << 0;
      const AntiAliasedFill = 1 << 1;
  }
}

bitflags! {
  #[repr(C)]
  pub struct ImFontAtlasFlags: c_int {
      const None = 0;
      const NoPowerOfTwoHeight = 1 << 0;
      const NoMouseCursors = 1 << 1;
  }
}

bitflags! {
  #[repr(C)]
  pub struct ImGuiBackendFlags: c_int {
      const None = 0;
      const HasGamepad = 1 << 0;
      const HasMouseCursors = 1 << 1;
      const HasSetMousePos = 1 << 2;
  }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiCol {
    Text = 0,
    TextDisabled = 1,
    WindowBg = 2,
    ChildBg = 3,
    PopupBg = 4,
    Border = 5,
    BorderShadow = 6,
    FrameBg = 7,
    FrameBgHovered = 8,
    FrameBgActive = 9,
    TitleBg = 10,
    TitleBgActive = 11,
    TitleBgCollapsed = 12,
    MenuBarBg = 13,
    ScrollbarBg = 14,
    ScrollbarGrab = 15,
    ScrollbarGrabHovered = 16,
    ScrollbarGrabActive = 17,
    CheckMark = 18,
    SliderGrab = 19,
    SliderGrabActive = 20,
    Button = 21,
    ButtonHovered = 22,
    ButtonActive = 23,
    Header = 24,
    HeaderHovered = 25,
    HeaderActive = 26,
    Separator = 27,
    SeparatorHovered = 28,
    SeparatorActive = 29,
    ResizeGrip = 30,
    ResizeGripHovered = 31,
    ResizeGripActive = 32,
    PlotLines = 33,
    PlotLinesHovered = 34,
    PlotHistogram = 35,
    PlotHistogramHovered = 36,
    TextSelectedBg = 37,
    DragDropTarget = 38,
    NavHighlight = 39,
    NavWindowingHighlight = 40,
    NavWindowingDimBg = 41,
    ModalWindowDimBg = 42,
    COUNT = 43,
}

bitflags! {
  #[repr(C)]
  pub struct ImGuiColorEditFlags: c_int {
      const None = 0;
      const NoAlpha = 1 << 1;
      const NoPicker = 1 << 2;
      const NoOptions = 1 << 3;
      const NoSmallPreview = 1 << 4;
      const NoInputs = 1 << 5;
      const NoTooltip = 1 << 6;
      const NoLabel = 1 << 7;
      const NoSidePreview = 1 << 8;
      const NoDragDrop = 1 << 9;
      const AlphaBar = 1 << 16;
      const AlphaPreview = 1 << 17;
      const AlphaPreviewHalf = 1 << 18;
      const HDR = 1 << 19;
      const RGB = 1 << 20;
      const HSV = 1 << 21;
      const HEX = 1 << 22;
      const Uint8 = 1 << 23;
      const Float = 1 << 24;
      const PickerHueBar = 1 << 25;
      const PickerHueWheel = 1 << 26;
      const _InputsMask = Self::RGB.bits|Self::HSV.bits|Self::HEX.bits;
      const _DataTypeMask = Self::Uint8.bits|Self::Float.bits;
      const _PickerMask = Self::PickerHueWheel.bits|Self::PickerHueBar.bits;
      const _OptionsDefault = Self::Uint8.bits|Self::RGB.bits|Self::PickerHueBar.bits;
  }
}

bitflags! {
  #[repr(C)]
  pub struct ImGuiComboFlags: c_int {
      const None = 0;
      const PopupAlignLeft = 1 << 0;
      const HeightSmall = 1 << 1;
      const HeightRegular = 1 << 2;
      const HeightLarge = 1 << 3;
      const HeightLargest = 1 << 4;
      const NoArrowButton = 1 << 5;
      const NoPreview = 1 << 6;
      const HeightMask_ = Self::HeightSmall.bits | Self::HeightRegular.bits | Self::HeightLarge.bits | Self::HeightLargest.bits;
  }
}

bitflags! {
  #[repr(C)]
  pub struct ImGuiCond: c_int {
      const Always = 1 << 0;
      const Once = 1 << 1;
      const FirstUseEver = 1 << 2;
      const Appearing = 1 << 3;
  }
}

bitflags! {
  #[repr(C)]
  pub struct ImGuiConfigFlags: c_int {
      const None = 0;
      const NavEnableKeyboard = 1 << 0;
      const NavEnableGamepad = 1 << 1;
      const NavEnableSetMousePos = 1 << 2;
      const NavNoCaptureKeyboard = 1 << 3;
      const NoMouse = 1 << 4;
      const NoMouseCursorChange = 1 << 5;
      const IsSRGB = 1 << 20;
      const IsTouchScreen = 1 << 21;
  }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiDataType {
    S32 = 0,
    U32 = 1,
    S64 = 2,
    U64 = 3,
    Float = 4,
    Double = 5,
    COUNT = 6,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiDir {
    None = -1,
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
    COUNT = 4,
}

bitflags! {
  #[repr(C)]
  pub struct ImGuiDragDropFlags: c_int {
      const None = 0;
      const SourceNoPreviewTooltip = 1 << 0;
      const SourceNoDisableHover = 1 << 1;
      const SourceNoHoldToOpenOthers = 1 << 2;
      const SourceAllowNullID = 1 << 3;
      const SourceExtern = 1 << 4;
      const SourceAutoExpirePayload = 1 << 5;
      const AcceptBeforeDelivery = 1 << 10;
      const AcceptNoDrawDefaultRect = 1 << 11;
      const AcceptNoPreviewTooltip = 1 << 12;
      const AcceptPeekOnly = Self::AcceptBeforeDelivery.bits | Self::AcceptNoDrawDefaultRect.bits;
  }
}

bitflags! {
  #[repr(C)]
  pub struct ImGuiFocusedFlags: c_int {
      const None = 0;
      const ChildWindows = 1 << 0;
      const RootWindow = 1 << 1;
      const AnyWindow = 1 << 2;
      const RootAndChildWindows = Self::RootWindow.bits | Self::ChildWindows.bits;
  }
}

bitflags! {
  #[repr(C)]
  pub struct ImGuiHoveredFlags: c_int {
      const None = 0;
      const ChildWindows = 1 << 0;
      const RootWindow = 1 << 1;
      const AnyWindow = 1 << 2;
      const AllowWhenBlockedByPopup = 1 << 3;
      const AllowWhenBlockedByActiveItem = 1 << 5;
      const AllowWhenOverlapped = 1 << 6;
      const AllowWhenDisabled = 1 << 7;
      const RectOnly = Self::AllowWhenBlockedByPopup.bits | Self::AllowWhenBlockedByActiveItem.bits | Self::AllowWhenOverlapped.bits;
      const RootAndChildWindows = Self::RootWindow.bits | Self::ChildWindows.bits;
  }
}

bitflags! {
  #[repr(C)]
  pub struct ImGuiInputTextFlags: c_int {
      const None = 0;
      const CharsDecimal = 1 << 0;
      const CharsHexadecimal = 1 << 1;
      const CharsUppercase = 1 << 2;
      const CharsNoBlank = 1 << 3;
      const AutoSelectAll = 1 << 4;
      const EnterReturnsTrue = 1 << 5;
      const CallbackCompletion = 1 << 6;
      const CallbackHistory = 1 << 7;
      const CallbackAlways = 1 << 8;
      const CallbackCharFilter = 1 << 9;
      const AllowTabInput = 1 << 10;
      const CtrlEnterForNewLine = 1 << 11;
      const NoHorizontalScroll = 1 << 12;
      const AlwaysInsertMode = 1 << 13;
      const ReadOnly = 1 << 14;
      const Password = 1 << 15;
      const NoUndoRedo = 1 << 16;
      const CharsScientific = 1 << 17;
      const CallbackResize = 1 << 18;
      const Multiline = 1 << 20;
  }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiKey {
    Tab = 0,
    LeftArrow = 1,
    RightArrow = 2,
    UpArrow = 3,
    DownArrow = 4,
    PageUp = 5,
    PageDown = 6,
    Home = 7,
    End = 8,
    Insert = 9,
    Delete = 10,
    Backspace = 11,
    Space = 12,
    Enter = 13,
    Escape = 14,
    A = 15,
    C = 16,
    V = 17,
    X = 18,
    Y = 19,
    Z = 20,
    COUNT = 21,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiMouseCursor {
    None = -1,
    Arrow = 0,
    TextInput = 1,
    ResizeAll = 2,
    ResizeNS = 3,
    ResizeEW = 4,
    ResizeNESW = 5,
    ResizeNWSE = 6,
    Hand = 7,
    COUNT = 8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiNavInput {
    Activate = 0,
    Cancel = 1,
    Input = 2,
    Menu = 3,
    DpadLeft = 4,
    DpadRight = 5,
    DpadUp = 6,
    DpadDown = 7,
    LStickLeft = 8,
    LStickRight = 9,
    LStickUp = 10,
    LStickDown = 11,
    FocusPrev = 12,
    FocusNext = 13,
    TweakSlow = 14,
    TweakFast = 15,
    KeyMenu_ = 16,
    KeyLeft_ = 17,
    KeyRight_ = 18,
    KeyUp_ = 19,
    KeyDown_ = 20,
    COUNT = 21,
}

bitflags! {
  #[repr(C)]
  pub struct ImGuiSelectableFlags: c_int {
      const None = 0;
      const DontClosePopups = 1 << 0;
      const SpanAllColumns = 1 << 1;
      const AllowDoubleClick = 1 << 2;
      const Disabled = 1 << 3;
  }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ImGuiStyleVar {
    Alpha = 0,
    WindowPadding = 1,
    WindowRounding = 2,
    WindowBorderSize = 3,
    WindowMinSize = 4,
    WindowTitleAlign = 5,
    ChildRounding = 6,
    ChildBorderSize = 7,
    PopupRounding = 8,
    PopupBorderSize = 9,
    FramePadding = 10,
    FrameRounding = 11,
    FrameBorderSize = 12,
    ItemSpacing = 13,
    ItemInnerSpacing = 14,
    IndentSpacing = 15,
    ScrollbarSize = 16,
    ScrollbarRounding = 17,
    GrabMinSize = 18,
    GrabRounding = 19,
    ButtonTextAlign = 20,
    COUNT = 21,
}

bitflags! {
  #[repr(C)]
  pub struct ImGuiTreeNodeFlags: c_int {
      const None = 0;
      const Selected = 1 << 0;
      const Framed = 1 << 1;
      const AllowItemOverlap = 1 << 2;
      const NoTreePushOnOpen = 1 << 3;
      const NoAutoOpenOnLog = 1 << 4;
      const DefaultOpen = 1 << 5;
      const OpenOnDoubleClick = 1 << 6;
      const OpenOnArrow = 1 << 7;
      const Leaf = 1 << 8;
      const Bullet = 1 << 9;
      const FramePadding = 1 << 10;
      const NavLeftJumpsBackHere = 1 << 13;
      const CollapsingHeader = Self::Framed.bits | Self::NoTreePushOnOpen.bits | Self::NoAutoOpenOnLog.bits;
  }
}

bitflags! {
  #[repr(C)]
  pub struct ImGuiWindowFlags: c_int {
      const None = 0;
      const NoTitleBar = 1 << 0;
      const NoResize = 1 << 1;
      const NoMove = 1 << 2;
      const NoScrollbar = 1 << 3;
      const NoScrollWithMouse = 1 << 4;
      const NoCollapse = 1 << 5;
      const AlwaysAutoResize = 1 << 6;
      const NoBackground = 1 << 7;
      const NoSavedSettings = 1 << 8;
      const NoMouseInputs = 1 << 9;
      const MenuBar = 1 << 10;
      const HorizontalScrollbar = 1 << 11;
      const NoFocusOnAppearing = 1 << 12;
      const NoBringToFrontOnFocus = 1 << 13;
      const AlwaysVerticalScrollbar = 1 << 14;
      const AlwaysHorizontalScrollbar = 1<< 15;
      const AlwaysUseWindowPadding = 1 << 16;
      const NoNavInputs = 1 << 18;
      const NoNavFocus = 1 << 19;
      const NoNav = Self::NoNavInputs.bits | Self::NoNavFocus.bits;
      const NoDecoration = Self::NoTitleBar.bits | Self::NoResize.bits | Self::NoScrollbar.bits | Self::NoCollapse.bits;
      const NoInputs = Self::NoMouseInputs.bits | Self::NoNavInputs.bits | Self::NoNavFocus.bits;
      const NavFlattened = 1 << 23;
      const ChildWindow = 1 << 24;
      const Tooltip = 1 << 25;
      const Popup = 1 << 26;
      const Modal = 1 << 27;
      const ChildMenu = 1 << 28;
  }
}

#[repr(C)]
pub struct CustomRect {
    pub id: c_uint,
    pub width: c_ushort,
    pub height: c_ushort,
    pub x: c_ushort,
    pub y: c_ushort,
    pub glyph_advance_x: c_float,
    pub glyph_offset: ImVec2,
    pub font: *mut ImFont,
}

#[repr(C)]
pub struct GlyphRangesBuilder {
    pub used_chars: ImVector<c_uchar>,
}

#[repr(C)]
pub struct ImColor {
    pub value: ImVec4,
}

#[repr(C)]
pub struct ImDrawChannel {
    pub cmd_buffer: ImVector<ImDrawCmd>,
    pub idx_buffer: ImVector<ImDrawIdx>,
}

#[repr(C)]
pub struct ImDrawCmd {
    pub elem_count: c_uint,
    pub clip_rect: ImVec4,
    pub texture_id: ImTextureID,
    pub user_callback: ImDrawCallback,
    pub user_callback_data: *mut c_void,
}

#[repr(C)]
pub struct ImDrawData {
    pub valid: bool,
    pub cmd_lists: *mut *mut ImDrawList,
    pub cmd_lists_count: c_int,
    pub total_idx_count: c_int,
    pub total_vtx_count: c_int,
    pub display_pos: ImVec2,
    pub display_size: ImVec2,
}

#[repr(C)]
pub struct ImDrawList {
    pub cmd_buffer: ImVector<ImDrawCmd>,
    pub idx_buffer: ImVector<ImDrawIdx>,
    pub vtx_buffer: ImVector<ImDrawVert>,
    pub flags: ImDrawListFlags,
    pub data: *const ImDrawListSharedData,
    pub owner_name: *const c_char,
    pub vtx_current_idx: c_uint,
    pub vtx_write_ptr: *mut ImDrawVert,
    pub idx_write_ptr: *mut ImDrawIdx,
    pub clip_rect_stack: ImVector<ImVec4>,
    pub texture_id_stack: ImVector<ImTextureID>,
    pub path: ImVector<ImVec2>,
    pub channels_current: c_int,
    pub channels_count: c_int,
    pub channels: ImVector<ImDrawChannel>,
}

#[repr(C)]
pub struct ImDrawVert {
    pub pos: ImVec2,
    pub uv: ImVec2,
    pub col: ImU32,
}

#[repr(C)]
pub struct ImFont {
    pub font_size: c_float,
    pub scale: c_float,
    pub display_offset: ImVec2,
    pub glyphs: ImVector<ImFontGlyph>,
    pub index_advance_x: ImVector<c_float>,
    pub index_lookup: ImVector<ImWchar>,
    pub fallback_glyph: *const ImFontGlyph,
    pub fallback_advance_x: c_float,
    pub fallback_char: ImWchar,
    pub config_data_count: c_short,
    pub config_data: *mut ImFontConfig,
    pub container_atlas: *mut ImFontAtlas,
    pub ascent: c_float,
    pub descent: c_float,
    pub dirty_lookup_tables: bool,
    pub metrics_total_surface: c_int,
}

#[repr(C)]
pub struct ImFontAtlas {
    pub locked: bool,
    pub flags: ImFontAtlasFlags,
    pub tex_id: ImTextureID,
    pub tex_desired_width: c_int,
    pub tex_glyph_padding: c_int,
    pub tex_pixels_alpha8: *mut c_uchar,
    pub tex_pixels_rgba32: *mut c_uint,
    pub tex_width: c_int,
    pub tex_height: c_int,
    pub tex_uv_scale: ImVec2,
    pub tex_uv_white_pixel: ImVec2,
    pub fonts: ImVector<ImFontPtr>,
    pub custom_rects: ImVector<CustomRect>,
    pub config_data: ImVector<ImFontConfig>,
    pub custom_rect_ids: [c_int; 1],
}

#[repr(C)]
pub struct ImFontConfig {
    pub font_data: *mut c_void,
    pub font_data_size: c_int,
    pub font_data_owned_by_atlas: bool,
    pub font_no: c_int,
    pub size_pixels: c_float,
    pub oversample_h: c_int,
    pub oversample_v: c_int,
    pub pixel_snap_h: bool,
    pub glyph_extra_spacing: ImVec2,
    pub glyph_offset: ImVec2,
    pub glyph_ranges: *const ImWchar,
    pub glyph_min_advance_x: c_float,
    pub glyph_max_advance_x: c_float,
    pub merge_mode: bool,
    pub rasterizer_flags: c_uint,
    pub rasterizer_multiply: c_float,
    pub name: [c_char; 40],
    pub dst_font: *mut ImFont,
}

#[repr(C)]
pub struct ImFontGlyph {
    pub codepoint: ImWchar,
    pub advance_x: c_float,
    pub x0: c_float,
    pub y0: c_float,
    pub x1: c_float,
    pub y1: c_float,
    pub u0: c_float,
    pub v0: c_float,
    pub u1: c_float,
    pub v1: c_float,
}

#[repr(C)]
pub struct ImGuiIO {
    pub config_flags: ImGuiConfigFlags,
    pub backend_flags: ImGuiBackendFlags,
    pub display_size: ImVec2,
    pub delta_time: c_float,
    pub ini_saving_rate: c_float,
    pub ini_filename: *const c_char,
    pub log_filename: *const c_char,
    pub mouse_double_click_time: c_float,
    pub mouse_double_click_max_dist: c_float,
    pub mouse_drag_threshold: c_float,
    pub key_map: [c_int; ImGuiKey::COUNT as usize],
    pub key_repeat_delay: c_float,
    pub key_repeat_rate: c_float,
    pub user_data: *mut c_void,
    pub fonts: *mut ImFontAtlas,
    pub font_global_scale: c_float,
    pub font_allow_user_scaling: bool,
    pub font_default: *mut ImFont,
    pub display_framebuffer_scale: ImVec2,
    pub display_visible_min: ImVec2,
    pub display_visible_max: ImVec2,
    pub mouse_draw_cursor: bool,
    pub config_mac_osx_behaviors: bool,
    pub config_input_text_cursor_blink: bool,
    pub config_resize_windows_from_edges: bool,
    pub backend_platform_name: *const c_char,
    pub backend_renderer_name: *const c_char,
    pub get_clipboard_text_fn: Option<extern "C" fn(user_data: *mut c_void) -> *const c_char>,
    pub set_clipboard_text_fn: Option<extern "C" fn(user_data: *mut c_void, text: *const c_char)>,
    pub clipboard_user_data: *mut c_void,
    pub ime_set_input_screen_pos_fn: Option<extern "C" fn(x: c_int, y: c_int)>,
    pub ime_window_handle: *mut c_void,
    pub render_draw_lists_fn_unused: *mut c_void,
    pub mouse_pos: ImVec2,
    pub mouse_down: [bool; 5],
    pub mouse_wheel: c_float,
    pub mouse_wheel_h: c_float,
    pub key_ctrl: bool,
    pub key_shift: bool,
    pub key_alt: bool,
    pub key_super: bool,
    pub keys_down: [bool; 512],
    pub input_characters: [ImWchar; 16 + 1],
    pub nav_inputs: [c_float; ImGuiNavInput::COUNT as usize],
    pub want_capture_mouse: bool,
    pub want_capture_keyboard: bool,
    pub want_text_input: bool,
    pub want_set_mouse_pos: bool,
    pub want_save_ini_settings: bool,
    pub nav_active: bool,
    pub nav_visible: bool,
    pub framerate: c_float,
    pub metrics_render_vertices: c_int,
    pub metrics_render_indices: c_int,
    pub metrics_render_windows: c_int,
    pub metrics_active_windows: c_int,
    pub metrics_active_allocations: c_int,
    pub mouse_delta: ImVec2,
    pub mouse_pos_prev: ImVec2,
    pub mouse_clicked_pos: [ImVec2; 5],
    pub mouse_clicked_time: [c_double; 5],
    pub mouse_clicked: [bool; 5],
    pub mouse_double_clicked: [bool; 5],
    pub mouse_released: [bool; 5],
    pub mouse_down_owned: [bool; 5],
    pub mouse_down_duration: [c_float; 5],
    pub mouse_down_duration_prev: [c_float; 5],
    pub mouse_drag_max_distance_abs: [ImVec2; 5],
    pub mouse_drag_max_distance_sqr: [c_float; 5],
    pub keys_down_duration: [c_float; 512],
    pub keys_down_duration_prev: [c_float; 512],
    pub nav_inputs_down_duration: [c_float; ImGuiNavInput::COUNT as usize],
    pub nav_inputs_down_duration_prev: [c_float; ImGuiNavInput::COUNT as usize],
}

#[repr(C)]
pub struct ImGuiInputTextCallbackData {
    pub event_flag: ImGuiInputTextFlags,
    pub flags: ImGuiInputTextFlags,
    pub user_data: *mut c_void,
    pub event_char: ImWchar,
    pub event_key: ImGuiKey,
    pub buf: *mut c_char,
    pub buf_text_len: c_int,
    pub buf_size: c_int,
    pub buf_dirty: bool,
    pub cursor_pos: c_int,
    pub selection_start: c_int,
    pub selection_end: c_int,
}

#[repr(C)]
pub struct ImGuiListClipper {
    pub start_pos_y: c_float,
    pub items_height: c_float,
    pub items_count: c_int,
    pub step_no: c_int,
    pub display_start: c_int,
    pub display_end: c_int,
}

#[repr(C)]
pub struct ImGuiOnceUponAFrame {
    pub ref_frame: c_int,
}

#[repr(C)]
pub struct ImGuiPayload {
    pub data: *mut c_void,
    pub data_size: c_int,
    pub source_id: ImGuiID,
    pub source_parent_id: ImGuiID,
    pub data_frame_count: c_int,
    pub data_type: [c_char; 32 + 1],
    pub preview: bool,
    pub delivery: bool,
}

#[repr(C)]
pub struct ImGuiSizeCallbackData {
    pub user_data: *mut c_void,
    pub pos: ImVec2,
    pub current_size: ImVec2,
    pub desired_size: ImVec2,
}

#[repr(C)]
pub struct ImGuiStorage {
    pub data: ImVector<Pair>,
}

#[repr(C)]
pub struct ImGuiStyle {
    pub alpha: c_float,
    pub window_padding: ImVec2,
    pub window_rounding: c_float,
    pub window_border_size: c_float,
    pub window_min_size: ImVec2,
    pub window_title_align: ImVec2,
    pub child_rounding: c_float,
    pub child_border_size: c_float,
    pub popup_rounding: c_float,
    pub popup_border_size: c_float,
    pub frame_padding: ImVec2,
    pub frame_rounding: c_float,
    pub frame_border_size: c_float,
    pub item_spacing: ImVec2,
    pub item_inner_spacing: ImVec2,
    pub touch_extra_padding: ImVec2,
    pub indent_spacing: c_float,
    pub columns_min_spacing: c_float,
    pub scrollbar_size: c_float,
    pub scrollbar_rounding: c_float,
    pub grab_min_size: c_float,
    pub grab_rounding: c_float,
    pub button_text_align: ImVec2,
    pub display_window_padding: ImVec2,
    pub display_safe_area_padding: ImVec2,
    pub mouse_cursor_scale: c_float,
    pub anti_aliased_lines: bool,
    pub anti_aliased_fill: bool,
    pub curve_tessellation_tol: c_float,
    pub colors: [ImVec4; ImGuiCol::COUNT as usize],
}

#[repr(C)]
pub struct ImGuiTextBuffer {
    pub buf: ImVector<c_char>,
}

#[repr(C)]
pub struct ImGuiTextFilter {
    pub input_buf: [c_char; 256],
    pub filters: ImVector<TextRange>,
    pub count_grep: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImVec2 {
    pub x: c_float,
    pub y: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImVec4 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float,
}

#[repr(C)]
pub struct TextRange {
    pub b: *const c_char,
    pub e: *const c_char,
}

extern "C" {
    pub fn CustomRect_CustomRect();
    pub fn CustomRect_IsPacked(_self: *mut CustomRect) -> bool;
    pub fn CustomRect_destroy(_self: *mut CustomRect);
    pub fn GlyphRangesBuilder_AddChar(_self: *mut GlyphRangesBuilder, c: ImWchar);
    pub fn GlyphRangesBuilder_AddRanges(_self: *mut GlyphRangesBuilder, ranges: *const ImWchar);
    pub fn GlyphRangesBuilder_AddText(_self: *mut GlyphRangesBuilder, text: *const c_char, text_end: *const c_char);
    pub fn GlyphRangesBuilder_BuildRanges(_self: *mut GlyphRangesBuilder, out_ranges: *mut ImVector<ImWchar>);
    pub fn GlyphRangesBuilder_GetBit(_self: *mut GlyphRangesBuilder, n: c_int) -> bool;
    pub fn GlyphRangesBuilder_GlyphRangesBuilder();
    pub fn GlyphRangesBuilder_SetBit(_self: *mut GlyphRangesBuilder, n: c_int);
    pub fn GlyphRangesBuilder_destroy(_self: *mut GlyphRangesBuilder);
    pub fn ImColor_HSV(_self: *mut ImColor, h: c_float, s: c_float, v: c_float, a: c_float) -> ImColor;
    pub fn ImColor_ImColor();
    pub fn ImColor_ImColorInt(r: c_int, g: c_int, b: c_int, a: c_int);
    pub fn ImColor_ImColorU32(rgba: ImU32);
    pub fn ImColor_ImColorFloat(r: c_float, g: c_float, b: c_float, a: c_float);
    pub fn ImColor_ImColorVec4(col: ImVec4);
    pub fn ImColor_SetHSV(_self: *mut ImColor, h: c_float, s: c_float, v: c_float, a: c_float);
    pub fn ImColor_destroy(_self: *mut ImColor);
    pub fn ImDrawCmd_ImDrawCmd();
    pub fn ImDrawCmd_destroy(_self: *mut ImDrawCmd);
    pub fn ImDrawData_Clear(_self: *mut ImDrawData);
    pub fn ImDrawData_DeIndexAllBuffers(_self: *mut ImDrawData);
    pub fn ImDrawData_ImDrawData();
    pub fn ImDrawData_ScaleClipRects(_self: *mut ImDrawData, sc: ImVec2);
    pub fn ImDrawData_destroy(_self: *mut ImDrawData);
    pub fn ImDrawList_AddBezierCurve(
        _self: *mut ImDrawList,
        pos0: ImVec2,
        cp0: ImVec2,
        cp1: ImVec2,
        pos1: ImVec2,
        col: ImU32,
        thickness: c_float,
        num_segments: c_int,
    );
    pub fn ImDrawList_AddCallback(_self: *mut ImDrawList, callback: ImDrawCallback, callback_data: *mut c_void);
    pub fn ImDrawList_AddCircle(
        _self: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        col: ImU32,
        num_segments: c_int,
        thickness: c_float,
    );
    pub fn ImDrawList_AddCircleFilled(
        _self: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        col: ImU32,
        num_segments: c_int,
    );
    pub fn ImDrawList_AddConvexPolyFilled(_self: *mut ImDrawList, points: *const ImVec2, num_points: c_int, col: ImU32);
    pub fn ImDrawList_AddDrawCmd(_self: *mut ImDrawList);
    pub fn ImDrawList_AddImage(
        _self: *mut ImDrawList,
        user_texture_id: ImTextureID,
        a: ImVec2,
        b: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_AddImageQuad(
        _self: *mut ImDrawList,
        user_texture_id: ImTextureID,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        uv_c: ImVec2,
        uv_d: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_AddImageRounded(
        _self: *mut ImDrawList,
        user_texture_id: ImTextureID,
        a: ImVec2,
        b: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        col: ImU32,
        rounding: c_float,
        rounding_corners: c_int,
    );
    pub fn ImDrawList_AddLine(_self: *mut ImDrawList, a: ImVec2, b: ImVec2, col: ImU32, thickness: c_float);
    pub fn ImDrawList_AddPolyline(
        _self: *mut ImDrawList,
        points: *const ImVec2,
        num_points: c_int,
        col: ImU32,
        closed: bool,
        thickness: c_float,
    );
    pub fn ImDrawList_AddQuad(
        _self: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        col: ImU32,
        thickness: c_float,
    );
    pub fn ImDrawList_AddQuadFilled(_self: *mut ImDrawList, a: ImVec2, b: ImVec2, c: ImVec2, d: ImVec2, col: ImU32);
    pub fn ImDrawList_AddRect(
        _self: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col: ImU32,
        rounding: c_float,
        rounding_corners_flags: c_int,
        thickness: c_float,
    );
    pub fn ImDrawList_AddRectFilled(
        _self: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col: ImU32,
        rounding: c_float,
        rounding_corners_flags: c_int,
    );
    pub fn ImDrawList_AddRectFilledMultiColor(
        _self: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        col_upr_left: ImU32,
        col_upr_right: ImU32,
        col_bot_right: ImU32,
        col_bot_left: ImU32,
    );
    pub fn ImDrawList_AddText(
        _self: *mut ImDrawList,
        pos: ImVec2,
        col: ImU32,
        text_begin: *const c_char,
        text_end: *const c_char,
    );
    pub fn ImDrawList_AddTextFontPtr(
        _self: *mut ImDrawList,
        font: *const ImFont,
        font_size: c_float,
        pos: ImVec2,
        col: ImU32,
        text_begin: *const c_char,
        text_end: *const c_char,
        wrap_width: c_float,
        cpu_fine_clip_rect: *const ImVec4,
    );
    pub fn ImDrawList_AddTriangle(
        _self: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        col: ImU32,
        thickness: c_float,
    );
    pub fn ImDrawList_AddTriangleFilled(_self: *mut ImDrawList, a: ImVec2, b: ImVec2, c: ImVec2, col: ImU32);
    pub fn ImDrawList_ChannelsMerge(_self: *mut ImDrawList);
    pub fn ImDrawList_ChannelsSetCurrent(_self: *mut ImDrawList, channel_index: c_int);
    pub fn ImDrawList_ChannelsSplit(_self: *mut ImDrawList, channels_count: c_int);
    pub fn ImDrawList_Clear(_self: *mut ImDrawList);
    pub fn ImDrawList_ClearFreeMemory(_self: *mut ImDrawList);
    pub fn ImDrawList_CloneOutput(_self: *mut ImDrawList) -> *mut ImDrawList;
    pub fn ImDrawList_GetClipRectMax(_self: *mut ImDrawList) -> ImVec2;
    pub fn ImDrawList_GetClipRectMin(_self: *mut ImDrawList) -> ImVec2;
    pub fn ImDrawList_ImDrawList(shared_data: *const ImDrawListSharedData);
    pub fn ImDrawList_PathArcTo(
        _self: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        a_min: c_float,
        a_max: c_float,
        num_segments: c_int,
    );
    pub fn ImDrawList_PathArcToFast(
        _self: *mut ImDrawList,
        centre: ImVec2,
        radius: c_float,
        a_min_of_12: c_int,
        a_max_of_12: c_int,
    );
    pub fn ImDrawList_PathBezierCurveTo(
        _self: *mut ImDrawList,
        p1: ImVec2,
        p2: ImVec2,
        p3: ImVec2,
        num_segments: c_int,
    );
    pub fn ImDrawList_PathClear(_self: *mut ImDrawList);
    pub fn ImDrawList_PathFillConvex(_self: *mut ImDrawList, col: ImU32);
    pub fn ImDrawList_PathLineTo(_self: *mut ImDrawList, pos: ImVec2);
    pub fn ImDrawList_PathLineToMergeDuplicate(_self: *mut ImDrawList, pos: ImVec2);
    pub fn ImDrawList_PathRect(
        _self: *mut ImDrawList,
        rect_min: ImVec2,
        rect_max: ImVec2,
        rounding: c_float,
        rounding_corners_flags: c_int,
    );
    pub fn ImDrawList_PathStroke(_self: *mut ImDrawList, col: ImU32, closed: bool, thickness: c_float);
    pub fn ImDrawList_PopClipRect(_self: *mut ImDrawList);
    pub fn ImDrawList_PopTextureID(_self: *mut ImDrawList);
    pub fn ImDrawList_PrimQuadUV(
        _self: *mut ImDrawList,
        a: ImVec2,
        b: ImVec2,
        c: ImVec2,
        d: ImVec2,
        uv_a: ImVec2,
        uv_b: ImVec2,
        uv_c: ImVec2,
        uv_d: ImVec2,
        col: ImU32,
    );
    pub fn ImDrawList_PrimRect(_self: *mut ImDrawList, a: ImVec2, b: ImVec2, col: ImU32);
    pub fn ImDrawList_PrimRectUV(_self: *mut ImDrawList, a: ImVec2, b: ImVec2, uv_a: ImVec2, uv_b: ImVec2, col: ImU32);
    pub fn ImDrawList_PrimReserve(_self: *mut ImDrawList, idx_count: c_int, vtx_count: c_int);
    pub fn ImDrawList_PrimVtx(_self: *mut ImDrawList, pos: ImVec2, uv: ImVec2, col: ImU32);
    pub fn ImDrawList_PrimWriteIdx(_self: *mut ImDrawList, idx: ImDrawIdx);
    pub fn ImDrawList_PrimWriteVtx(_self: *mut ImDrawList, pos: ImVec2, uv: ImVec2, col: ImU32);
    pub fn ImDrawList_PushClipRect(
        _self: *mut ImDrawList,
        clip_rect_min: ImVec2,
        clip_rect_max: ImVec2,
        intersect_with_current_clip_rect: bool,
    );
    pub fn ImDrawList_PushClipRectFullScreen(_self: *mut ImDrawList);
    pub fn ImDrawList_PushTextureID(_self: *mut ImDrawList, texture_id: ImTextureID);
    pub fn ImDrawList_UpdateClipRect(_self: *mut ImDrawList);
    pub fn ImDrawList_UpdateTextureID(_self: *mut ImDrawList);
    pub fn ImDrawList_destroy(_self: *mut ImDrawList);
    pub fn ImFontAtlas_AddCustomRectFontGlyph(
        _self: *mut ImFontAtlas,
        font: *mut ImFont,
        id: ImWchar,
        width: c_int,
        height: c_int,
        advance_x: c_float,
        offset: ImVec2,
    ) -> c_int;
    pub fn ImFontAtlas_AddCustomRectRegular(_self: *mut ImFontAtlas, id: c_uint, width: c_int, height: c_int) -> c_int;
    pub fn ImFontAtlas_AddFont(_self: *mut ImFontAtlas, font_cfg: *const ImFontConfig) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontDefault(_self: *mut ImFontAtlas, font_cfg: *const ImFontConfig) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromFileTTF(
        _self: *mut ImFontAtlas,
        filename: *const c_char,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryCompressedBase85TTF(
        _self: *mut ImFontAtlas,
        compressed_font_data_base85: *const c_char,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryCompressedTTF(
        _self: *mut ImFontAtlas,
        compressed_font_data: *const c_void,
        compressed_font_size: c_int,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_AddFontFromMemoryTTF(
        _self: *mut ImFontAtlas,
        font_data: *mut c_void,
        font_size: c_int,
        size_pixels: c_float,
        font_cfg: *const ImFontConfig,
        glyph_ranges: *const ImWchar,
    ) -> *mut ImFont;
    pub fn ImFontAtlas_Build(_self: *mut ImFontAtlas) -> bool;
    pub fn ImFontAtlas_CalcCustomRectUV(
        _self: *mut ImFontAtlas,
        rect: *const CustomRect,
        out_uv_min: *mut ImVec2,
        out_uv_max: *mut ImVec2,
    );
    pub fn ImFontAtlas_Clear(_self: *mut ImFontAtlas);
    pub fn ImFontAtlas_ClearFonts(_self: *mut ImFontAtlas);
    pub fn ImFontAtlas_ClearInputData(_self: *mut ImFontAtlas);
    pub fn ImFontAtlas_ClearTexData(_self: *mut ImFontAtlas);
    pub fn ImFontAtlas_GetCustomRectByIndex(_self: *mut ImFontAtlas, index: c_int) -> *const CustomRect;
    pub fn ImFontAtlas_GetGlyphRangesChineseFull(_self: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesChineseSimplifiedCommon(_self: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesCyrillic(_self: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesDefault(_self: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesJapanese(_self: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesKorean(_self: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetGlyphRangesThai(_self: *mut ImFontAtlas) -> *const ImWchar;
    pub fn ImFontAtlas_GetMouseCursorTexData(
        _self: *mut ImFontAtlas,
        cursor: ImGuiMouseCursor,
        out_offset: *mut ImVec2,
        out_size: *mut ImVec2,
        out_uv_border: *mut [ImVec2; 2],
        out_uv_fill: *mut [ImVec2; 2],
    ) -> bool;
    pub fn ImFontAtlas_GetTexDataAsAlpha8(
        _self: *mut ImFontAtlas,
        out_pixels: *mut *mut c_uchar,
        out_width: *mut c_int,
        out_height: *mut c_int,
        out_bytes_per_pixel: *mut c_int,
    );
    pub fn ImFontAtlas_GetTexDataAsRGBA32(
        _self: *mut ImFontAtlas,
        out_pixels: *mut *mut c_uchar,
        out_width: *mut c_int,
        out_height: *mut c_int,
        out_bytes_per_pixel: *mut c_int,
    );
    pub fn ImFontAtlas_ImFontAtlas();
    pub fn ImFontAtlas_IsBuilt(_self: *mut ImFontAtlas) -> bool;
    pub fn ImFontAtlas_SetTexID(_self: *mut ImFontAtlas, id: ImTextureID);
    pub fn ImFontAtlas_destroy(_self: *mut ImFontAtlas);
    pub fn ImFontConfig_ImFontConfig();
    pub fn ImFontConfig_destroy(_self: *mut ImFontConfig);
    pub fn ImFont_AddGlyph(
        _self: *mut ImFont,
        c: ImWchar,
        x0: c_float,
        y0: c_float,
        x1: c_float,
        y1: c_float,
        u0: c_float,
        v0: c_float,
        u1: c_float,
        v1: c_float,
        advance_x: c_float,
    );
    pub fn ImFont_AddRemapChar(_self: *mut ImFont, dst: ImWchar, src: ImWchar, overwrite_dst: bool);
    pub fn ImFont_BuildLookupTable(_self: *mut ImFont);
    pub fn ImFont_CalcTextSizeA(
        _self: *mut ImFont,
        size: c_float,
        max_width: c_float,
        wrap_width: c_float,
        text_begin: *const c_char,
        text_end: *const c_char,
        remaining: *mut *const c_char,
    ) -> ImVec2;
    pub fn ImFont_CalcWordWrapPositionA(
        _self: *mut ImFont,
        scale: c_float,
        text: *const c_char,
        text_end: *const c_char,
        wrap_width: c_float,
    ) -> *const c_char;
    pub fn ImFont_ClearOutputData(_self: *mut ImFont);
    pub fn ImFont_FindGlyph(_self: *mut ImFont, c: ImWchar) -> *const ImFontGlyph;
    pub fn ImFont_FindGlyphNoFallback(_self: *mut ImFont, c: ImWchar) -> *const ImFontGlyph;
    pub fn ImFont_GetCharAdvance(_self: *mut ImFont, c: ImWchar) -> c_float;
    pub fn ImFont_GetDebugName(_self: *mut ImFont) -> *const c_char;
    pub fn ImFont_GrowIndex(_self: *mut ImFont, new_size: c_int);
    pub fn ImFont_ImFont();
    pub fn ImFont_IsLoaded(_self: *mut ImFont) -> bool;
    pub fn ImFont_RenderChar(
        _self: *mut ImFont,
        draw_list: *mut ImDrawList,
        size: c_float,
        pos: ImVec2,
        col: ImU32,
        c: ImWchar,
    );
    pub fn ImFont_RenderText(
        _self: *mut ImFont,
        draw_list: *mut ImDrawList,
        size: c_float,
        pos: ImVec2,
        col: ImU32,
        clip_rect: ImVec4,
        text_begin: *const c_char,
        text_end: *const c_char,
        wrap_width: c_float,
        cpu_fine_clip: bool,
    );
    pub fn ImFont_SetFallbackChar(_self: *mut ImFont, c: ImWchar);
    pub fn ImFont_destroy(_self: *mut ImFont);
    pub fn ImGuiIO_AddInputCharacter(_self: *mut ImGuiIO, c: ImWchar);
    pub fn ImGuiIO_AddInputCharactersUTF8(_self: *mut ImGuiIO, utf8_chars: *const c_char);
    pub fn ImGuiIO_ClearInputCharacters(_self: *mut ImGuiIO);
    pub fn ImGuiIO_ImGuiIO();
    pub fn ImGuiIO_destroy(_self: *mut ImGuiIO);
    pub fn ImGuiInputTextCallbackData_DeleteChars(
        _self: *mut ImGuiInputTextCallbackData,
        pos: c_int,
        bytes_count: c_int,
    );
    pub fn ImGuiInputTextCallbackData_HasSelection(_self: *mut ImGuiInputTextCallbackData) -> bool;
    pub fn ImGuiInputTextCallbackData_ImGuiInputTextCallbackData();
    pub fn ImGuiInputTextCallbackData_InsertChars(
        _self: *mut ImGuiInputTextCallbackData,
        pos: c_int,
        text: *const c_char,
        text_end: *const c_char,
    );
    pub fn ImGuiInputTextCallbackData_destroy(_self: *mut ImGuiInputTextCallbackData);
    pub fn ImGuiListClipper_Begin(_self: *mut ImGuiListClipper, items_count: c_int, items_height: c_float);
    pub fn ImGuiListClipper_End(_self: *mut ImGuiListClipper);
    pub fn ImGuiListClipper_ImGuiListClipper(items_count: c_int, items_height: c_float);
    pub fn ImGuiListClipper_Step(_self: *mut ImGuiListClipper) -> bool;
    pub fn ImGuiListClipper_destroy(_self: *mut ImGuiListClipper);
    pub fn ImGuiOnceUponAFrame_ImGuiOnceUponAFrame();
    pub fn ImGuiOnceUponAFrame_destroy(_self: *mut ImGuiOnceUponAFrame);
    pub fn ImGuiPayload_Clear(_self: *mut ImGuiPayload);
    pub fn ImGuiPayload_ImGuiPayload();
    pub fn ImGuiPayload_IsDataType(_self: *mut ImGuiPayload, _type: *const c_char) -> bool;
    pub fn ImGuiPayload_IsDelivery(_self: *mut ImGuiPayload) -> bool;
    pub fn ImGuiPayload_IsPreview(_self: *mut ImGuiPayload) -> bool;
    pub fn ImGuiPayload_destroy(_self: *mut ImGuiPayload);
    pub fn ImGuiStorage_BuildSortByKey(_self: *mut ImGuiStorage);
    pub fn ImGuiStorage_Clear(_self: *mut ImGuiStorage);
    pub fn ImGuiStorage_GetBool(_self: *mut ImGuiStorage, key: ImGuiID, default_val: bool) -> bool;
    pub fn ImGuiStorage_GetBoolRef(_self: *mut ImGuiStorage, key: ImGuiID, default_val: bool) -> *mut bool;
    pub fn ImGuiStorage_GetFloat(_self: *mut ImGuiStorage, key: ImGuiID, default_val: c_float) -> c_float;
    pub fn ImGuiStorage_GetFloatRef(_self: *mut ImGuiStorage, key: ImGuiID, default_val: c_float) -> *mut c_float;
    pub fn ImGuiStorage_GetInt(_self: *mut ImGuiStorage, key: ImGuiID, default_val: c_int) -> c_int;
    pub fn ImGuiStorage_GetIntRef(_self: *mut ImGuiStorage, key: ImGuiID, default_val: c_int) -> *mut c_int;
    pub fn ImGuiStorage_GetVoidPtr(_self: *mut ImGuiStorage, key: ImGuiID) -> *mut c_void;
    pub fn ImGuiStorage_GetVoidPtrRef(
        _self: *mut ImGuiStorage,
        key: ImGuiID,
        default_val: *mut c_void,
    ) -> *mut *mut c_void;
    pub fn ImGuiStorage_SetAllInt(_self: *mut ImGuiStorage, val: c_int);
    pub fn ImGuiStorage_SetBool(_self: *mut ImGuiStorage, key: ImGuiID, val: bool);
    pub fn ImGuiStorage_SetFloat(_self: *mut ImGuiStorage, key: ImGuiID, val: c_float);
    pub fn ImGuiStorage_SetInt(_self: *mut ImGuiStorage, key: ImGuiID, val: c_int);
    pub fn ImGuiStorage_SetVoidPtr(_self: *mut ImGuiStorage, key: ImGuiID, val: *mut c_void);
    pub fn ImGuiStyle_ImGuiStyle();
    pub fn ImGuiStyle_ScaleAllSizes(_self: *mut ImGuiStyle, scale_factor: c_float);
    pub fn ImGuiStyle_destroy(_self: *mut ImGuiStyle);
    pub fn ImGuiTextBuffer_ImGuiTextBuffer();
    pub fn ImGuiTextBuffer_appendf(_self: *mut ImGuiTextBuffer, fmt: *const c_char, ...);
    pub fn ImGuiTextBuffer_begin(_self: *mut ImGuiTextBuffer) -> *const c_char;
    pub fn ImGuiTextBuffer_c_str(_self: *mut ImGuiTextBuffer) -> *const c_char;
    pub fn ImGuiTextBuffer_clear(_self: *mut ImGuiTextBuffer);
    pub fn ImGuiTextBuffer_destroy(_self: *mut ImGuiTextBuffer);
    pub fn ImGuiTextBuffer_empty(_self: *mut ImGuiTextBuffer) -> bool;
    pub fn ImGuiTextBuffer_end(_self: *mut ImGuiTextBuffer) -> *const c_char;
    pub fn ImGuiTextBuffer_reserve(_self: *mut ImGuiTextBuffer, capacity: c_int);
    pub fn ImGuiTextBuffer_size(_self: *mut ImGuiTextBuffer) -> c_int;
    pub fn ImGuiTextFilter_Build(_self: *mut ImGuiTextFilter);
    pub fn ImGuiTextFilter_Clear(_self: *mut ImGuiTextFilter);
    pub fn ImGuiTextFilter_Draw(_self: *mut ImGuiTextFilter, label: *const c_char, width: c_float) -> bool;
    pub fn ImGuiTextFilter_ImGuiTextFilter(default_filter: *const c_char);
    pub fn ImGuiTextFilter_IsActive(_self: *mut ImGuiTextFilter) -> bool;
    pub fn ImGuiTextFilter_PassFilter(
        _self: *mut ImGuiTextFilter,
        text: *const c_char,
        text_end: *const c_char,
    ) -> bool;
    pub fn ImGuiTextFilter_destroy(_self: *mut ImGuiTextFilter);
    pub fn ImVec2_ImVec2();
    pub fn ImVec2_ImVec2Float(_x: c_float, _y: c_float);
    pub fn ImVec2_destroy(_self: *mut ImVec2);
    pub fn ImVec4_ImVec4();
    pub fn ImVec4_ImVec4Float(_x: c_float, _y: c_float, _z: c_float, _w: c_float);
    pub fn ImVec4_destroy(_self: *mut ImVec4);
    pub fn Pair_PairInt(_key: ImGuiID, _val_i: c_int);
    pub fn Pair_PairFloat(_key: ImGuiID, _val_f: c_float);
    pub fn Pair_PairPtr(_key: ImGuiID, _val_p: *mut c_void);
    pub fn Pair_destroy(_self: *mut Pair);
    pub fn TextRange_TextRange();
    pub fn TextRange_TextRangeStr(_b: *const c_char, _e: *const c_char);
    pub fn TextRange_begin(_self: *mut TextRange) -> *const c_char;
    pub fn TextRange_destroy(_self: *mut TextRange);
    pub fn TextRange_empty(_self: *mut TextRange) -> bool;
    pub fn TextRange_end(_self: *mut TextRange) -> *const c_char;
    pub fn TextRange_split(_self: *mut TextRange, separator: c_char, out: *mut ImVector<TextRange>);
    pub fn igAcceptDragDropPayload(_type: *const c_char, flags: ImGuiDragDropFlags) -> *const ImGuiPayload;
    pub fn igAlignTextToFramePadding();
    pub fn igArrowButton(str_id: *const c_char, dir: ImGuiDir) -> bool;
    pub fn igBegin(name: *const c_char, p_open: *mut bool, flags: ImGuiWindowFlags) -> bool;
    pub fn igBeginChild(str_id: *const c_char, size: ImVec2, border: bool, flags: ImGuiWindowFlags) -> bool;
    pub fn igBeginChildID(id: ImGuiID, size: ImVec2, border: bool, flags: ImGuiWindowFlags) -> bool;
    pub fn igBeginChildFrame(id: ImGuiID, size: ImVec2, flags: ImGuiWindowFlags) -> bool;
    pub fn igBeginCombo(label: *const c_char, preview_value: *const c_char, flags: ImGuiComboFlags) -> bool;
    pub fn igBeginDragDropSource(flags: ImGuiDragDropFlags) -> bool;
    pub fn igBeginDragDropTarget() -> bool;
    pub fn igBeginGroup();
    pub fn igBeginMainMenuBar() -> bool;
    pub fn igBeginMenu(label: *const c_char, enabled: bool) -> bool;
    pub fn igBeginMenuBar() -> bool;
    pub fn igBeginPopup(str_id: *const c_char, flags: ImGuiWindowFlags) -> bool;
    pub fn igBeginPopupContextItem(str_id: *const c_char, mouse_button: c_int) -> bool;
    pub fn igBeginPopupContextVoid(str_id: *const c_char, mouse_button: c_int) -> bool;
    pub fn igBeginPopupContextWindow(str_id: *const c_char, mouse_button: c_int, also_over_items: bool) -> bool;
    pub fn igBeginPopupModal(name: *const c_char, p_open: *mut bool, flags: ImGuiWindowFlags) -> bool;
    pub fn igBeginTooltip();
    pub fn igBullet();
    pub fn igBulletText(fmt: *const c_char, ...);
    pub fn igButton(label: *const c_char, size: ImVec2) -> bool;
    pub fn igCalcItemWidth() -> c_float;
    pub fn igCalcListClipping(
        items_count: c_int,
        items_height: c_float,
        out_items_display_start: *mut c_int,
        out_items_display_end: *mut c_int,
    );
    pub fn igCalcTextSize(
        text: *const c_char,
        text_end: *const c_char,
        hide_text_after_double_hash: bool,
        wrap_width: c_float,
    ) -> ImVec2;
    pub fn igCaptureKeyboardFromApp(capture: bool);
    pub fn igCaptureMouseFromApp(capture: bool);
    pub fn igCheckbox(label: *const c_char, v: *mut bool) -> bool;
    pub fn igCheckboxFlags(label: *const c_char, flags: *mut c_uint, flags_value: c_uint) -> bool;
    pub fn igCloseCurrentPopup();
    pub fn igCollapsingHeader(label: *const c_char, flags: ImGuiTreeNodeFlags) -> bool;
    pub fn igCollapsingHeaderBoolPtr(label: *const c_char, p_open: *mut bool, flags: ImGuiTreeNodeFlags) -> bool;
    pub fn igColorButton(desc_id: *const c_char, col: ImVec4, flags: ImGuiColorEditFlags, size: ImVec2) -> bool;
    pub fn igColorConvertFloat4ToU32(_in: ImVec4) -> ImU32;
    pub fn igColorConvertHSVtoRGB(
        h: c_float,
        s: c_float,
        v: c_float,
        out_r: *mut c_float,
        out_g: *mut c_float,
        out_b: *mut c_float,
    );
    pub fn igColorConvertRGBtoHSV(
        r: c_float,
        g: c_float,
        b: c_float,
        out_h: *mut c_float,
        out_s: *mut c_float,
        out_v: *mut c_float,
    );
    pub fn igColorConvertU32ToFloat4(_in: ImU32) -> ImVec4;
    pub fn igColorEdit3(label: *const c_char, col: *mut [c_float; 3], flags: ImGuiColorEditFlags) -> bool;
    pub fn igColorEdit4(label: *const c_char, col: *mut [c_float; 4], flags: ImGuiColorEditFlags) -> bool;
    pub fn igColorPicker3(label: *const c_char, col: *mut [c_float; 3], flags: ImGuiColorEditFlags) -> bool;
    pub fn igColorPicker4(
        label: *const c_char,
        col: *mut [c_float; 4],
        flags: ImGuiColorEditFlags,
        ref_col: *const c_float,
    ) -> bool;
    pub fn igColumns(count: c_int, id: *const c_char, border: bool);
    pub fn igCombo(
        label: *const c_char,
        current_item: *mut c_int,
        items: *const *const c_char,
        items_count: c_int,
        popup_max_height_in_items: c_int,
    ) -> bool;
    pub fn igComboStr(
        label: *const c_char,
        current_item: *mut c_int,
        items_separated_by_zeros: *const c_char,
        popup_max_height_in_items: c_int,
    ) -> bool;
    pub fn igComboFnPtr(
        label: *const c_char,
        current_item: *mut c_int,
        items_getter: extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char) -> bool,
        data: *mut c_void,
        items_count: c_int,
        popup_max_height_in_items: c_int,
    ) -> bool;
    pub fn igCreateContext(shared_font_atlas: *mut ImFontAtlas) -> *mut ImGuiContext;
    pub fn igDebugCheckVersionAndDataLayout(
        version_str: *const c_char,
        sz_io: size_t,
        sz_style: size_t,
        sz_vec2: size_t,
        sz_vec4: size_t,
        sz_drawvert: size_t,
    ) -> bool;
    pub fn igDestroyContext(ctx: *mut ImGuiContext);
    pub fn igDragFloat(
        label: *const c_char,
        v: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragFloat2(
        label: *const c_char,
        v: *mut [c_float; 2],
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragFloat3(
        label: *const c_char,
        v: *mut [c_float; 3],
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragFloat4(
        label: *const c_char,
        v: *mut [c_float; 4],
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragFloatRange2(
        label: *const c_char,
        v_current_min: *mut c_float,
        v_current_max: *mut c_float,
        v_speed: c_float,
        v_min: c_float,
        v_max: c_float,
        format: *const c_char,
        format_max: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragInt(
        label: *const c_char,
        v: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igDragInt2(
        label: *const c_char,
        v: *mut [c_int; 2],
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igDragInt3(
        label: *const c_char,
        v: *mut [c_int; 3],
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igDragInt4(
        label: *const c_char,
        v: *mut [c_int; 4],
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igDragIntRange2(
        label: *const c_char,
        v_current_min: *mut c_int,
        v_current_max: *mut c_int,
        v_speed: c_float,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
        format_max: *const c_char,
    ) -> bool;
    pub fn igDragScalar(
        label: *const c_char,
        data_type: ImGuiDataType,
        v: *mut c_void,
        v_speed: c_float,
        v_min: *const c_void,
        v_max: *const c_void,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDragScalarN(
        label: *const c_char,
        data_type: ImGuiDataType,
        v: *mut c_void,
        components: c_int,
        v_speed: c_float,
        v_min: *const c_void,
        v_max: *const c_void,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igDummy(size: ImVec2);
    pub fn igEnd();
    pub fn igEndChild();
    pub fn igEndChildFrame();
    pub fn igEndCombo();
    pub fn igEndDragDropSource();
    pub fn igEndDragDropTarget();
    pub fn igEndFrame();
    pub fn igEndGroup();
    pub fn igEndMainMenuBar();
    pub fn igEndMenu();
    pub fn igEndMenuBar();
    pub fn igEndPopup();
    pub fn igEndTooltip();
    pub fn igGetClipboardText() -> *const c_char;
    pub fn igGetColorU32(idx: ImGuiCol, alpha_mul: c_float) -> ImU32;
    pub fn igGetColorU32Vec4(col: ImVec4) -> ImU32;
    pub fn igGetColorU32U32(col: ImU32) -> ImU32;
    pub fn igGetColumnIndex() -> c_int;
    pub fn igGetColumnOffset(column_index: c_int) -> c_float;
    pub fn igGetColumnWidth(column_index: c_int) -> c_float;
    pub fn igGetColumnsCount() -> c_int;
    pub fn igGetContentRegionAvail() -> ImVec2;
    pub fn igGetContentRegionAvailWidth() -> c_float;
    pub fn igGetContentRegionMax() -> ImVec2;
    pub fn igGetCurrentContext() -> *mut ImGuiContext;
    pub fn igGetCursorPos() -> ImVec2;
    pub fn igGetCursorPosX() -> c_float;
    pub fn igGetCursorPosY() -> c_float;
    pub fn igGetCursorScreenPos() -> ImVec2;
    pub fn igGetCursorStartPos() -> ImVec2;
    pub fn igGetDragDropPayload() -> *const ImGuiPayload;
    pub fn igGetDrawData() -> *mut ImDrawData;
    pub fn igGetDrawListSharedData() -> *mut ImDrawListSharedData;
    pub fn igGetFont() -> *mut ImFont;
    pub fn igGetFontSize() -> c_float;
    pub fn igGetFontTexUvWhitePixel() -> ImVec2;
    pub fn igGetFrameCount() -> c_int;
    pub fn igGetFrameHeight() -> c_float;
    pub fn igGetFrameHeightWithSpacing() -> c_float;
    pub fn igGetIDStr(str_id: *const c_char) -> ImGuiID;
    pub fn igGetIDRange(str_id_begin: *const c_char, str_id_end: *const c_char) -> ImGuiID;
    pub fn igGetIDPtr(ptr_id: *const c_void) -> ImGuiID;
    pub fn igGetIO() -> *mut ImGuiIO;
    pub fn igGetItemRectMax() -> ImVec2;
    pub fn igGetItemRectMin() -> ImVec2;
    pub fn igGetItemRectSize() -> ImVec2;
    pub fn igGetKeyIndex(imgui_key: ImGuiKey) -> c_int;
    pub fn igGetKeyPressedAmount(key_index: c_int, repeat_delay: c_float, rate: c_float) -> c_int;
    pub fn igGetMouseCursor() -> ImGuiMouseCursor;
    pub fn igGetMouseDragDelta(button: c_int, lock_threshold: c_float) -> ImVec2;
    pub fn igGetMousePos() -> ImVec2;
    pub fn igGetMousePosOnOpeningCurrentPopup() -> ImVec2;
    pub fn igGetOverlayDrawList() -> *mut ImDrawList;
    pub fn igGetScrollMaxX() -> c_float;
    pub fn igGetScrollMaxY() -> c_float;
    pub fn igGetScrollX() -> c_float;
    pub fn igGetScrollY() -> c_float;
    pub fn igGetStateStorage() -> *mut ImGuiStorage;
    pub fn igGetStyle() -> *mut ImGuiStyle;
    pub fn igGetStyleColorName(idx: ImGuiCol) -> *const c_char;
    pub fn igGetStyleColorVec4(idx: ImGuiCol) -> *const ImVec4;
    pub fn igGetTextLineHeight() -> c_float;
    pub fn igGetTextLineHeightWithSpacing() -> c_float;
    pub fn igGetTime() -> c_double;
    pub fn igGetTreeNodeToLabelSpacing() -> c_float;
    pub fn igGetVersion() -> *const c_char;
    pub fn igGetWindowContentRegionMax() -> ImVec2;
    pub fn igGetWindowContentRegionMin() -> ImVec2;
    pub fn igGetWindowContentRegionWidth() -> c_float;
    pub fn igGetWindowDrawList() -> *mut ImDrawList;
    pub fn igGetWindowHeight() -> c_float;
    pub fn igGetWindowPos() -> ImVec2;
    pub fn igGetWindowSize() -> ImVec2;
    pub fn igGetWindowWidth() -> c_float;
    pub fn igImage(
        user_texture_id: ImTextureID,
        size: ImVec2,
        uv0: ImVec2,
        uv1: ImVec2,
        tint_col: ImVec4,
        border_col: ImVec4,
    );
    pub fn igImageButton(
        user_texture_id: ImTextureID,
        size: ImVec2,
        uv0: ImVec2,
        uv1: ImVec2,
        frame_padding: c_int,
        bg_col: ImVec4,
        tint_col: ImVec4,
    ) -> bool;
    pub fn igIndent(indent_w: c_float);
    pub fn igInputDouble(
        label: *const c_char,
        v: *mut c_double,
        step: c_double,
        step_fast: c_double,
        format: *const c_char,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat(
        label: *const c_char,
        v: *mut c_float,
        step: c_float,
        step_fast: c_float,
        format: *const c_char,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat2(
        label: *const c_char,
        v: *mut [c_float; 2],
        format: *const c_char,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat3(
        label: *const c_char,
        v: *mut [c_float; 3],
        format: *const c_char,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputFloat4(
        label: *const c_char,
        v: *mut [c_float; 4],
        format: *const c_char,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt(
        label: *const c_char,
        v: *mut c_int,
        step: c_int,
        step_fast: c_int,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputInt2(label: *const c_char, v: *mut [c_int; 2], extra_flags: ImGuiInputTextFlags) -> bool;
    pub fn igInputInt3(label: *const c_char, v: *mut [c_int; 3], extra_flags: ImGuiInputTextFlags) -> bool;
    pub fn igInputInt4(label: *const c_char, v: *mut [c_int; 4], extra_flags: ImGuiInputTextFlags) -> bool;
    pub fn igInputScalar(
        label: *const c_char,
        data_type: ImGuiDataType,
        v: *mut c_void,
        step: *const c_void,
        step_fast: *const c_void,
        format: *const c_char,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputScalarN(
        label: *const c_char,
        data_type: ImGuiDataType,
        v: *mut c_void,
        components: c_int,
        step: *const c_void,
        step_fast: *const c_void,
        format: *const c_char,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool;
    pub fn igInputText(
        label: *const c_char,
        buf: *mut c_char,
        buf_size: size_t,
        flags: ImGuiInputTextFlags,
        callback: ImGuiInputTextCallback,
        user_data: *mut c_void,
    ) -> bool;
    pub fn igInputTextMultiline(
        label: *const c_char,
        buf: *mut c_char,
        buf_size: size_t,
        size: ImVec2,
        flags: ImGuiInputTextFlags,
        callback: ImGuiInputTextCallback,
        user_data: *mut c_void,
    ) -> bool;
    pub fn igInvisibleButton(str_id: *const c_char, size: ImVec2) -> bool;
    pub fn igIsAnyItemActive() -> bool;
    pub fn igIsAnyItemFocused() -> bool;
    pub fn igIsAnyItemHovered() -> bool;
    pub fn igIsAnyMouseDown() -> bool;
    pub fn igIsItemActive() -> bool;
    pub fn igIsItemClicked(mouse_button: c_int) -> bool;
    pub fn igIsItemDeactivated() -> bool;
    pub fn igIsItemDeactivatedAfterEdit() -> bool;
    pub fn igIsItemEdited() -> bool;
    pub fn igIsItemFocused() -> bool;
    pub fn igIsItemHovered(flags: ImGuiHoveredFlags) -> bool;
    pub fn igIsItemVisible() -> bool;
    pub fn igIsKeyDown(user_key_index: c_int) -> bool;
    pub fn igIsKeyPressed(user_key_index: c_int, repeat: bool) -> bool;
    pub fn igIsKeyReleased(user_key_index: c_int) -> bool;
    pub fn igIsMouseClicked(button: c_int, repeat: bool) -> bool;
    pub fn igIsMouseDoubleClicked(button: c_int) -> bool;
    pub fn igIsMouseDown(button: c_int) -> bool;
    pub fn igIsMouseDragging(button: c_int, lock_threshold: c_float) -> bool;
    pub fn igIsMouseHoveringRect(r_min: ImVec2, r_max: ImVec2, clip: bool) -> bool;
    pub fn igIsMousePosValid(mouse_pos: *const ImVec2) -> bool;
    pub fn igIsMouseReleased(button: c_int) -> bool;
    pub fn igIsPopupOpen(str_id: *const c_char) -> bool;
    pub fn igIsRectVisible(size: ImVec2) -> bool;
    pub fn igIsRectVisibleVec2(rect_min: ImVec2, rect_max: ImVec2) -> bool;
    pub fn igIsWindowAppearing() -> bool;
    pub fn igIsWindowCollapsed() -> bool;
    pub fn igIsWindowFocused(flags: ImGuiFocusedFlags) -> bool;
    pub fn igIsWindowHovered(flags: ImGuiHoveredFlags) -> bool;
    pub fn igLabelText(label: *const c_char, fmt: *const c_char, ...);
    pub fn igListBoxStr_arr(
        label: *const c_char,
        current_item: *mut c_int,
        items: *const *const c_char,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBoxFnPtr(
        label: *const c_char,
        current_item: *mut c_int,
        items_getter: extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char) -> bool,
        data: *mut c_void,
        items_count: c_int,
        height_in_items: c_int,
    ) -> bool;
    pub fn igListBoxFooter();
    pub fn igListBoxHeaderVec2(label: *const c_char, size: ImVec2) -> bool;
    pub fn igListBoxHeaderInt(label: *const c_char, items_count: c_int, height_in_items: c_int) -> bool;
    pub fn igLoadIniSettingsFromDisk(ini_filename: *const c_char);
    pub fn igLoadIniSettingsFromMemory(ini_data: *const c_char, ini_size: size_t);
    pub fn igLogButtons();
    pub fn igLogFinish();
    pub fn igLogText(fmt: *const c_char, ...);
    pub fn igLogToClipboard(max_depth: c_int);
    pub fn igLogToFile(max_depth: c_int, filename: *const c_char);
    pub fn igLogToTTY(max_depth: c_int);
    pub fn igMemAlloc(size: size_t) -> *mut c_void;
    pub fn igMemFree(ptr: *mut c_void);
    pub fn igMenuItemBool(label: *const c_char, shortcut: *const c_char, selected: bool, enabled: bool) -> bool;
    pub fn igMenuItemBoolPtr(
        label: *const c_char,
        shortcut: *const c_char,
        p_selected: *mut bool,
        enabled: bool,
    ) -> bool;
    pub fn igNewFrame();
    pub fn igNewLine();
    pub fn igNextColumn();
    pub fn igOpenPopup(str_id: *const c_char);
    pub fn igOpenPopupOnItemClick(str_id: *const c_char, mouse_button: c_int) -> bool;
    pub fn igPlotHistogramFloatPtr(
        label: *const c_char,
        values: *const c_float,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
        stride: c_int,
    );
    pub fn igPlotHistogramFnPtr(
        label: *const c_char,
        values_getter: extern "C" fn(data: *mut c_void, idx: c_int) -> c_float,
        data: *mut c_void,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
    );
    pub fn igPlotLines(
        label: *const c_char,
        values: *const c_float,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
        stride: c_int,
    );
    pub fn igPlotLinesFnPtr(
        label: *const c_char,
        values_getter: extern "C" fn(data: *mut c_void, idx: c_int) -> c_float,
        data: *mut c_void,
        values_count: c_int,
        values_offset: c_int,
        overlay_text: *const c_char,
        scale_min: c_float,
        scale_max: c_float,
        graph_size: ImVec2,
    );
    pub fn igPopAllowKeyboardFocus();
    pub fn igPopButtonRepeat();
    pub fn igPopClipRect();
    pub fn igPopFont();
    pub fn igPopID();
    pub fn igPopItemWidth();
    pub fn igPopStyleColor(count: c_int);
    pub fn igPopStyleVar(count: c_int);
    pub fn igPopTextWrapPos();
    pub fn igProgressBar(fraction: c_float, size_arg: ImVec2, overlay: *const c_char);
    pub fn igPushAllowKeyboardFocus(allow_keyboard_focus: bool);
    pub fn igPushButtonRepeat(repeat: bool);
    pub fn igPushClipRect(clip_rect_min: ImVec2, clip_rect_max: ImVec2, intersect_with_current_clip_rect: bool);
    pub fn igPushFont(font: *mut ImFont);
    pub fn igPushIDStr(str_id: *const c_char);
    pub fn igPushIDRange(str_id_begin: *const c_char, str_id_end: *const c_char);
    pub fn igPushIDPtr(ptr_id: *const c_void);
    pub fn igPushIDInt(int_id: c_int);
    pub fn igPushItemWidth(item_width: c_float);
    pub fn igPushStyleColorU32(idx: ImGuiCol, col: ImU32);
    pub fn igPushStyleColor(idx: ImGuiCol, col: ImVec4);
    pub fn igPushStyleVarFloat(idx: ImGuiStyleVar, val: c_float);
    pub fn igPushStyleVarVec2(idx: ImGuiStyleVar, val: ImVec2);
    pub fn igPushTextWrapPos(wrap_pos_x: c_float);
    pub fn igRadioButtonBool(label: *const c_char, active: bool) -> bool;
    pub fn igRadioButtonIntPtr(label: *const c_char, v: *mut c_int, v_button: c_int) -> bool;
    pub fn igRender();
    pub fn igResetMouseDragDelta(button: c_int);
    pub fn igSameLine(pos_x: c_float, spacing_w: c_float);
    pub fn igSaveIniSettingsToDisk(ini_filename: *const c_char);
    pub fn igSaveIniSettingsToMemory(out_ini_size: *mut size_t) -> *const c_char;
    pub fn igSelectable(label: *const c_char, selected: bool, flags: ImGuiSelectableFlags, size: ImVec2) -> bool;
    pub fn igSelectableBoolPtr(
        label: *const c_char,
        p_selected: *mut bool,
        flags: ImGuiSelectableFlags,
        size: ImVec2,
    ) -> bool;
    pub fn igSeparator();
    pub fn igSetAllocatorFunctions(
        alloc_func: Option<extern "C" fn(sz: usize, user_data: *mut c_void) -> *mut c_void>,
        free_func: Option<extern "C" fn(ptr: *mut c_void, user_data: *mut c_void)>,
        user_data: *mut c_void,
    );
    pub fn igSetClipboardText(text: *const c_char);
    pub fn igSetColorEditOptions(flags: ImGuiColorEditFlags);
    pub fn igSetColumnOffset(column_index: c_int, offset_x: c_float);
    pub fn igSetColumnWidth(column_index: c_int, width: c_float);
    pub fn igSetCurrentContext(ctx: *mut ImGuiContext);
    pub fn igSetCursorPos(local_pos: ImVec2);
    pub fn igSetCursorPosX(x: c_float);
    pub fn igSetCursorPosY(y: c_float);
    pub fn igSetCursorScreenPos(screen_pos: ImVec2);
    pub fn igSetDragDropPayload(_type: *const c_char, data: *const c_void, size: size_t, cond: ImGuiCond) -> bool;
    pub fn igSetItemAllowOverlap();
    pub fn igSetItemDefaultFocus();
    pub fn igSetKeyboardFocusHere(offset: c_int);
    pub fn igSetMouseCursor(_type: ImGuiMouseCursor);
    pub fn igSetNextTreeNodeOpen(is_open: bool, cond: ImGuiCond);
    pub fn igSetNextWindowBgAlpha(alpha: c_float);
    pub fn igSetNextWindowCollapsed(collapsed: bool, cond: ImGuiCond);
    pub fn igSetNextWindowContentSize(size: ImVec2);
    pub fn igSetNextWindowFocus();
    pub fn igSetNextWindowPos(pos: ImVec2, cond: ImGuiCond, pivot: ImVec2);
    pub fn igSetNextWindowSize(size: ImVec2, cond: ImGuiCond);
    pub fn igSetNextWindowSizeConstraints(
        size_min: ImVec2,
        size_max: ImVec2,
        custom_callback: ImGuiSizeCallback,
        custom_callback_data: *mut c_void,
    );
    pub fn igSetScrollFromPosY(pos_y: c_float, center_y_ratio: c_float);
    pub fn igSetScrollHereY(center_y_ratio: c_float);
    pub fn igSetScrollX(scroll_x: c_float);
    pub fn igSetScrollY(scroll_y: c_float);
    pub fn igSetStateStorage(storage: *mut ImGuiStorage);
    pub fn igSetTooltip(fmt: *const c_char, ...);
    pub fn igSetWindowCollapsedBool(collapsed: bool, cond: ImGuiCond);
    pub fn igSetWindowCollapsedStr(name: *const c_char, collapsed: bool, cond: ImGuiCond);
    pub fn igSetWindowFocus();
    pub fn igSetWindowFocusStr(name: *const c_char);
    pub fn igSetWindowFontScale(scale: c_float);
    pub fn igSetWindowPosVec2(pos: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowPosStr(name: *const c_char, pos: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowSizeVec2(size: ImVec2, cond: ImGuiCond);
    pub fn igSetWindowSizeStr(name: *const c_char, size: ImVec2, cond: ImGuiCond);
    pub fn igShowAboutWindow(p_open: *mut bool);
    pub fn igShowDemoWindow(p_open: *mut bool);
    pub fn igShowFontSelector(label: *const c_char);
    pub fn igShowMetricsWindow(p_open: *mut bool);
    pub fn igShowStyleEditor(_ref: *mut ImGuiStyle);
    pub fn igShowStyleSelector(label: *const c_char) -> bool;
    pub fn igShowUserGuide();
    pub fn igSliderAngle(
        label: *const c_char,
        v_rad: *mut c_float,
        v_degrees_min: c_float,
        v_degrees_max: c_float,
        format: *const c_char,
    ) -> bool;
    pub fn igSliderFloat(
        label: *const c_char,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSliderFloat2(
        label: *const c_char,
        v: *mut [c_float; 2],
        v_min: c_float,
        v_max: c_float,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSliderFloat3(
        label: *const c_char,
        v: *mut [c_float; 3],
        v_min: c_float,
        v_max: c_float,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSliderFloat4(
        label: *const c_char,
        v: *mut [c_float; 4],
        v_min: c_float,
        v_max: c_float,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSliderInt(label: *const c_char, v: *mut c_int, v_min: c_int, v_max: c_int, format: *const c_char) -> bool;
    pub fn igSliderInt2(
        label: *const c_char,
        v: *mut [c_int; 2],
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igSliderInt3(
        label: *const c_char,
        v: *mut [c_int; 3],
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igSliderInt4(
        label: *const c_char,
        v: *mut [c_int; 4],
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igSliderScalar(
        label: *const c_char,
        data_type: ImGuiDataType,
        v: *mut c_void,
        v_min: *const c_void,
        v_max: *const c_void,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSliderScalarN(
        label: *const c_char,
        data_type: ImGuiDataType,
        v: *mut c_void,
        components: c_int,
        v_min: *const c_void,
        v_max: *const c_void,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igSmallButton(label: *const c_char) -> bool;
    pub fn igSpacing();
    pub fn igStyleColorsClassic(dst: *mut ImGuiStyle);
    pub fn igStyleColorsDark(dst: *mut ImGuiStyle);
    pub fn igStyleColorsLight(dst: *mut ImGuiStyle);
    pub fn igText(fmt: *const c_char, ...);
    pub fn igTextColored(col: ImVec4, fmt: *const c_char, ...);
    pub fn igTextDisabled(fmt: *const c_char, ...);
    pub fn igTextUnformatted(text: *const c_char, text_end: *const c_char);
    pub fn igTextWrapped(fmt: *const c_char, ...);
    pub fn igTreeAdvanceToLabelPos();
    pub fn igTreeNodeStr(label: *const c_char) -> bool;
    pub fn igTreeNodeStrStr(str_id: *const c_char, fmt: *const c_char, ...) -> bool;
    pub fn igTreeNodePtr(ptr_id: *const c_void, fmt: *const c_char, ...) -> bool;
    pub fn igTreeNodeExStr(label: *const c_char, flags: ImGuiTreeNodeFlags) -> bool;
    pub fn igTreeNodeExStrStr(str_id: *const c_char, flags: ImGuiTreeNodeFlags, fmt: *const c_char, ...) -> bool;
    pub fn igTreeNodeExPtr(ptr_id: *const c_void, flags: ImGuiTreeNodeFlags, fmt: *const c_char, ...) -> bool;
    pub fn igTreePop();
    pub fn igTreePushStr(str_id: *const c_char);
    pub fn igTreePushPtr(ptr_id: *const c_void);
    pub fn igUnindent(indent_w: c_float);
    pub fn igVSliderFloat(
        label: *const c_char,
        size: ImVec2,
        v: *mut c_float,
        v_min: c_float,
        v_max: c_float,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igVSliderInt(
        label: *const c_char,
        size: ImVec2,
        v: *mut c_int,
        v_min: c_int,
        v_max: c_int,
        format: *const c_char,
    ) -> bool;
    pub fn igVSliderScalar(
        label: *const c_char,
        size: ImVec2,
        data_type: ImGuiDataType,
        v: *mut c_void,
        v_min: *const c_void,
        v_max: *const c_void,
        format: *const c_char,
        power: c_float,
    ) -> bool;
    pub fn igValueBool(prefix: *const c_char, b: bool);
    pub fn igValueInt(prefix: *const c_char, v: c_int);
    pub fn igValueUint(prefix: *const c_char, v: c_uint);
    pub fn igValueFloat(prefix: *const c_char, v: c_float, float_format: *const c_char);
} // extern "C"
