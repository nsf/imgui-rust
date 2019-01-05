use super::{cstr_ptr, ImGui};
use nsf_imgui_raw::*;
use std::ffi::CStr;
impl ImGui {
    // pub fn accept_drag_drop_payload : (*const c_char, ImGuiDragDropFlags) -> *const ImGuiPayload
    #[inline]
    pub fn align_text_to_frame_padding(&self) {
        unsafe { igAlignTextToFramePadding() };
    }
    #[inline]
    pub fn arrow_button(&self, str_id: &CStr, dir: ImGuiDir) -> bool {
        unsafe { igArrowButton(str_id.as_ptr(), dir) }
    }
    #[inline]
    pub fn begin(&self, name: &CStr, p_open: &mut bool, flags: ImGuiWindowFlags) -> bool {
        unsafe { igBegin(name.as_ptr(), p_open, flags) }
    }
    #[inline]
    pub fn begin_child(&self, str_id: &CStr, size: ImVec2, border: bool, flags: ImGuiWindowFlags) -> bool {
        unsafe { igBeginChild(str_id.as_ptr(), size, border, flags) }
    }
    #[inline]
    pub fn begin_child_id(&self, id: ImGuiID, size: ImVec2, border: bool, flags: ImGuiWindowFlags) -> bool {
        unsafe { igBeginChildID(id, size, border, flags) }
    }
    #[inline]
    pub fn begin_child_frame(&self, id: ImGuiID, size: ImVec2, flags: ImGuiWindowFlags) -> bool {
        unsafe { igBeginChildFrame(id, size, flags) }
    }
    #[inline]
    pub fn begin_combo(&self, label: &CStr, preview_value: &CStr, flags: ImGuiComboFlags) -> bool {
        unsafe { igBeginCombo(label.as_ptr(), preview_value.as_ptr(), flags) }
    }
    #[inline]
    pub fn begin_drag_drop_source(&self, flags: ImGuiDragDropFlags) -> bool {
        unsafe { igBeginDragDropSource(flags) }
    }
    #[inline]
    pub fn begin_drag_drop_target(&self) -> bool {
        unsafe { igBeginDragDropTarget() }
    }
    #[inline]
    pub fn begin_group(&self) {
        unsafe { igBeginGroup() };
    }
    #[inline]
    pub fn begin_main_menu_bar(&self) -> bool {
        unsafe { igBeginMainMenuBar() }
    }
    #[inline]
    pub fn begin_menu(&self, label: &CStr, enabled: bool) -> bool {
        unsafe { igBeginMenu(label.as_ptr(), enabled) }
    }
    #[inline]
    pub fn begin_menu_bar(&self) -> bool {
        unsafe { igBeginMenuBar() }
    }
    #[inline]
    pub fn begin_popup(&self, str_id: &CStr, flags: ImGuiWindowFlags) -> bool {
        unsafe { igBeginPopup(str_id.as_ptr(), flags) }
    }
    #[inline]
    pub fn begin_popup_context_item(&self, str_id: &CStr, mouse_button: i32) -> bool {
        unsafe { igBeginPopupContextItem(str_id.as_ptr(), mouse_button) }
    }
    #[inline]
    pub fn begin_popup_context_void(&self, str_id: &CStr, mouse_button: i32) -> bool {
        unsafe { igBeginPopupContextVoid(str_id.as_ptr(), mouse_button) }
    }
    #[inline]
    pub fn begin_popup_context_window(&self, str_id: &CStr, mouse_button: i32, also_over_items: bool) -> bool {
        unsafe { igBeginPopupContextWindow(str_id.as_ptr(), mouse_button, also_over_items) }
    }
    #[inline]
    pub fn begin_popup_modal(&self, name: &CStr, p_open: &mut bool, flags: ImGuiWindowFlags) -> bool {
        unsafe { igBeginPopupModal(name.as_ptr(), p_open, flags) }
    }
    #[inline]
    pub fn begin_tooltip(&self) {
        unsafe { igBeginTooltip() };
    }
    #[inline]
    pub fn bullet(&self) {
        unsafe { igBullet() };
    }
    #[inline]
    pub fn bullet_text(&self, fmtstr: &CStr) {
        unsafe { igBulletText(cstr_ptr!("%s"), fmtstr.as_ptr()) };
    }
    #[inline]
    pub fn button(&self, label: &CStr, size: ImVec2) -> bool {
        unsafe { igButton(label.as_ptr(), size) }
    }
    #[inline]
    pub fn calc_item_width(&self) -> f32 {
        unsafe { igCalcItemWidth() }
    }
    #[inline]
    pub fn calc_list_clipping(
        &self,
        items_count: i32,
        items_height: f32,
        out_items_display_start: &mut i32,
        out_items_display_end: &mut i32,
    ) {
        unsafe {
            igCalcListClipping(
                items_count,
                items_height,
                out_items_display_start,
                out_items_display_end,
            )
        };
    }
    #[inline]
    pub fn calc_text_size(
        &self,
        text: &CStr,
        text_end: &CStr,
        hide_text_after_double_hash: bool,
        wrap_width: f32,
    ) -> ImVec2 {
        unsafe {
            igCalcTextSize(
                text.as_ptr(),
                text_end.as_ptr(),
                hide_text_after_double_hash,
                wrap_width,
            )
        }
    }
    #[inline]
    pub fn capture_keyboard_from_app(&self, capture: bool) {
        unsafe { igCaptureKeyboardFromApp(capture) };
    }
    #[inline]
    pub fn capture_mouse_from_app(&self, capture: bool) {
        unsafe { igCaptureMouseFromApp(capture) };
    }
    #[inline]
    pub fn checkbox(&self, label: &CStr, v: &mut bool) -> bool {
        unsafe { igCheckbox(label.as_ptr(), v) }
    }
    #[inline]
    pub fn checkbox_flags(&self, label: &CStr, flags: &mut u32, flags_value: u32) -> bool {
        unsafe { igCheckboxFlags(label.as_ptr(), flags, flags_value) }
    }
    #[inline]
    pub fn close_current_popup(&self) {
        unsafe { igCloseCurrentPopup() };
    }
    #[inline]
    pub fn collapsing_header(&self, label: &CStr, flags: ImGuiTreeNodeFlags) -> bool {
        unsafe { igCollapsingHeader(label.as_ptr(), flags) }
    }
    #[inline]
    pub fn collapsing_header_bool_ptr(&self, label: &CStr, p_open: &mut bool, flags: ImGuiTreeNodeFlags) -> bool {
        unsafe { igCollapsingHeaderBoolPtr(label.as_ptr(), p_open, flags) }
    }
    #[inline]
    pub fn color_button(&self, desc_id: &CStr, col: ImVec4, flags: ImGuiColorEditFlags, size: ImVec2) -> bool {
        unsafe { igColorButton(desc_id.as_ptr(), col, flags, size) }
    }
    #[inline]
    pub fn color_convert_float4_to_u32(&self, _in: ImVec4) -> u32 {
        unsafe { igColorConvertFloat4ToU32(_in) }
    }
    #[inline]
    pub fn color_convert_hs_vto_rgb(&self, h: f32, s: f32, v: f32, out_r: &mut f32, out_g: &mut f32, out_b: &mut f32) {
        unsafe { igColorConvertHSVtoRGB(h, s, v, out_r, out_g, out_b) };
    }
    #[inline]
    pub fn color_convert_rg_bto_hsv(&self, r: f32, g: f32, b: f32, out_h: &mut f32, out_s: &mut f32, out_v: &mut f32) {
        unsafe { igColorConvertRGBtoHSV(r, g, b, out_h, out_s, out_v) };
    }
    #[inline]
    pub fn color_convert_u32_to_float4(&self, _in: u32) -> ImVec4 {
        unsafe { igColorConvertU32ToFloat4(_in) }
    }
    #[inline]
    pub fn color_edit3(&self, label: &CStr, col: &mut [f32; 3], flags: ImGuiColorEditFlags) -> bool {
        unsafe { igColorEdit3(label.as_ptr(), col, flags) }
    }
    #[inline]
    pub fn color_edit4(&self, label: &CStr, col: &mut [f32; 4], flags: ImGuiColorEditFlags) -> bool {
        unsafe { igColorEdit4(label.as_ptr(), col, flags) }
    }
    #[inline]
    pub fn color_picker3(&self, label: &CStr, col: &mut [f32; 3], flags: ImGuiColorEditFlags) -> bool {
        unsafe { igColorPicker3(label.as_ptr(), col, flags) }
    }
    #[inline]
    pub fn color_picker4(&self, label: &CStr, col: &mut [f32; 4], flags: ImGuiColorEditFlags, ref_col: &f32) -> bool {
        unsafe { igColorPicker4(label.as_ptr(), col, flags, ref_col) }
    }
    #[inline]
    pub fn columns(&self, count: i32, id: &CStr, border: bool) {
        unsafe { igColumns(count, id.as_ptr(), border) };
    }
    // pub fn combo : (*const c_char, *mut c_int, *const *const c_char, c_int, c_int) -> bool
    #[inline]
    pub fn combo_str(
        &self,
        label: &CStr,
        current_item: &mut i32,
        items_separated_by_zeros: &CStr,
        popup_max_height_in_items: i32,
    ) -> bool {
        unsafe {
            igComboStr(
                label.as_ptr(),
                current_item,
                items_separated_by_zeros.as_ptr(),
                popup_max_height_in_items,
            )
        }
    }
    // pub fn combo_fn_ptr : (*const c_char, *mut c_int, extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char) -> bool, *mut c_void, c_int, c_int) -> bool
    #[inline]
    pub fn debug_check_version_and_data_layout(
        &self,
        version_str: &CStr,
        sz_io: usize,
        sz_style: usize,
        sz_vec2: usize,
        sz_vec4: usize,
        sz_drawvert: usize,
    ) -> bool {
        unsafe {
            igDebugCheckVersionAndDataLayout(version_str.as_ptr(), sz_io, sz_style, sz_vec2, sz_vec4, sz_drawvert)
        }
    }
    #[inline]
    pub fn drag_float(
        &self,
        label: &CStr,
        v: &mut f32,
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: &CStr,
        power: f32,
    ) -> bool {
        unsafe { igDragFloat(label.as_ptr(), v, v_speed, v_min, v_max, format.as_ptr(), power) }
    }
    #[inline]
    pub fn drag_float2(
        &self,
        label: &CStr,
        v: &mut [f32; 2],
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: &CStr,
        power: f32,
    ) -> bool {
        unsafe { igDragFloat2(label.as_ptr(), v, v_speed, v_min, v_max, format.as_ptr(), power) }
    }
    #[inline]
    pub fn drag_float3(
        &self,
        label: &CStr,
        v: &mut [f32; 3],
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: &CStr,
        power: f32,
    ) -> bool {
        unsafe { igDragFloat3(label.as_ptr(), v, v_speed, v_min, v_max, format.as_ptr(), power) }
    }
    #[inline]
    pub fn drag_float4(
        &self,
        label: &CStr,
        v: &mut [f32; 4],
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: &CStr,
        power: f32,
    ) -> bool {
        unsafe { igDragFloat4(label.as_ptr(), v, v_speed, v_min, v_max, format.as_ptr(), power) }
    }
    #[inline]
    pub fn drag_float_range2(
        &self,
        label: &CStr,
        v_current_min: &mut f32,
        v_current_max: &mut f32,
        v_speed: f32,
        v_min: f32,
        v_max: f32,
        format: &CStr,
        format_max: &CStr,
        power: f32,
    ) -> bool {
        unsafe {
            igDragFloatRange2(
                label.as_ptr(),
                v_current_min,
                v_current_max,
                v_speed,
                v_min,
                v_max,
                format.as_ptr(),
                format_max.as_ptr(),
                power,
            )
        }
    }
    #[inline]
    pub fn drag_int(&self, label: &CStr, v: &mut i32, v_speed: f32, v_min: i32, v_max: i32, format: &CStr) -> bool {
        unsafe { igDragInt(label.as_ptr(), v, v_speed, v_min, v_max, format.as_ptr()) }
    }
    #[inline]
    pub fn drag_int2(
        &self,
        label: &CStr,
        v: &mut [i32; 2],
        v_speed: f32,
        v_min: i32,
        v_max: i32,
        format: &CStr,
    ) -> bool {
        unsafe { igDragInt2(label.as_ptr(), v, v_speed, v_min, v_max, format.as_ptr()) }
    }
    #[inline]
    pub fn drag_int3(
        &self,
        label: &CStr,
        v: &mut [i32; 3],
        v_speed: f32,
        v_min: i32,
        v_max: i32,
        format: &CStr,
    ) -> bool {
        unsafe { igDragInt3(label.as_ptr(), v, v_speed, v_min, v_max, format.as_ptr()) }
    }
    #[inline]
    pub fn drag_int4(
        &self,
        label: &CStr,
        v: &mut [i32; 4],
        v_speed: f32,
        v_min: i32,
        v_max: i32,
        format: &CStr,
    ) -> bool {
        unsafe { igDragInt4(label.as_ptr(), v, v_speed, v_min, v_max, format.as_ptr()) }
    }
    #[inline]
    pub fn drag_int_range2(
        &self,
        label: &CStr,
        v_current_min: &mut i32,
        v_current_max: &mut i32,
        v_speed: f32,
        v_min: i32,
        v_max: i32,
        format: &CStr,
        format_max: &CStr,
    ) -> bool {
        unsafe {
            igDragIntRange2(
                label.as_ptr(),
                v_current_min,
                v_current_max,
                v_speed,
                v_min,
                v_max,
                format.as_ptr(),
                format_max.as_ptr(),
            )
        }
    }
    // pub fn drag_scalar : (*const c_char, ImGuiDataType, *mut c_void, c_float, *const c_void, *const c_void, *const c_char, c_float) -> bool
    // pub fn drag_scalar_n : (*const c_char, ImGuiDataType, *mut c_void, c_int, c_float, *const c_void, *const c_void, *const c_char, c_float) -> bool
    #[inline]
    pub fn dummy(&self, size: ImVec2) {
        unsafe { igDummy(size) };
    }
    #[inline]
    pub fn end(&self) {
        unsafe { igEnd() };
    }
    #[inline]
    pub fn end_child(&self) {
        unsafe { igEndChild() };
    }
    #[inline]
    pub fn end_child_frame(&self) {
        unsafe { igEndChildFrame() };
    }
    #[inline]
    pub fn end_combo(&self) {
        unsafe { igEndCombo() };
    }
    #[inline]
    pub fn end_drag_drop_source(&self) {
        unsafe { igEndDragDropSource() };
    }
    #[inline]
    pub fn end_drag_drop_target(&self) {
        unsafe { igEndDragDropTarget() };
    }
    #[inline]
    pub fn end_frame(&self) {
        unsafe { igEndFrame() };
    }
    #[inline]
    pub fn end_group(&self) {
        unsafe { igEndGroup() };
    }
    #[inline]
    pub fn end_main_menu_bar(&self) {
        unsafe { igEndMainMenuBar() };
    }
    #[inline]
    pub fn end_menu(&self) {
        unsafe { igEndMenu() };
    }
    #[inline]
    pub fn end_menu_bar(&self) {
        unsafe { igEndMenuBar() };
    }
    #[inline]
    pub fn end_popup(&self) {
        unsafe { igEndPopup() };
    }
    #[inline]
    pub fn end_tooltip(&self) {
        unsafe { igEndTooltip() };
    }
    #[inline]
    pub fn get_clipboard_text(&self) -> String {
        unsafe { CStr::from_ptr(igGetClipboardText()).to_string_lossy().into_owned() }
    }
    #[inline]
    pub fn get_color_u32(&self, idx: ImGuiCol, alpha_mul: f32) -> u32 {
        unsafe { igGetColorU32(idx, alpha_mul) }
    }
    #[inline]
    pub fn get_color_u32_vec4(&self, col: ImVec4) -> u32 {
        unsafe { igGetColorU32Vec4(col) }
    }
    #[inline]
    pub fn get_color_u32_u32(&self, col: u32) -> u32 {
        unsafe { igGetColorU32U32(col) }
    }
    #[inline]
    pub fn get_column_index(&self) -> i32 {
        unsafe { igGetColumnIndex() }
    }
    #[inline]
    pub fn get_column_offset(&self, column_index: i32) -> f32 {
        unsafe { igGetColumnOffset(column_index) }
    }
    #[inline]
    pub fn get_column_width(&self, column_index: i32) -> f32 {
        unsafe { igGetColumnWidth(column_index) }
    }
    #[inline]
    pub fn get_columns_count(&self) -> i32 {
        unsafe { igGetColumnsCount() }
    }
    #[inline]
    pub fn get_content_region_avail(&self) -> ImVec2 {
        unsafe { igGetContentRegionAvail() }
    }
    #[inline]
    pub fn get_content_region_avail_width(&self) -> f32 {
        unsafe { igGetContentRegionAvailWidth() }
    }
    #[inline]
    pub fn get_content_region_max(&self) -> ImVec2 {
        unsafe { igGetContentRegionMax() }
    }
    // pub fn get_current_context : () -> *mut ImGuiContext
    #[inline]
    pub fn get_cursor_pos(&self) -> ImVec2 {
        unsafe { igGetCursorPos() }
    }
    #[inline]
    pub fn get_cursor_pos_x(&self) -> f32 {
        unsafe { igGetCursorPosX() }
    }
    #[inline]
    pub fn get_cursor_pos_y(&self) -> f32 {
        unsafe { igGetCursorPosY() }
    }
    #[inline]
    pub fn get_cursor_screen_pos(&self) -> ImVec2 {
        unsafe { igGetCursorScreenPos() }
    }
    #[inline]
    pub fn get_cursor_start_pos(&self) -> ImVec2 {
        unsafe { igGetCursorStartPos() }
    }
    // pub fn get_drag_drop_payload : () -> *const ImGuiPayload
    // pub fn get_draw_data : () -> *mut ImDrawData
    // pub fn get_draw_list_shared_data : () -> *mut ImDrawListSharedData
    // pub fn get_font : () -> *mut ImFont
    #[inline]
    pub fn get_font_size(&self) -> f32 {
        unsafe { igGetFontSize() }
    }
    #[inline]
    pub fn get_font_tex_uv_white_pixel(&self) -> ImVec2 {
        unsafe { igGetFontTexUvWhitePixel() }
    }
    #[inline]
    pub fn get_frame_count(&self) -> i32 {
        unsafe { igGetFrameCount() }
    }
    #[inline]
    pub fn get_frame_height(&self) -> f32 {
        unsafe { igGetFrameHeight() }
    }
    #[inline]
    pub fn get_frame_height_with_spacing(&self) -> f32 {
        unsafe { igGetFrameHeightWithSpacing() }
    }
    #[inline]
    pub fn get_id_str(&self, str_id: &CStr) -> ImGuiID {
        unsafe { igGetIDStr(str_id.as_ptr()) }
    }
    #[inline]
    pub fn get_id_range(&self, str_id_begin: &CStr, str_id_end: &CStr) -> ImGuiID {
        unsafe { igGetIDRange(str_id_begin.as_ptr(), str_id_end.as_ptr()) }
    }
    // pub fn get_id_ptr : (*const c_void) -> ImGuiID
    // pub fn get_io : () -> *mut ImGuiIO
    #[inline]
    pub fn get_item_rect_max(&self) -> ImVec2 {
        unsafe { igGetItemRectMax() }
    }
    #[inline]
    pub fn get_item_rect_min(&self) -> ImVec2 {
        unsafe { igGetItemRectMin() }
    }
    #[inline]
    pub fn get_item_rect_size(&self) -> ImVec2 {
        unsafe { igGetItemRectSize() }
    }
    #[inline]
    pub fn get_key_index(&self, imgui_key: ImGuiKey) -> i32 {
        unsafe { igGetKeyIndex(imgui_key) }
    }
    #[inline]
    pub fn get_key_pressed_amount(&self, key_index: i32, repeat_delay: f32, rate: f32) -> i32 {
        unsafe { igGetKeyPressedAmount(key_index, repeat_delay, rate) }
    }
    #[inline]
    pub fn get_mouse_cursor(&self) -> ImGuiMouseCursor {
        unsafe { igGetMouseCursor() }
    }
    #[inline]
    pub fn get_mouse_drag_delta(&self, button: i32, lock_threshold: f32) -> ImVec2 {
        unsafe { igGetMouseDragDelta(button, lock_threshold) }
    }
    #[inline]
    pub fn get_mouse_pos(&self) -> ImVec2 {
        unsafe { igGetMousePos() }
    }
    #[inline]
    pub fn get_mouse_pos_on_opening_current_popup(&self) -> ImVec2 {
        unsafe { igGetMousePosOnOpeningCurrentPopup() }
    }
    // pub fn get_overlay_draw_list : () -> *mut ImDrawList
    #[inline]
    pub fn get_scroll_max_x(&self) -> f32 {
        unsafe { igGetScrollMaxX() }
    }
    #[inline]
    pub fn get_scroll_max_y(&self) -> f32 {
        unsafe { igGetScrollMaxY() }
    }
    #[inline]
    pub fn get_scroll_x(&self) -> f32 {
        unsafe { igGetScrollX() }
    }
    #[inline]
    pub fn get_scroll_y(&self) -> f32 {
        unsafe { igGetScrollY() }
    }
    // pub fn get_state_storage : () -> *mut ImGuiStorage
    // pub fn get_style : () -> *mut ImGuiStyle
    #[inline]
    pub fn get_style_color_name(&self, idx: ImGuiCol) -> String {
        unsafe { CStr::from_ptr(igGetStyleColorName(idx)).to_string_lossy().into_owned() }
    }
    // pub fn get_style_color_vec4 : (ImGuiCol) -> *const ImVec4
    #[inline]
    pub fn get_text_line_height(&self) -> f32 {
        unsafe { igGetTextLineHeight() }
    }
    #[inline]
    pub fn get_text_line_height_with_spacing(&self) -> f32 {
        unsafe { igGetTextLineHeightWithSpacing() }
    }
    #[inline]
    pub fn get_time(&self) -> f64 {
        unsafe { igGetTime() }
    }
    #[inline]
    pub fn get_tree_node_to_label_spacing(&self) -> f32 {
        unsafe { igGetTreeNodeToLabelSpacing() }
    }
    #[inline]
    pub fn get_version(&self) -> String {
        unsafe { CStr::from_ptr(igGetVersion()).to_string_lossy().into_owned() }
    }
    #[inline]
    pub fn get_window_content_region_max(&self) -> ImVec2 {
        unsafe { igGetWindowContentRegionMax() }
    }
    #[inline]
    pub fn get_window_content_region_min(&self) -> ImVec2 {
        unsafe { igGetWindowContentRegionMin() }
    }
    #[inline]
    pub fn get_window_content_region_width(&self) -> f32 {
        unsafe { igGetWindowContentRegionWidth() }
    }
    // pub fn get_window_draw_list : () -> *mut ImDrawList
    #[inline]
    pub fn get_window_height(&self) -> f32 {
        unsafe { igGetWindowHeight() }
    }
    #[inline]
    pub fn get_window_pos(&self) -> ImVec2 {
        unsafe { igGetWindowPos() }
    }
    #[inline]
    pub fn get_window_size(&self) -> ImVec2 {
        unsafe { igGetWindowSize() }
    }
    #[inline]
    pub fn get_window_width(&self) -> f32 {
        unsafe { igGetWindowWidth() }
    }
    #[inline]
    pub fn image(
        &self,
        user_texture_id: ImTextureID,
        size: ImVec2,
        uv0: ImVec2,
        uv1: ImVec2,
        tint_col: ImVec4,
        border_col: ImVec4,
    ) {
        unsafe { igImage(user_texture_id, size, uv0, uv1, tint_col, border_col) };
    }
    #[inline]
    pub fn image_button(
        &self,
        user_texture_id: ImTextureID,
        size: ImVec2,
        uv0: ImVec2,
        uv1: ImVec2,
        frame_padding: i32,
        bg_col: ImVec4,
        tint_col: ImVec4,
    ) -> bool {
        unsafe { igImageButton(user_texture_id, size, uv0, uv1, frame_padding, bg_col, tint_col) }
    }
    #[inline]
    pub fn indent(&self, indent_w: f32) {
        unsafe { igIndent(indent_w) };
    }
    #[inline]
    pub fn input_double(
        &self,
        label: &CStr,
        v: &mut f64,
        step: f64,
        step_fast: f64,
        format: &CStr,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool {
        unsafe { igInputDouble(label.as_ptr(), v, step, step_fast, format.as_ptr(), extra_flags) }
    }
    #[inline]
    pub fn input_float(
        &self,
        label: &CStr,
        v: &mut f32,
        step: f32,
        step_fast: f32,
        format: &CStr,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool {
        unsafe { igInputFloat(label.as_ptr(), v, step, step_fast, format.as_ptr(), extra_flags) }
    }
    #[inline]
    pub fn input_float2(
        &self,
        label: &CStr,
        v: &mut [f32; 2],
        format: &CStr,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool {
        unsafe { igInputFloat2(label.as_ptr(), v, format.as_ptr(), extra_flags) }
    }
    #[inline]
    pub fn input_float3(
        &self,
        label: &CStr,
        v: &mut [f32; 3],
        format: &CStr,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool {
        unsafe { igInputFloat3(label.as_ptr(), v, format.as_ptr(), extra_flags) }
    }
    #[inline]
    pub fn input_float4(
        &self,
        label: &CStr,
        v: &mut [f32; 4],
        format: &CStr,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool {
        unsafe { igInputFloat4(label.as_ptr(), v, format.as_ptr(), extra_flags) }
    }
    #[inline]
    pub fn input_int(
        &self,
        label: &CStr,
        v: &mut i32,
        step: i32,
        step_fast: i32,
        extra_flags: ImGuiInputTextFlags,
    ) -> bool {
        unsafe { igInputInt(label.as_ptr(), v, step, step_fast, extra_flags) }
    }
    #[inline]
    pub fn input_int2(&self, label: &CStr, v: &mut [i32; 2], extra_flags: ImGuiInputTextFlags) -> bool {
        unsafe { igInputInt2(label.as_ptr(), v, extra_flags) }
    }
    #[inline]
    pub fn input_int3(&self, label: &CStr, v: &mut [i32; 3], extra_flags: ImGuiInputTextFlags) -> bool {
        unsafe { igInputInt3(label.as_ptr(), v, extra_flags) }
    }
    #[inline]
    pub fn input_int4(&self, label: &CStr, v: &mut [i32; 4], extra_flags: ImGuiInputTextFlags) -> bool {
        unsafe { igInputInt4(label.as_ptr(), v, extra_flags) }
    }
    // pub fn input_scalar : (*const c_char, ImGuiDataType, *mut c_void, *const c_void, *const c_void, *const c_char, ImGuiInputTextFlags) -> bool
    // pub fn input_scalar_n : (*const c_char, ImGuiDataType, *mut c_void, c_int, *const c_void, *const c_void, *const c_char, ImGuiInputTextFlags) -> bool
    // pub fn input_text : (*const c_char, *mut c_char, size_t, ImGuiInputTextFlags, ImGuiInputTextCallback, *mut c_void) -> bool
    // pub fn input_text_multiline : (*const c_char, *mut c_char, size_t, ImVec2, ImGuiInputTextFlags, ImGuiInputTextCallback, *mut c_void) -> bool
    #[inline]
    pub fn invisible_button(&self, str_id: &CStr, size: ImVec2) -> bool {
        unsafe { igInvisibleButton(str_id.as_ptr(), size) }
    }
    #[inline]
    pub fn is_any_item_active(&self) -> bool {
        unsafe { igIsAnyItemActive() }
    }
    #[inline]
    pub fn is_any_item_focused(&self) -> bool {
        unsafe { igIsAnyItemFocused() }
    }
    #[inline]
    pub fn is_any_item_hovered(&self) -> bool {
        unsafe { igIsAnyItemHovered() }
    }
    #[inline]
    pub fn is_any_mouse_down(&self) -> bool {
        unsafe { igIsAnyMouseDown() }
    }
    #[inline]
    pub fn is_item_active(&self) -> bool {
        unsafe { igIsItemActive() }
    }
    #[inline]
    pub fn is_item_clicked(&self, mouse_button: i32) -> bool {
        unsafe { igIsItemClicked(mouse_button) }
    }
    #[inline]
    pub fn is_item_deactivated(&self) -> bool {
        unsafe { igIsItemDeactivated() }
    }
    #[inline]
    pub fn is_item_deactivated_after_edit(&self) -> bool {
        unsafe { igIsItemDeactivatedAfterEdit() }
    }
    #[inline]
    pub fn is_item_edited(&self) -> bool {
        unsafe { igIsItemEdited() }
    }
    #[inline]
    pub fn is_item_focused(&self) -> bool {
        unsafe { igIsItemFocused() }
    }
    #[inline]
    pub fn is_item_hovered(&self, flags: ImGuiHoveredFlags) -> bool {
        unsafe { igIsItemHovered(flags) }
    }
    #[inline]
    pub fn is_item_visible(&self) -> bool {
        unsafe { igIsItemVisible() }
    }
    #[inline]
    pub fn is_key_down(&self, user_key_index: i32) -> bool {
        unsafe { igIsKeyDown(user_key_index) }
    }
    #[inline]
    pub fn is_key_pressed(&self, user_key_index: i32, repeat: bool) -> bool {
        unsafe { igIsKeyPressed(user_key_index, repeat) }
    }
    #[inline]
    pub fn is_key_released(&self, user_key_index: i32) -> bool {
        unsafe { igIsKeyReleased(user_key_index) }
    }
    #[inline]
    pub fn is_mouse_clicked(&self, button: i32, repeat: bool) -> bool {
        unsafe { igIsMouseClicked(button, repeat) }
    }
    #[inline]
    pub fn is_mouse_double_clicked(&self, button: i32) -> bool {
        unsafe { igIsMouseDoubleClicked(button) }
    }
    #[inline]
    pub fn is_mouse_down(&self, button: i32) -> bool {
        unsafe { igIsMouseDown(button) }
    }
    #[inline]
    pub fn is_mouse_dragging(&self, button: i32, lock_threshold: f32) -> bool {
        unsafe { igIsMouseDragging(button, lock_threshold) }
    }
    #[inline]
    pub fn is_mouse_hovering_rect(&self, r_min: ImVec2, r_max: ImVec2, clip: bool) -> bool {
        unsafe { igIsMouseHoveringRect(r_min, r_max, clip) }
    }
    #[inline]
    pub fn is_mouse_pos_valid(&self, mouse_pos: &ImVec2) -> bool {
        unsafe { igIsMousePosValid(mouse_pos) }
    }
    #[inline]
    pub fn is_mouse_released(&self, button: i32) -> bool {
        unsafe { igIsMouseReleased(button) }
    }
    #[inline]
    pub fn is_popup_open(&self, str_id: &CStr) -> bool {
        unsafe { igIsPopupOpen(str_id.as_ptr()) }
    }
    #[inline]
    pub fn is_rect_visible(&self, size: ImVec2) -> bool {
        unsafe { igIsRectVisible(size) }
    }
    #[inline]
    pub fn is_rect_visible_vec2(&self, rect_min: ImVec2, rect_max: ImVec2) -> bool {
        unsafe { igIsRectVisibleVec2(rect_min, rect_max) }
    }
    #[inline]
    pub fn is_window_appearing(&self) -> bool {
        unsafe { igIsWindowAppearing() }
    }
    #[inline]
    pub fn is_window_collapsed(&self) -> bool {
        unsafe { igIsWindowCollapsed() }
    }
    #[inline]
    pub fn is_window_focused(&self, flags: ImGuiFocusedFlags) -> bool {
        unsafe { igIsWindowFocused(flags) }
    }
    #[inline]
    pub fn is_window_hovered(&self, flags: ImGuiHoveredFlags) -> bool {
        unsafe { igIsWindowHovered(flags) }
    }
    #[inline]
    pub fn label_text(&self, label: &CStr, fmtstr: &CStr) {
        unsafe { igLabelText(label.as_ptr(), cstr_ptr!("%s"), fmtstr.as_ptr()) };
    }
    // pub fn list_box_str_arr : (*const c_char, *mut c_int, *const *const c_char, c_int, c_int) -> bool
    // pub fn list_box_fn_ptr : (*const c_char, *mut c_int, extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char) -> bool, *mut c_void, c_int, c_int) -> bool
    #[inline]
    pub fn list_box_footer(&self) {
        unsafe { igListBoxFooter() };
    }
    #[inline]
    pub fn list_box_header_vec2(&self, label: &CStr, size: ImVec2) -> bool {
        unsafe { igListBoxHeaderVec2(label.as_ptr(), size) }
    }
    #[inline]
    pub fn list_box_header_int(&self, label: &CStr, items_count: i32, height_in_items: i32) -> bool {
        unsafe { igListBoxHeaderInt(label.as_ptr(), items_count, height_in_items) }
    }
    #[inline]
    pub fn load_ini_settings_from_disk(&self, ini_filename: &CStr) {
        unsafe { igLoadIniSettingsFromDisk(ini_filename.as_ptr()) };
    }
    #[inline]
    pub fn load_ini_settings_from_memory(&self, ini_data: &CStr, ini_size: usize) {
        unsafe { igLoadIniSettingsFromMemory(ini_data.as_ptr(), ini_size) };
    }
    #[inline]
    pub fn log_buttons(&self) {
        unsafe { igLogButtons() };
    }
    #[inline]
    pub fn log_finish(&self) {
        unsafe { igLogFinish() };
    }
    #[inline]
    pub fn log_text(&self, fmtstr: &CStr) {
        unsafe { igLogText(cstr_ptr!("%s"), fmtstr.as_ptr()) };
    }
    #[inline]
    pub fn log_to_clipboard(&self, max_depth: i32) {
        unsafe { igLogToClipboard(max_depth) };
    }
    #[inline]
    pub fn log_to_file(&self, max_depth: i32, filename: &CStr) {
        unsafe { igLogToFile(max_depth, filename.as_ptr()) };
    }
    #[inline]
    pub fn log_to_tty(&self, max_depth: i32) {
        unsafe { igLogToTTY(max_depth) };
    }
    #[inline]
    pub fn menu_item_bool(&self, label: &CStr, shortcut: &CStr, selected: bool, enabled: bool) -> bool {
        unsafe { igMenuItemBool(label.as_ptr(), shortcut.as_ptr(), selected, enabled) }
    }
    #[inline]
    pub fn menu_item_bool_ptr(&self, label: &CStr, shortcut: &CStr, p_selected: &mut bool, enabled: bool) -> bool {
        unsafe { igMenuItemBoolPtr(label.as_ptr(), shortcut.as_ptr(), p_selected, enabled) }
    }
    #[inline]
    pub fn new_frame(&self) {
        unsafe { igNewFrame() };
    }
    #[inline]
    pub fn new_line(&self) {
        unsafe { igNewLine() };
    }
    #[inline]
    pub fn next_column(&self) {
        unsafe { igNextColumn() };
    }
    #[inline]
    pub fn open_popup(&self, str_id: &CStr) {
        unsafe { igOpenPopup(str_id.as_ptr()) };
    }
    #[inline]
    pub fn open_popup_on_item_click(&self, str_id: &CStr, mouse_button: i32) -> bool {
        unsafe { igOpenPopupOnItemClick(str_id.as_ptr(), mouse_button) }
    }
    #[inline]
    pub fn plot_histogram_float_ptr(
        &self,
        label: &CStr,
        values: &f32,
        values_count: i32,
        values_offset: i32,
        overlay_text: &CStr,
        scale_min: f32,
        scale_max: f32,
        graph_size: ImVec2,
        stride: i32,
    ) {
        unsafe {
            igPlotHistogramFloatPtr(
                label.as_ptr(),
                values,
                values_count,
                values_offset,
                overlay_text.as_ptr(),
                scale_min,
                scale_max,
                graph_size,
                stride,
            )
        };
    }
    // pub fn plot_histogram_fn_ptr : (*const c_char, extern "C" fn(data: *mut c_void, idx: c_int) -> c_float, *mut c_void, c_int, c_int, *const c_char, c_float, c_float, ImVec2) -> undefined
    #[inline]
    pub fn plot_lines(
        &self,
        label: &CStr,
        values: &f32,
        values_count: i32,
        values_offset: i32,
        overlay_text: &CStr,
        scale_min: f32,
        scale_max: f32,
        graph_size: ImVec2,
        stride: i32,
    ) {
        unsafe {
            igPlotLines(
                label.as_ptr(),
                values,
                values_count,
                values_offset,
                overlay_text.as_ptr(),
                scale_min,
                scale_max,
                graph_size,
                stride,
            )
        };
    }
    // pub fn plot_lines_fn_ptr : (*const c_char, extern "C" fn(data: *mut c_void, idx: c_int) -> c_float, *mut c_void, c_int, c_int, *const c_char, c_float, c_float, ImVec2) -> undefined
    #[inline]
    pub fn pop_allow_keyboard_focus(&self) {
        unsafe { igPopAllowKeyboardFocus() };
    }
    #[inline]
    pub fn pop_button_repeat(&self) {
        unsafe { igPopButtonRepeat() };
    }
    #[inline]
    pub fn pop_clip_rect(&self) {
        unsafe { igPopClipRect() };
    }
    #[inline]
    pub fn pop_font(&self) {
        unsafe { igPopFont() };
    }
    #[inline]
    pub fn pop_id(&self) {
        unsafe { igPopID() };
    }
    #[inline]
    pub fn pop_item_width(&self) {
        unsafe { igPopItemWidth() };
    }
    #[inline]
    pub fn pop_style_color(&self, count: i32) {
        unsafe { igPopStyleColor(count) };
    }
    #[inline]
    pub fn pop_style_var(&self, count: i32) {
        unsafe { igPopStyleVar(count) };
    }
    #[inline]
    pub fn pop_text_wrap_pos(&self) {
        unsafe { igPopTextWrapPos() };
    }
    #[inline]
    pub fn progress_bar(&self, fraction: f32, size_arg: ImVec2, overlay: &CStr) {
        unsafe { igProgressBar(fraction, size_arg, overlay.as_ptr()) };
    }
    #[inline]
    pub fn push_allow_keyboard_focus(&self, allow_keyboard_focus: bool) {
        unsafe { igPushAllowKeyboardFocus(allow_keyboard_focus) };
    }
    #[inline]
    pub fn push_button_repeat(&self, repeat: bool) {
        unsafe { igPushButtonRepeat(repeat) };
    }
    #[inline]
    pub fn push_clip_rect(&self, clip_rect_min: ImVec2, clip_rect_max: ImVec2, intersect_with_current_clip_rect: bool) {
        unsafe { igPushClipRect(clip_rect_min, clip_rect_max, intersect_with_current_clip_rect) };
    }
    // pub fn push_font : (*mut ImFont) -> undefined
    #[inline]
    pub fn push_id_str(&self, str_id: &CStr) {
        unsafe { igPushIDStr(str_id.as_ptr()) };
    }
    #[inline]
    pub fn push_id_range(&self, str_id_begin: &CStr, str_id_end: &CStr) {
        unsafe { igPushIDRange(str_id_begin.as_ptr(), str_id_end.as_ptr()) };
    }
    // pub fn push_id_ptr : (*const c_void) -> undefined
    #[inline]
    pub fn push_id_int(&self, int_id: i32) {
        unsafe { igPushIDInt(int_id) };
    }
    #[inline]
    pub fn push_item_width(&self, item_width: f32) {
        unsafe { igPushItemWidth(item_width) };
    }
    #[inline]
    pub fn push_style_color_u32(&self, idx: ImGuiCol, col: u32) {
        unsafe { igPushStyleColorU32(idx, col) };
    }
    #[inline]
    pub fn push_style_color(&self, idx: ImGuiCol, col: ImVec4) {
        unsafe { igPushStyleColor(idx, col) };
    }
    #[inline]
    pub fn push_style_var_float(&self, idx: ImGuiStyleVar, val: f32) {
        unsafe { igPushStyleVarFloat(idx, val) };
    }
    #[inline]
    pub fn push_style_var_vec2(&self, idx: ImGuiStyleVar, val: ImVec2) {
        unsafe { igPushStyleVarVec2(idx, val) };
    }
    #[inline]
    pub fn push_text_wrap_pos(&self, wrap_pos_x: f32) {
        unsafe { igPushTextWrapPos(wrap_pos_x) };
    }
    #[inline]
    pub fn radio_button_bool(&self, label: &CStr, active: bool) -> bool {
        unsafe { igRadioButtonBool(label.as_ptr(), active) }
    }
    #[inline]
    pub fn radio_button_int_ptr(&self, label: &CStr, v: &mut i32, v_button: i32) -> bool {
        unsafe { igRadioButtonIntPtr(label.as_ptr(), v, v_button) }
    }
    #[inline]
    pub fn render(&self) {
        unsafe { igRender() };
    }
    #[inline]
    pub fn reset_mouse_drag_delta(&self, button: i32) {
        unsafe { igResetMouseDragDelta(button) };
    }
    #[inline]
    pub fn same_line(&self, pos_x: f32, spacing_w: f32) {
        unsafe { igSameLine(pos_x, spacing_w) };
    }
    #[inline]
    pub fn save_ini_settings_to_disk(&self, ini_filename: &CStr) {
        unsafe { igSaveIniSettingsToDisk(ini_filename.as_ptr()) };
    }
    #[inline]
    pub fn save_ini_settings_to_memory(&self, out_ini_size: &mut usize) -> String {
        unsafe {
            CStr::from_ptr(igSaveIniSettingsToMemory(out_ini_size))
                .to_string_lossy()
                .into_owned()
        }
    }
    #[inline]
    pub fn selectable(&self, label: &CStr, selected: bool, flags: ImGuiSelectableFlags, size: ImVec2) -> bool {
        unsafe { igSelectable(label.as_ptr(), selected, flags, size) }
    }
    #[inline]
    pub fn selectable_bool_ptr(
        &self,
        label: &CStr,
        p_selected: &mut bool,
        flags: ImGuiSelectableFlags,
        size: ImVec2,
    ) -> bool {
        unsafe { igSelectableBoolPtr(label.as_ptr(), p_selected, flags, size) }
    }
    #[inline]
    pub fn separator(&self) {
        unsafe { igSeparator() };
    }
    #[inline]
    pub fn set_clipboard_text(&self, text: &CStr) {
        unsafe { igSetClipboardText(text.as_ptr()) };
    }
    #[inline]
    pub fn set_color_edit_options(&self, flags: ImGuiColorEditFlags) {
        unsafe { igSetColorEditOptions(flags) };
    }
    #[inline]
    pub fn set_column_offset(&self, column_index: i32, offset_x: f32) {
        unsafe { igSetColumnOffset(column_index, offset_x) };
    }
    #[inline]
    pub fn set_column_width(&self, column_index: i32, width: f32) {
        unsafe { igSetColumnWidth(column_index, width) };
    }
    #[inline]
    pub fn set_cursor_pos(&self, local_pos: ImVec2) {
        unsafe { igSetCursorPos(local_pos) };
    }
    #[inline]
    pub fn set_cursor_pos_x(&self, x: f32) {
        unsafe { igSetCursorPosX(x) };
    }
    #[inline]
    pub fn set_cursor_pos_y(&self, y: f32) {
        unsafe { igSetCursorPosY(y) };
    }
    #[inline]
    pub fn set_cursor_screen_pos(&self, screen_pos: ImVec2) {
        unsafe { igSetCursorScreenPos(screen_pos) };
    }
    // pub fn set_drag_drop_payload : (*const c_char, *const c_void, size_t, ImGuiCond) -> bool
    #[inline]
    pub fn set_item_allow_overlap(&self) {
        unsafe { igSetItemAllowOverlap() };
    }
    #[inline]
    pub fn set_item_default_focus(&self) {
        unsafe { igSetItemDefaultFocus() };
    }
    #[inline]
    pub fn set_keyboard_focus_here(&self, offset: i32) {
        unsafe { igSetKeyboardFocusHere(offset) };
    }
    #[inline]
    pub fn set_mouse_cursor(&self, _type: ImGuiMouseCursor) {
        unsafe { igSetMouseCursor(_type) };
    }
    #[inline]
    pub fn set_next_tree_node_open(&self, is_open: bool, cond: ImGuiCond) {
        unsafe { igSetNextTreeNodeOpen(is_open, cond) };
    }
    #[inline]
    pub fn set_next_window_bg_alpha(&self, alpha: f32) {
        unsafe { igSetNextWindowBgAlpha(alpha) };
    }
    #[inline]
    pub fn set_next_window_collapsed(&self, collapsed: bool, cond: ImGuiCond) {
        unsafe { igSetNextWindowCollapsed(collapsed, cond) };
    }
    #[inline]
    pub fn set_next_window_content_size(&self, size: ImVec2) {
        unsafe { igSetNextWindowContentSize(size) };
    }
    #[inline]
    pub fn set_next_window_focus(&self) {
        unsafe { igSetNextWindowFocus() };
    }
    #[inline]
    pub fn set_next_window_pos(&self, pos: ImVec2, cond: ImGuiCond, pivot: ImVec2) {
        unsafe { igSetNextWindowPos(pos, cond, pivot) };
    }
    #[inline]
    pub fn set_next_window_size(&self, size: ImVec2, cond: ImGuiCond) {
        unsafe { igSetNextWindowSize(size, cond) };
    }
    // pub fn set_next_window_size_constraints : (ImVec2, ImVec2, ImGuiSizeCallback, *mut c_void) -> undefined
    #[inline]
    pub fn set_scroll_from_pos_y(&self, pos_y: f32, center_y_ratio: f32) {
        unsafe { igSetScrollFromPosY(pos_y, center_y_ratio) };
    }
    #[inline]
    pub fn set_scroll_here_y(&self, center_y_ratio: f32) {
        unsafe { igSetScrollHereY(center_y_ratio) };
    }
    #[inline]
    pub fn set_scroll_x(&self, scroll_x: f32) {
        unsafe { igSetScrollX(scroll_x) };
    }
    #[inline]
    pub fn set_scroll_y(&self, scroll_y: f32) {
        unsafe { igSetScrollY(scroll_y) };
    }
    // pub fn set_state_storage : (*mut ImGuiStorage) -> undefined
    #[inline]
    pub fn set_tooltip(&self, fmtstr: &CStr) {
        unsafe { igSetTooltip(cstr_ptr!("%s"), fmtstr.as_ptr()) };
    }
    #[inline]
    pub fn set_window_collapsed_bool(&self, collapsed: bool, cond: ImGuiCond) {
        unsafe { igSetWindowCollapsedBool(collapsed, cond) };
    }
    #[inline]
    pub fn set_window_collapsed_str(&self, name: &CStr, collapsed: bool, cond: ImGuiCond) {
        unsafe { igSetWindowCollapsedStr(name.as_ptr(), collapsed, cond) };
    }
    #[inline]
    pub fn set_window_focus(&self) {
        unsafe { igSetWindowFocus() };
    }
    #[inline]
    pub fn set_window_focus_str(&self, name: &CStr) {
        unsafe { igSetWindowFocusStr(name.as_ptr()) };
    }
    #[inline]
    pub fn set_window_font_scale(&self, scale: f32) {
        unsafe { igSetWindowFontScale(scale) };
    }
    #[inline]
    pub fn set_window_pos_vec2(&self, pos: ImVec2, cond: ImGuiCond) {
        unsafe { igSetWindowPosVec2(pos, cond) };
    }
    #[inline]
    pub fn set_window_pos_str(&self, name: &CStr, pos: ImVec2, cond: ImGuiCond) {
        unsafe { igSetWindowPosStr(name.as_ptr(), pos, cond) };
    }
    #[inline]
    pub fn set_window_size_vec2(&self, size: ImVec2, cond: ImGuiCond) {
        unsafe { igSetWindowSizeVec2(size, cond) };
    }
    #[inline]
    pub fn set_window_size_str(&self, name: &CStr, size: ImVec2, cond: ImGuiCond) {
        unsafe { igSetWindowSizeStr(name.as_ptr(), size, cond) };
    }
    #[inline]
    pub fn show_about_window(&self, p_open: &mut bool) {
        unsafe { igShowAboutWindow(p_open) };
    }
    #[inline]
    pub fn show_demo_window(&self, p_open: &mut bool) {
        unsafe { igShowDemoWindow(p_open) };
    }
    #[inline]
    pub fn show_font_selector(&self, label: &CStr) {
        unsafe { igShowFontSelector(label.as_ptr()) };
    }
    #[inline]
    pub fn show_metrics_window(&self, p_open: &mut bool) {
        unsafe { igShowMetricsWindow(p_open) };
    }
    // pub fn show_style_editor : (*mut ImGuiStyle) -> undefined
    #[inline]
    pub fn show_style_selector(&self, label: &CStr) -> bool {
        unsafe { igShowStyleSelector(label.as_ptr()) }
    }
    #[inline]
    pub fn show_user_guide(&self) {
        unsafe { igShowUserGuide() };
    }
    #[inline]
    pub fn slider_angle(
        &self,
        label: &CStr,
        v_rad: &mut f32,
        v_degrees_min: f32,
        v_degrees_max: f32,
        format: &CStr,
    ) -> bool {
        unsafe { igSliderAngle(label.as_ptr(), v_rad, v_degrees_min, v_degrees_max, format.as_ptr()) }
    }
    #[inline]
    pub fn slider_float(&self, label: &CStr, v: &mut f32, v_min: f32, v_max: f32, format: &CStr, power: f32) -> bool {
        unsafe { igSliderFloat(label.as_ptr(), v, v_min, v_max, format.as_ptr(), power) }
    }
    #[inline]
    pub fn slider_float2(
        &self,
        label: &CStr,
        v: &mut [f32; 2],
        v_min: f32,
        v_max: f32,
        format: &CStr,
        power: f32,
    ) -> bool {
        unsafe { igSliderFloat2(label.as_ptr(), v, v_min, v_max, format.as_ptr(), power) }
    }
    #[inline]
    pub fn slider_float3(
        &self,
        label: &CStr,
        v: &mut [f32; 3],
        v_min: f32,
        v_max: f32,
        format: &CStr,
        power: f32,
    ) -> bool {
        unsafe { igSliderFloat3(label.as_ptr(), v, v_min, v_max, format.as_ptr(), power) }
    }
    #[inline]
    pub fn slider_float4(
        &self,
        label: &CStr,
        v: &mut [f32; 4],
        v_min: f32,
        v_max: f32,
        format: &CStr,
        power: f32,
    ) -> bool {
        unsafe { igSliderFloat4(label.as_ptr(), v, v_min, v_max, format.as_ptr(), power) }
    }
    #[inline]
    pub fn slider_int(&self, label: &CStr, v: &mut i32, v_min: i32, v_max: i32, format: &CStr) -> bool {
        unsafe { igSliderInt(label.as_ptr(), v, v_min, v_max, format.as_ptr()) }
    }
    #[inline]
    pub fn slider_int2(&self, label: &CStr, v: &mut [i32; 2], v_min: i32, v_max: i32, format: &CStr) -> bool {
        unsafe { igSliderInt2(label.as_ptr(), v, v_min, v_max, format.as_ptr()) }
    }
    #[inline]
    pub fn slider_int3(&self, label: &CStr, v: &mut [i32; 3], v_min: i32, v_max: i32, format: &CStr) -> bool {
        unsafe { igSliderInt3(label.as_ptr(), v, v_min, v_max, format.as_ptr()) }
    }
    #[inline]
    pub fn slider_int4(&self, label: &CStr, v: &mut [i32; 4], v_min: i32, v_max: i32, format: &CStr) -> bool {
        unsafe { igSliderInt4(label.as_ptr(), v, v_min, v_max, format.as_ptr()) }
    }
    // pub fn slider_scalar : (*const c_char, ImGuiDataType, *mut c_void, *const c_void, *const c_void, *const c_char, c_float) -> bool
    // pub fn slider_scalar_n : (*const c_char, ImGuiDataType, *mut c_void, c_int, *const c_void, *const c_void, *const c_char, c_float) -> bool
    #[inline]
    pub fn small_button(&self, label: &CStr) -> bool {
        unsafe { igSmallButton(label.as_ptr()) }
    }
    #[inline]
    pub fn spacing(&self) {
        unsafe { igSpacing() };
    }
    // pub fn style_colors_classic : (*mut ImGuiStyle) -> undefined
    // pub fn style_colors_dark : (*mut ImGuiStyle) -> undefined
    // pub fn style_colors_light : (*mut ImGuiStyle) -> undefined
    #[inline]
    pub fn text(&self, fmtstr: &CStr) {
        unsafe { igText(cstr_ptr!("%s"), fmtstr.as_ptr()) };
    }
    #[inline]
    pub fn text_colored(&self, col: ImVec4, fmtstr: &CStr) {
        unsafe { igTextColored(col, cstr_ptr!("%s"), fmtstr.as_ptr()) };
    }
    #[inline]
    pub fn text_disabled(&self, fmtstr: &CStr) {
        unsafe { igTextDisabled(cstr_ptr!("%s"), fmtstr.as_ptr()) };
    }
    #[inline]
    pub fn text_unformatted(&self, text: &CStr, text_end: &CStr) {
        unsafe { igTextUnformatted(text.as_ptr(), text_end.as_ptr()) };
    }
    #[inline]
    pub fn text_wrapped(&self, fmtstr: &CStr) {
        unsafe { igTextWrapped(cstr_ptr!("%s"), fmtstr.as_ptr()) };
    }
    #[inline]
    pub fn tree_advance_to_label_pos(&self) {
        unsafe { igTreeAdvanceToLabelPos() };
    }
    #[inline]
    pub fn tree_node_str(&self, label: &CStr) -> bool {
        unsafe { igTreeNodeStr(label.as_ptr()) }
    }
    #[inline]
    pub fn tree_node_str_str(&self, str_id: &CStr, fmtstr: &CStr) -> bool {
        unsafe { igTreeNodeStrStr(str_id.as_ptr(), cstr_ptr!("%s"), fmtstr.as_ptr()) }
    }
    // pub fn tree_node_ptr : (*const c_void, *const c_char, ) -> bool
    #[inline]
    pub fn tree_node_ex_str(&self, label: &CStr, flags: ImGuiTreeNodeFlags) -> bool {
        unsafe { igTreeNodeExStr(label.as_ptr(), flags) }
    }
    #[inline]
    pub fn tree_node_ex_str_str(&self, str_id: &CStr, flags: ImGuiTreeNodeFlags, fmtstr: &CStr) -> bool {
        unsafe { igTreeNodeExStrStr(str_id.as_ptr(), flags, cstr_ptr!("%s"), fmtstr.as_ptr()) }
    }
    // pub fn tree_node_ex_ptr : (*const c_void, ImGuiTreeNodeFlags, *const c_char, ) -> bool
    #[inline]
    pub fn tree_pop(&self) {
        unsafe { igTreePop() };
    }
    #[inline]
    pub fn tree_push_str(&self, str_id: &CStr) {
        unsafe { igTreePushStr(str_id.as_ptr()) };
    }
    // pub fn tree_push_ptr : (*const c_void) -> undefined
    #[inline]
    pub fn unindent(&self, indent_w: f32) {
        unsafe { igUnindent(indent_w) };
    }
    #[inline]
    pub fn v_slider_float(
        &self,
        label: &CStr,
        size: ImVec2,
        v: &mut f32,
        v_min: f32,
        v_max: f32,
        format: &CStr,
        power: f32,
    ) -> bool {
        unsafe { igVSliderFloat(label.as_ptr(), size, v, v_min, v_max, format.as_ptr(), power) }
    }
    #[inline]
    pub fn v_slider_int(&self, label: &CStr, size: ImVec2, v: &mut i32, v_min: i32, v_max: i32, format: &CStr) -> bool {
        unsafe { igVSliderInt(label.as_ptr(), size, v, v_min, v_max, format.as_ptr()) }
    }
    // pub fn v_slider_scalar : (*const c_char, ImVec2, ImGuiDataType, *mut c_void, *const c_void, *const c_void, *const c_char, c_float) -> bool
    #[inline]
    pub fn value_bool(&self, prefix: &CStr, b: bool) {
        unsafe { igValueBool(prefix.as_ptr(), b) };
    }
    #[inline]
    pub fn value_int(&self, prefix: &CStr, v: i32) {
        unsafe { igValueInt(prefix.as_ptr(), v) };
    }
    #[inline]
    pub fn value_uint(&self, prefix: &CStr, v: u32) {
        unsafe { igValueUint(prefix.as_ptr(), v) };
    }
    #[inline]
    pub fn value_float(&self, prefix: &CStr, v: f32, float_format: &CStr) {
        unsafe { igValueFloat(prefix.as_ptr(), v, float_format.as_ptr()) };
    }
} // impl ImGui
pub mod types_used {
    pub use nsf_imgui_raw::ImGuiCol;
    pub use nsf_imgui_raw::ImGuiColorEditFlags;
    pub use nsf_imgui_raw::ImGuiComboFlags;
    pub use nsf_imgui_raw::ImGuiCond;
    pub use nsf_imgui_raw::ImGuiDir;
    pub use nsf_imgui_raw::ImGuiDragDropFlags;
    pub use nsf_imgui_raw::ImGuiFocusedFlags;
    pub use nsf_imgui_raw::ImGuiHoveredFlags;
    pub use nsf_imgui_raw::ImGuiID;
    pub use nsf_imgui_raw::ImGuiInputTextFlags;
    pub use nsf_imgui_raw::ImGuiKey;
    pub use nsf_imgui_raw::ImGuiMouseCursor;
    pub use nsf_imgui_raw::ImGuiSelectableFlags;
    pub use nsf_imgui_raw::ImGuiStyleVar;
    pub use nsf_imgui_raw::ImGuiTreeNodeFlags;
    pub use nsf_imgui_raw::ImGuiWindowFlags;
    pub use nsf_imgui_raw::ImTextureID;
    pub use nsf_imgui_raw::ImVec2;
    pub use nsf_imgui_raw::ImVec4;
} // mod types_used
