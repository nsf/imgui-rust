use super::{cstr_ptr, ImGui};
use nsf_imgui_raw::*;
use std::ffi::CStr;
impl ImGui {
    // pub fn accept_drag_drop_payload : (*const c_char, ImGuiDragDropFlags) -> *const ImGuiPayload
    #[inline]
    pub fn align_text_to_frame_padding<'a>(&'a self) {
        unsafe { igAlignTextToFramePadding() };
    }
    #[inline]
    pub fn arrow_button<'a, 'b>(&'a self, str_id: &'b CStr, dir: ImGuiDir) -> bool {
        unsafe { igArrowButton(str_id.as_ptr(), dir) }
    }
    #[inline]
    pub fn begin<'a, 'b, 'c>(
        &'a self,
        name: &'b CStr,
        p_open: impl Into<Option<&'c mut bool>>,
        flags: impl Into<Option<ImGuiWindowFlags>>,
    ) -> bool {
        unsafe {
            igBegin(
                name.as_ptr(),
                match p_open.into() {
                    Some(v) => v,
                    None => ::std::ptr::null_mut(),
                },
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiWindowFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn begin_child<'a, 'b>(
        &'a self,
        str_id: &'b CStr,
        size: impl Into<Option<ImVec2>>,
        border: impl Into<Option<bool>>,
        flags: impl Into<Option<ImGuiWindowFlags>>,
    ) -> bool {
        unsafe {
            igBeginChild(
                str_id.as_ptr(),
                match size.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 0.0, y: 0.0 },
                },
                match border.into() {
                    Some(v) => v,
                    None => false,
                },
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiWindowFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn begin_child_id<'a>(
        &'a self,
        id: ImGuiID,
        size: impl Into<Option<ImVec2>>,
        border: impl Into<Option<bool>>,
        flags: impl Into<Option<ImGuiWindowFlags>>,
    ) -> bool {
        unsafe {
            igBeginChildID(
                id,
                match size.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 0.0, y: 0.0 },
                },
                match border.into() {
                    Some(v) => v,
                    None => false,
                },
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiWindowFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn begin_child_frame<'a>(
        &'a self,
        id: ImGuiID,
        size: ImVec2,
        flags: impl Into<Option<ImGuiWindowFlags>>,
    ) -> bool {
        unsafe {
            igBeginChildFrame(
                id,
                size,
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiWindowFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn begin_combo<'a, 'b, 'c>(
        &'a self,
        label: &'b CStr,
        preview_value: &'c CStr,
        flags: impl Into<Option<ImGuiComboFlags>>,
    ) -> bool {
        unsafe {
            igBeginCombo(
                label.as_ptr(),
                preview_value.as_ptr(),
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiComboFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn begin_drag_drop_source<'a>(&'a self, flags: impl Into<Option<ImGuiDragDropFlags>>) -> bool {
        unsafe {
            igBeginDragDropSource(match flags.into() {
                Some(v) => v,
                None => ImGuiDragDropFlags::empty(),
            })
        }
    }
    #[inline]
    pub fn begin_drag_drop_target<'a>(&'a self) -> bool {
        unsafe { igBeginDragDropTarget() }
    }
    #[inline]
    pub fn begin_group<'a>(&'a self) {
        unsafe { igBeginGroup() };
    }
    #[inline]
    pub fn begin_main_menu_bar<'a>(&'a self) -> bool {
        unsafe { igBeginMainMenuBar() }
    }
    #[inline]
    pub fn begin_menu<'a, 'b>(&'a self, label: &'b CStr, enabled: impl Into<Option<bool>>) -> bool {
        unsafe {
            igBeginMenu(
                label.as_ptr(),
                match enabled.into() {
                    Some(v) => v,
                    None => true,
                },
            )
        }
    }
    #[inline]
    pub fn begin_menu_bar<'a>(&'a self) -> bool {
        unsafe { igBeginMenuBar() }
    }
    #[inline]
    pub fn begin_popup<'a, 'b>(&'a self, str_id: &'b CStr, flags: impl Into<Option<ImGuiWindowFlags>>) -> bool {
        unsafe {
            igBeginPopup(
                str_id.as_ptr(),
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiWindowFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn begin_popup_context_item<'a, 'b>(
        &'a self,
        str_id: impl Into<Option<&'b CStr>>,
        mouse_button: impl Into<Option<i32>>,
    ) -> bool {
        unsafe {
            igBeginPopupContextItem(
                match str_id.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
                match mouse_button.into() {
                    Some(v) => v,
                    None => 1,
                },
            )
        }
    }
    #[inline]
    pub fn begin_popup_context_void<'a, 'b>(
        &'a self,
        str_id: impl Into<Option<&'b CStr>>,
        mouse_button: impl Into<Option<i32>>,
    ) -> bool {
        unsafe {
            igBeginPopupContextVoid(
                match str_id.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
                match mouse_button.into() {
                    Some(v) => v,
                    None => 1,
                },
            )
        }
    }
    #[inline]
    pub fn begin_popup_context_window<'a, 'b>(
        &'a self,
        str_id: impl Into<Option<&'b CStr>>,
        mouse_button: impl Into<Option<i32>>,
        also_over_items: impl Into<Option<bool>>,
    ) -> bool {
        unsafe {
            igBeginPopupContextWindow(
                match str_id.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
                match mouse_button.into() {
                    Some(v) => v,
                    None => 1,
                },
                match also_over_items.into() {
                    Some(v) => v,
                    None => true,
                },
            )
        }
    }
    #[inline]
    pub fn begin_popup_modal<'a, 'b, 'c>(
        &'a self,
        name: &'b CStr,
        p_open: impl Into<Option<&'c mut bool>>,
        flags: impl Into<Option<ImGuiWindowFlags>>,
    ) -> bool {
        unsafe {
            igBeginPopupModal(
                name.as_ptr(),
                match p_open.into() {
                    Some(v) => v,
                    None => ::std::ptr::null_mut(),
                },
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiWindowFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn begin_tooltip<'a>(&'a self) {
        unsafe { igBeginTooltip() };
    }
    #[inline]
    pub fn bullet<'a>(&'a self) {
        unsafe { igBullet() };
    }
    #[inline]
    pub fn bullet_text<'a, 'b>(&'a self, fmt: &'b CStr) {
        unsafe { igBulletText(cstr_ptr!("%s"), fmt.as_ptr()) };
    }
    #[inline]
    pub fn button<'a, 'b>(&'a self, label: &'b CStr, size: impl Into<Option<ImVec2>>) -> bool {
        unsafe {
            igButton(
                label.as_ptr(),
                match size.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 0.0, y: 0.0 },
                },
            )
        }
    }
    #[inline]
    pub fn calc_item_width<'a>(&'a self) -> f32 {
        unsafe { igCalcItemWidth() }
    }
    #[inline]
    pub fn calc_list_clipping<'a, 'b, 'c>(
        &'a self,
        items_count: i32,
        items_height: f32,
        out_items_display_start: &'b mut i32,
        out_items_display_end: &'c mut i32,
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
    pub fn calc_text_size<'a, 'b, 'c>(
        &'a self,
        text: &'b CStr,
        text_end: impl Into<Option<&'c CStr>>,
        hide_text_after_double_hash: impl Into<Option<bool>>,
        wrap_width: impl Into<Option<f32>>,
    ) -> ImVec2 {
        unsafe {
            igCalcTextSize(
                text.as_ptr(),
                match text_end.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
                match hide_text_after_double_hash.into() {
                    Some(v) => v,
                    None => false,
                },
                match wrap_width.into() {
                    Some(v) => v,
                    None => -1.0,
                },
            )
        }
    }
    #[inline]
    pub fn capture_keyboard_from_app<'a>(&'a self, capture: impl Into<Option<bool>>) {
        unsafe {
            igCaptureKeyboardFromApp(match capture.into() {
                Some(v) => v,
                None => true,
            })
        };
    }
    #[inline]
    pub fn capture_mouse_from_app<'a>(&'a self, capture: impl Into<Option<bool>>) {
        unsafe {
            igCaptureMouseFromApp(match capture.into() {
                Some(v) => v,
                None => true,
            })
        };
    }
    #[inline]
    pub fn checkbox<'a, 'b, 'c>(&'a self, label: &'b CStr, v: &'c mut bool) -> bool {
        unsafe { igCheckbox(label.as_ptr(), v) }
    }
    #[inline]
    pub fn checkbox_flags<'a, 'b, 'c>(&'a self, label: &'b CStr, flags: &'c mut u32, flags_value: u32) -> bool {
        unsafe { igCheckboxFlags(label.as_ptr(), flags, flags_value) }
    }
    #[inline]
    pub fn close_current_popup<'a>(&'a self) {
        unsafe { igCloseCurrentPopup() };
    }
    #[inline]
    pub fn collapsing_header<'a, 'b>(&'a self, label: &'b CStr, flags: impl Into<Option<ImGuiTreeNodeFlags>>) -> bool {
        unsafe {
            igCollapsingHeader(
                label.as_ptr(),
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiTreeNodeFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn collapsing_header_bool_ptr<'a, 'b, 'c>(
        &'a self,
        label: &'b CStr,
        p_open: &'c mut bool,
        flags: impl Into<Option<ImGuiTreeNodeFlags>>,
    ) -> bool {
        unsafe {
            igCollapsingHeaderBoolPtr(
                label.as_ptr(),
                p_open,
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiTreeNodeFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn color_button<'a, 'b>(
        &'a self,
        desc_id: &'b CStr,
        col: ImVec4,
        flags: impl Into<Option<ImGuiColorEditFlags>>,
        size: impl Into<Option<ImVec2>>,
    ) -> bool {
        unsafe {
            igColorButton(
                desc_id.as_ptr(),
                col,
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiColorEditFlags::empty(),
                },
                match size.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 0.0, y: 0.0 },
                },
            )
        }
    }
    #[inline]
    pub fn color_convert_float4_to_u32<'a>(&'a self, _in: ImVec4) -> u32 {
        unsafe { igColorConvertFloat4ToU32(_in) }
    }
    #[inline]
    pub fn color_convert_hsv_to_rgb<'a, 'b, 'c, 'd>(
        &'a self,
        h: f32,
        s: f32,
        v: f32,
        out_r: &'b mut f32,
        out_g: &'c mut f32,
        out_b: &'d mut f32,
    ) {
        unsafe { igColorConvertHSVtoRGB(h, s, v, out_r, out_g, out_b) };
    }
    #[inline]
    pub fn color_convert_rgb_to_hsv<'a, 'b, 'c, 'd>(
        &'a self,
        r: f32,
        g: f32,
        b: f32,
        out_h: &'b mut f32,
        out_s: &'c mut f32,
        out_v: &'d mut f32,
    ) {
        unsafe { igColorConvertRGBtoHSV(r, g, b, out_h, out_s, out_v) };
    }
    #[inline]
    pub fn color_convert_u32_to_float4<'a>(&'a self, _in: u32) -> ImVec4 {
        unsafe { igColorConvertU32ToFloat4(_in) }
    }
    #[inline]
    pub fn color_edit3<'a, 'b, 'c>(
        &'a self,
        label: &'b CStr,
        col: &'c mut [f32; 3],
        flags: impl Into<Option<ImGuiColorEditFlags>>,
    ) -> bool {
        unsafe {
            igColorEdit3(
                label.as_ptr(),
                col,
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiColorEditFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn color_edit4<'a, 'b, 'c>(
        &'a self,
        label: &'b CStr,
        col: &'c mut [f32; 4],
        flags: impl Into<Option<ImGuiColorEditFlags>>,
    ) -> bool {
        unsafe {
            igColorEdit4(
                label.as_ptr(),
                col,
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiColorEditFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn color_picker3<'a, 'b, 'c>(
        &'a self,
        label: &'b CStr,
        col: &'c mut [f32; 3],
        flags: impl Into<Option<ImGuiColorEditFlags>>,
    ) -> bool {
        unsafe {
            igColorPicker3(
                label.as_ptr(),
                col,
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiColorEditFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn color_picker4<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        col: &'c mut [f32; 4],
        flags: impl Into<Option<ImGuiColorEditFlags>>,
        ref_col: impl Into<Option<&'d f32>>,
    ) -> bool {
        unsafe {
            igColorPicker4(
                label.as_ptr(),
                col,
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiColorEditFlags::empty(),
                },
                match ref_col.into() {
                    Some(v) => v,
                    None => ::std::ptr::null(),
                },
            )
        }
    }
    #[inline]
    pub fn columns<'a, 'b>(
        &'a self,
        count: impl Into<Option<i32>>,
        id: impl Into<Option<&'b CStr>>,
        border: impl Into<Option<bool>>,
    ) {
        unsafe {
            igColumns(
                match count.into() {
                    Some(v) => v,
                    None => 1,
                },
                match id.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
                match border.into() {
                    Some(v) => v,
                    None => true,
                },
            )
        };
    }
    // pub fn combo : (*const c_char, *mut c_int, *const *const c_char, c_int, c_int) -> bool
    #[inline]
    pub fn combo_str<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        current_item: &'c mut i32,
        items_separated_by_zeros: &'d CStr,
        popup_max_height_in_items: impl Into<Option<i32>>,
    ) -> bool {
        unsafe {
            igComboStr(
                label.as_ptr(),
                current_item,
                items_separated_by_zeros.as_ptr(),
                match popup_max_height_in_items.into() {
                    Some(v) => v,
                    None => -1,
                },
            )
        }
    }
    // pub fn combo_fn_ptr : (*const c_char, *mut c_int, extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char) -> bool, *mut c_void, c_int, c_int) -> bool
    #[inline]
    pub fn debug_check_version_and_data_layout<'a, 'b>(
        &'a self,
        version_str: &'b CStr,
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
    pub fn drag_float<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut f32,
        v_speed: impl Into<Option<f32>>,
        v_min: impl Into<Option<f32>>,
        v_max: impl Into<Option<f32>>,
        format: impl Into<Option<&'d CStr>>,
        power: impl Into<Option<f32>>,
    ) -> bool {
        unsafe {
            igDragFloat(
                label.as_ptr(),
                v,
                match v_speed.into() {
                    Some(v) => v,
                    None => 1.0,
                },
                match v_min.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match v_max.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match power.into() {
                    Some(v) => v,
                    None => 1.0,
                },
            )
        }
    }
    #[inline]
    pub fn drag_float2<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [f32; 2],
        v_speed: impl Into<Option<f32>>,
        v_min: impl Into<Option<f32>>,
        v_max: impl Into<Option<f32>>,
        format: impl Into<Option<&'d CStr>>,
        power: impl Into<Option<f32>>,
    ) -> bool {
        unsafe {
            igDragFloat2(
                label.as_ptr(),
                v,
                match v_speed.into() {
                    Some(v) => v,
                    None => 1.0,
                },
                match v_min.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match v_max.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match power.into() {
                    Some(v) => v,
                    None => 1.0,
                },
            )
        }
    }
    #[inline]
    pub fn drag_float3<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [f32; 3],
        v_speed: impl Into<Option<f32>>,
        v_min: impl Into<Option<f32>>,
        v_max: impl Into<Option<f32>>,
        format: impl Into<Option<&'d CStr>>,
        power: impl Into<Option<f32>>,
    ) -> bool {
        unsafe {
            igDragFloat3(
                label.as_ptr(),
                v,
                match v_speed.into() {
                    Some(v) => v,
                    None => 1.0,
                },
                match v_min.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match v_max.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match power.into() {
                    Some(v) => v,
                    None => 1.0,
                },
            )
        }
    }
    #[inline]
    pub fn drag_float4<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [f32; 4],
        v_speed: impl Into<Option<f32>>,
        v_min: impl Into<Option<f32>>,
        v_max: impl Into<Option<f32>>,
        format: impl Into<Option<&'d CStr>>,
        power: impl Into<Option<f32>>,
    ) -> bool {
        unsafe {
            igDragFloat4(
                label.as_ptr(),
                v,
                match v_speed.into() {
                    Some(v) => v,
                    None => 1.0,
                },
                match v_min.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match v_max.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match power.into() {
                    Some(v) => v,
                    None => 1.0,
                },
            )
        }
    }
    #[inline]
    pub fn drag_float_range2<'a, 'b, 'c, 'd, 'e, 'f>(
        &'a self,
        label: &'b CStr,
        v_current_min: &'c mut f32,
        v_current_max: &'d mut f32,
        v_speed: impl Into<Option<f32>>,
        v_min: impl Into<Option<f32>>,
        v_max: impl Into<Option<f32>>,
        format: impl Into<Option<&'e CStr>>,
        format_max: impl Into<Option<&'f CStr>>,
        power: impl Into<Option<f32>>,
    ) -> bool {
        unsafe {
            igDragFloatRange2(
                label.as_ptr(),
                v_current_min,
                v_current_max,
                match v_speed.into() {
                    Some(v) => v,
                    None => 1.0,
                },
                match v_min.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match v_max.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match format_max.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
                match power.into() {
                    Some(v) => v,
                    None => 1.0,
                },
            )
        }
    }
    #[inline]
    pub fn drag_int<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut i32,
        v_speed: impl Into<Option<f32>>,
        v_min: impl Into<Option<i32>>,
        v_max: impl Into<Option<i32>>,
        format: impl Into<Option<&'d CStr>>,
    ) -> bool {
        unsafe {
            igDragInt(
                label.as_ptr(),
                v,
                match v_speed.into() {
                    Some(v) => v,
                    None => 1.0,
                },
                match v_min.into() {
                    Some(v) => v,
                    None => 0,
                },
                match v_max.into() {
                    Some(v) => v,
                    None => 0,
                },
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%d"),
                },
            )
        }
    }
    #[inline]
    pub fn drag_int2<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [i32; 2],
        v_speed: impl Into<Option<f32>>,
        v_min: impl Into<Option<i32>>,
        v_max: impl Into<Option<i32>>,
        format: impl Into<Option<&'d CStr>>,
    ) -> bool {
        unsafe {
            igDragInt2(
                label.as_ptr(),
                v,
                match v_speed.into() {
                    Some(v) => v,
                    None => 1.0,
                },
                match v_min.into() {
                    Some(v) => v,
                    None => 0,
                },
                match v_max.into() {
                    Some(v) => v,
                    None => 0,
                },
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%d"),
                },
            )
        }
    }
    #[inline]
    pub fn drag_int3<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [i32; 3],
        v_speed: impl Into<Option<f32>>,
        v_min: impl Into<Option<i32>>,
        v_max: impl Into<Option<i32>>,
        format: impl Into<Option<&'d CStr>>,
    ) -> bool {
        unsafe {
            igDragInt3(
                label.as_ptr(),
                v,
                match v_speed.into() {
                    Some(v) => v,
                    None => 1.0,
                },
                match v_min.into() {
                    Some(v) => v,
                    None => 0,
                },
                match v_max.into() {
                    Some(v) => v,
                    None => 0,
                },
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%d"),
                },
            )
        }
    }
    #[inline]
    pub fn drag_int4<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [i32; 4],
        v_speed: impl Into<Option<f32>>,
        v_min: impl Into<Option<i32>>,
        v_max: impl Into<Option<i32>>,
        format: impl Into<Option<&'d CStr>>,
    ) -> bool {
        unsafe {
            igDragInt4(
                label.as_ptr(),
                v,
                match v_speed.into() {
                    Some(v) => v,
                    None => 1.0,
                },
                match v_min.into() {
                    Some(v) => v,
                    None => 0,
                },
                match v_max.into() {
                    Some(v) => v,
                    None => 0,
                },
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%d"),
                },
            )
        }
    }
    #[inline]
    pub fn drag_int_range2<'a, 'b, 'c, 'd, 'e, 'f>(
        &'a self,
        label: &'b CStr,
        v_current_min: &'c mut i32,
        v_current_max: &'d mut i32,
        v_speed: impl Into<Option<f32>>,
        v_min: impl Into<Option<i32>>,
        v_max: impl Into<Option<i32>>,
        format: impl Into<Option<&'e CStr>>,
        format_max: impl Into<Option<&'f CStr>>,
    ) -> bool {
        unsafe {
            igDragIntRange2(
                label.as_ptr(),
                v_current_min,
                v_current_max,
                match v_speed.into() {
                    Some(v) => v,
                    None => 1.0,
                },
                match v_min.into() {
                    Some(v) => v,
                    None => 0,
                },
                match v_max.into() {
                    Some(v) => v,
                    None => 0,
                },
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%d"),
                },
                match format_max.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
            )
        }
    }
    // pub fn drag_scalar : (*const c_char, ImGuiDataType, *mut c_void, c_float, *const c_void, *const c_void, *const c_char, c_float) -> bool
    // pub fn drag_scalar_n : (*const c_char, ImGuiDataType, *mut c_void, c_int, c_float, *const c_void, *const c_void, *const c_char, c_float) -> bool
    #[inline]
    pub fn dummy<'a>(&'a self, size: ImVec2) {
        unsafe { igDummy(size) };
    }
    #[inline]
    pub fn end<'a>(&'a self) {
        unsafe { igEnd() };
    }
    #[inline]
    pub fn end_child<'a>(&'a self) {
        unsafe { igEndChild() };
    }
    #[inline]
    pub fn end_child_frame<'a>(&'a self) {
        unsafe { igEndChildFrame() };
    }
    #[inline]
    pub fn end_combo<'a>(&'a self) {
        unsafe { igEndCombo() };
    }
    #[inline]
    pub fn end_drag_drop_source<'a>(&'a self) {
        unsafe { igEndDragDropSource() };
    }
    #[inline]
    pub fn end_drag_drop_target<'a>(&'a self) {
        unsafe { igEndDragDropTarget() };
    }
    #[inline]
    pub fn end_frame<'a>(&'a self) {
        unsafe { igEndFrame() };
    }
    #[inline]
    pub fn end_group<'a>(&'a self) {
        unsafe { igEndGroup() };
    }
    #[inline]
    pub fn end_main_menu_bar<'a>(&'a self) {
        unsafe { igEndMainMenuBar() };
    }
    #[inline]
    pub fn end_menu<'a>(&'a self) {
        unsafe { igEndMenu() };
    }
    #[inline]
    pub fn end_menu_bar<'a>(&'a self) {
        unsafe { igEndMenuBar() };
    }
    #[inline]
    pub fn end_popup<'a>(&'a self) {
        unsafe { igEndPopup() };
    }
    #[inline]
    pub fn end_tooltip<'a>(&'a self) {
        unsafe { igEndTooltip() };
    }
    #[inline]
    pub fn get_clipboard_text<'a>(&'a self) -> String {
        unsafe { CStr::from_ptr(igGetClipboardText()).to_string_lossy().into_owned() }
    }
    #[inline]
    pub fn get_color_u32<'a>(&'a self, idx: ImGuiCol, alpha_mul: impl Into<Option<f32>>) -> u32 {
        unsafe {
            igGetColorU32(
                idx,
                match alpha_mul.into() {
                    Some(v) => v,
                    None => 1.0,
                },
            )
        }
    }
    #[inline]
    pub fn get_color_u32_vec4<'a>(&'a self, col: ImVec4) -> u32 {
        unsafe { igGetColorU32Vec4(col) }
    }
    #[inline]
    pub fn get_color_u32_u32<'a>(&'a self, col: u32) -> u32 {
        unsafe { igGetColorU32U32(col) }
    }
    #[inline]
    pub fn get_column_index<'a>(&'a self) -> i32 {
        unsafe { igGetColumnIndex() }
    }
    #[inline]
    pub fn get_column_offset<'a>(&'a self, column_index: impl Into<Option<i32>>) -> f32 {
        unsafe {
            igGetColumnOffset(match column_index.into() {
                Some(v) => v,
                None => -1,
            })
        }
    }
    #[inline]
    pub fn get_column_width<'a>(&'a self, column_index: impl Into<Option<i32>>) -> f32 {
        unsafe {
            igGetColumnWidth(match column_index.into() {
                Some(v) => v,
                None => -1,
            })
        }
    }
    #[inline]
    pub fn get_columns_count<'a>(&'a self) -> i32 {
        unsafe { igGetColumnsCount() }
    }
    #[inline]
    pub fn get_content_region_avail<'a>(&'a self) -> ImVec2 {
        unsafe { igGetContentRegionAvail() }
    }
    #[inline]
    pub fn get_content_region_avail_width<'a>(&'a self) -> f32 {
        unsafe { igGetContentRegionAvailWidth() }
    }
    #[inline]
    pub fn get_content_region_max<'a>(&'a self) -> ImVec2 {
        unsafe { igGetContentRegionMax() }
    }
    // pub fn get_current_context : () -> *mut ImGuiContext
    #[inline]
    pub fn get_cursor_pos<'a>(&'a self) -> ImVec2 {
        unsafe { igGetCursorPos() }
    }
    #[inline]
    pub fn get_cursor_pos_x<'a>(&'a self) -> f32 {
        unsafe { igGetCursorPosX() }
    }
    #[inline]
    pub fn get_cursor_pos_y<'a>(&'a self) -> f32 {
        unsafe { igGetCursorPosY() }
    }
    #[inline]
    pub fn get_cursor_screen_pos<'a>(&'a self) -> ImVec2 {
        unsafe { igGetCursorScreenPos() }
    }
    #[inline]
    pub fn get_cursor_start_pos<'a>(&'a self) -> ImVec2 {
        unsafe { igGetCursorStartPos() }
    }
    // pub fn get_drag_drop_payload : () -> *const ImGuiPayload
    // pub fn get_draw_data : () -> *mut ImDrawData
    // pub fn get_draw_list_shared_data : () -> *mut ImDrawListSharedData
    // pub fn get_font : () -> *mut ImFont
    #[inline]
    pub fn get_font_size<'a>(&'a self) -> f32 {
        unsafe { igGetFontSize() }
    }
    #[inline]
    pub fn get_font_tex_uv_white_pixel<'a>(&'a self) -> ImVec2 {
        unsafe { igGetFontTexUvWhitePixel() }
    }
    #[inline]
    pub fn get_frame_count<'a>(&'a self) -> i32 {
        unsafe { igGetFrameCount() }
    }
    #[inline]
    pub fn get_frame_height<'a>(&'a self) -> f32 {
        unsafe { igGetFrameHeight() }
    }
    #[inline]
    pub fn get_frame_height_with_spacing<'a>(&'a self) -> f32 {
        unsafe { igGetFrameHeightWithSpacing() }
    }
    #[inline]
    pub fn get_id_str<'a, 'b>(&'a self, str_id: &'b CStr) -> ImGuiID {
        unsafe { igGetIDStr(str_id.as_ptr()) }
    }
    #[inline]
    pub fn get_id_range<'a, 'b, 'c>(&'a self, str_id_begin: &'b CStr, str_id_end: &'c CStr) -> ImGuiID {
        unsafe { igGetIDRange(str_id_begin.as_ptr(), str_id_end.as_ptr()) }
    }
    // pub fn get_id_ptr : (*const c_void) -> ImGuiID
    // pub fn get_io : () -> *mut ImGuiIO
    #[inline]
    pub fn get_item_rect_max<'a>(&'a self) -> ImVec2 {
        unsafe { igGetItemRectMax() }
    }
    #[inline]
    pub fn get_item_rect_min<'a>(&'a self) -> ImVec2 {
        unsafe { igGetItemRectMin() }
    }
    #[inline]
    pub fn get_item_rect_size<'a>(&'a self) -> ImVec2 {
        unsafe { igGetItemRectSize() }
    }
    #[inline]
    pub fn get_key_index<'a>(&'a self, imgui_key: ImGuiKey) -> i32 {
        unsafe { igGetKeyIndex(imgui_key) }
    }
    #[inline]
    pub fn get_key_pressed_amount<'a>(&'a self, key_index: i32, repeat_delay: f32, rate: f32) -> i32 {
        unsafe { igGetKeyPressedAmount(key_index, repeat_delay, rate) }
    }
    #[inline]
    pub fn get_mouse_cursor<'a>(&'a self) -> ImGuiMouseCursor {
        unsafe { igGetMouseCursor() }
    }
    #[inline]
    pub fn get_mouse_drag_delta<'a>(
        &'a self,
        button: impl Into<Option<i32>>,
        lock_threshold: impl Into<Option<f32>>,
    ) -> ImVec2 {
        unsafe {
            igGetMouseDragDelta(
                match button.into() {
                    Some(v) => v,
                    None => 0,
                },
                match lock_threshold.into() {
                    Some(v) => v,
                    None => -1.0,
                },
            )
        }
    }
    #[inline]
    pub fn get_mouse_pos<'a>(&'a self) -> ImVec2 {
        unsafe { igGetMousePos() }
    }
    #[inline]
    pub fn get_mouse_pos_on_opening_current_popup<'a>(&'a self) -> ImVec2 {
        unsafe { igGetMousePosOnOpeningCurrentPopup() }
    }
    // pub fn get_overlay_draw_list : () -> *mut ImDrawList
    #[inline]
    pub fn get_scroll_max_x<'a>(&'a self) -> f32 {
        unsafe { igGetScrollMaxX() }
    }
    #[inline]
    pub fn get_scroll_max_y<'a>(&'a self) -> f32 {
        unsafe { igGetScrollMaxY() }
    }
    #[inline]
    pub fn get_scroll_x<'a>(&'a self) -> f32 {
        unsafe { igGetScrollX() }
    }
    #[inline]
    pub fn get_scroll_y<'a>(&'a self) -> f32 {
        unsafe { igGetScrollY() }
    }
    // pub fn get_state_storage : () -> *mut ImGuiStorage
    // pub fn get_style : () -> *mut ImGuiStyle
    #[inline]
    pub fn get_style_color_name<'a>(&'a self, idx: ImGuiCol) -> String {
        unsafe { CStr::from_ptr(igGetStyleColorName(idx)).to_string_lossy().into_owned() }
    }
    // pub fn get_style_color_vec4 : (ImGuiCol) -> *const ImVec4
    #[inline]
    pub fn get_text_line_height<'a>(&'a self) -> f32 {
        unsafe { igGetTextLineHeight() }
    }
    #[inline]
    pub fn get_text_line_height_with_spacing<'a>(&'a self) -> f32 {
        unsafe { igGetTextLineHeightWithSpacing() }
    }
    #[inline]
    pub fn get_time<'a>(&'a self) -> f64 {
        unsafe { igGetTime() }
    }
    #[inline]
    pub fn get_tree_node_to_label_spacing<'a>(&'a self) -> f32 {
        unsafe { igGetTreeNodeToLabelSpacing() }
    }
    #[inline]
    pub fn get_version<'a>(&'a self) -> String {
        unsafe { CStr::from_ptr(igGetVersion()).to_string_lossy().into_owned() }
    }
    #[inline]
    pub fn get_window_content_region_max<'a>(&'a self) -> ImVec2 {
        unsafe { igGetWindowContentRegionMax() }
    }
    #[inline]
    pub fn get_window_content_region_min<'a>(&'a self) -> ImVec2 {
        unsafe { igGetWindowContentRegionMin() }
    }
    #[inline]
    pub fn get_window_content_region_width<'a>(&'a self) -> f32 {
        unsafe { igGetWindowContentRegionWidth() }
    }
    // pub fn get_window_draw_list : () -> *mut ImDrawList
    #[inline]
    pub fn get_window_height<'a>(&'a self) -> f32 {
        unsafe { igGetWindowHeight() }
    }
    #[inline]
    pub fn get_window_pos<'a>(&'a self) -> ImVec2 {
        unsafe { igGetWindowPos() }
    }
    #[inline]
    pub fn get_window_size<'a>(&'a self) -> ImVec2 {
        unsafe { igGetWindowSize() }
    }
    #[inline]
    pub fn get_window_width<'a>(&'a self) -> f32 {
        unsafe { igGetWindowWidth() }
    }
    #[inline]
    pub fn image<'a>(
        &'a self,
        user_texture_id: ImTextureID,
        size: ImVec2,
        uv0: impl Into<Option<ImVec2>>,
        uv1: impl Into<Option<ImVec2>>,
        tint_col: impl Into<Option<ImVec4>>,
        border_col: impl Into<Option<ImVec4>>,
    ) {
        unsafe {
            igImage(
                user_texture_id,
                size,
                match uv0.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 0.0, y: 0.0 },
                },
                match uv1.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 1.0, y: 1.0 },
                },
                match tint_col.into() {
                    Some(v) => v,
                    None => ImVec4 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                        w: 1.0,
                    },
                },
                match border_col.into() {
                    Some(v) => v,
                    None => ImVec4 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                },
            )
        };
    }
    #[inline]
    pub fn image_button<'a>(
        &'a self,
        user_texture_id: ImTextureID,
        size: ImVec2,
        uv0: impl Into<Option<ImVec2>>,
        uv1: impl Into<Option<ImVec2>>,
        frame_padding: impl Into<Option<i32>>,
        bg_col: impl Into<Option<ImVec4>>,
        tint_col: impl Into<Option<ImVec4>>,
    ) -> bool {
        unsafe {
            igImageButton(
                user_texture_id,
                size,
                match uv0.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 0.0, y: 0.0 },
                },
                match uv1.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 1.0, y: 1.0 },
                },
                match frame_padding.into() {
                    Some(v) => v,
                    None => -1,
                },
                match bg_col.into() {
                    Some(v) => v,
                    None => ImVec4 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        w: 0.0,
                    },
                },
                match tint_col.into() {
                    Some(v) => v,
                    None => ImVec4 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                        w: 1.0,
                    },
                },
            )
        }
    }
    #[inline]
    pub fn indent<'a>(&'a self, indent_w: impl Into<Option<f32>>) {
        unsafe {
            igIndent(match indent_w.into() {
                Some(v) => v,
                None => 0.0,
            })
        };
    }
    #[inline]
    pub fn input_double<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut f64,
        step: impl Into<Option<f64>>,
        step_fast: impl Into<Option<f64>>,
        format: impl Into<Option<&'d CStr>>,
        extra_flags: impl Into<Option<ImGuiInputTextFlags>>,
    ) -> bool {
        unsafe {
            igInputDouble(
                label.as_ptr(),
                v,
                match step.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match step_fast.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.6f"),
                },
                match extra_flags.into() {
                    Some(v) => v,
                    None => ImGuiInputTextFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn input_float<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut f32,
        step: impl Into<Option<f32>>,
        step_fast: impl Into<Option<f32>>,
        format: impl Into<Option<&'d CStr>>,
        extra_flags: impl Into<Option<ImGuiInputTextFlags>>,
    ) -> bool {
        unsafe {
            igInputFloat(
                label.as_ptr(),
                v,
                match step.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match step_fast.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match extra_flags.into() {
                    Some(v) => v,
                    None => ImGuiInputTextFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn input_float2<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [f32; 2],
        format: impl Into<Option<&'d CStr>>,
        extra_flags: impl Into<Option<ImGuiInputTextFlags>>,
    ) -> bool {
        unsafe {
            igInputFloat2(
                label.as_ptr(),
                v,
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match extra_flags.into() {
                    Some(v) => v,
                    None => ImGuiInputTextFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn input_float3<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [f32; 3],
        format: impl Into<Option<&'d CStr>>,
        extra_flags: impl Into<Option<ImGuiInputTextFlags>>,
    ) -> bool {
        unsafe {
            igInputFloat3(
                label.as_ptr(),
                v,
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match extra_flags.into() {
                    Some(v) => v,
                    None => ImGuiInputTextFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn input_float4<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [f32; 4],
        format: impl Into<Option<&'d CStr>>,
        extra_flags: impl Into<Option<ImGuiInputTextFlags>>,
    ) -> bool {
        unsafe {
            igInputFloat4(
                label.as_ptr(),
                v,
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match extra_flags.into() {
                    Some(v) => v,
                    None => ImGuiInputTextFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn input_int<'a, 'b, 'c>(
        &'a self,
        label: &'b CStr,
        v: &'c mut i32,
        step: impl Into<Option<i32>>,
        step_fast: impl Into<Option<i32>>,
        extra_flags: impl Into<Option<ImGuiInputTextFlags>>,
    ) -> bool {
        unsafe {
            igInputInt(
                label.as_ptr(),
                v,
                match step.into() {
                    Some(v) => v,
                    None => 1,
                },
                match step_fast.into() {
                    Some(v) => v,
                    None => 100,
                },
                match extra_flags.into() {
                    Some(v) => v,
                    None => ImGuiInputTextFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn input_int2<'a, 'b, 'c>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [i32; 2],
        extra_flags: impl Into<Option<ImGuiInputTextFlags>>,
    ) -> bool {
        unsafe {
            igInputInt2(
                label.as_ptr(),
                v,
                match extra_flags.into() {
                    Some(v) => v,
                    None => ImGuiInputTextFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn input_int3<'a, 'b, 'c>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [i32; 3],
        extra_flags: impl Into<Option<ImGuiInputTextFlags>>,
    ) -> bool {
        unsafe {
            igInputInt3(
                label.as_ptr(),
                v,
                match extra_flags.into() {
                    Some(v) => v,
                    None => ImGuiInputTextFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn input_int4<'a, 'b, 'c>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [i32; 4],
        extra_flags: impl Into<Option<ImGuiInputTextFlags>>,
    ) -> bool {
        unsafe {
            igInputInt4(
                label.as_ptr(),
                v,
                match extra_flags.into() {
                    Some(v) => v,
                    None => ImGuiInputTextFlags::empty(),
                },
            )
        }
    }
    // pub fn input_scalar : (*const c_char, ImGuiDataType, *mut c_void, *const c_void, *const c_void, *const c_char, ImGuiInputTextFlags) -> bool
    // pub fn input_scalar_n : (*const c_char, ImGuiDataType, *mut c_void, c_int, *const c_void, *const c_void, *const c_char, ImGuiInputTextFlags) -> bool
    // pub fn input_text : (*const c_char, *mut c_char, size_t, ImGuiInputTextFlags, ImGuiInputTextCallback, *mut c_void) -> bool
    // pub fn input_text_multiline : (*const c_char, *mut c_char, size_t, ImVec2, ImGuiInputTextFlags, ImGuiInputTextCallback, *mut c_void) -> bool
    #[inline]
    pub fn invisible_button<'a, 'b>(&'a self, str_id: &'b CStr, size: ImVec2) -> bool {
        unsafe { igInvisibleButton(str_id.as_ptr(), size) }
    }
    #[inline]
    pub fn is_any_item_active<'a>(&'a self) -> bool {
        unsafe { igIsAnyItemActive() }
    }
    #[inline]
    pub fn is_any_item_focused<'a>(&'a self) -> bool {
        unsafe { igIsAnyItemFocused() }
    }
    #[inline]
    pub fn is_any_item_hovered<'a>(&'a self) -> bool {
        unsafe { igIsAnyItemHovered() }
    }
    #[inline]
    pub fn is_any_mouse_down<'a>(&'a self) -> bool {
        unsafe { igIsAnyMouseDown() }
    }
    #[inline]
    pub fn is_item_active<'a>(&'a self) -> bool {
        unsafe { igIsItemActive() }
    }
    #[inline]
    pub fn is_item_clicked<'a>(&'a self, mouse_button: impl Into<Option<i32>>) -> bool {
        unsafe {
            igIsItemClicked(match mouse_button.into() {
                Some(v) => v,
                None => 0,
            })
        }
    }
    #[inline]
    pub fn is_item_deactivated<'a>(&'a self) -> bool {
        unsafe { igIsItemDeactivated() }
    }
    #[inline]
    pub fn is_item_deactivated_after_edit<'a>(&'a self) -> bool {
        unsafe { igIsItemDeactivatedAfterEdit() }
    }
    #[inline]
    pub fn is_item_edited<'a>(&'a self) -> bool {
        unsafe { igIsItemEdited() }
    }
    #[inline]
    pub fn is_item_focused<'a>(&'a self) -> bool {
        unsafe { igIsItemFocused() }
    }
    #[inline]
    pub fn is_item_hovered<'a>(&'a self, flags: impl Into<Option<ImGuiHoveredFlags>>) -> bool {
        unsafe {
            igIsItemHovered(match flags.into() {
                Some(v) => v,
                None => ImGuiHoveredFlags::empty(),
            })
        }
    }
    #[inline]
    pub fn is_item_visible<'a>(&'a self) -> bool {
        unsafe { igIsItemVisible() }
    }
    #[inline]
    pub fn is_key_down<'a>(&'a self, user_key_index: i32) -> bool {
        unsafe { igIsKeyDown(user_key_index) }
    }
    #[inline]
    pub fn is_key_pressed<'a>(&'a self, user_key_index: i32, repeat: impl Into<Option<bool>>) -> bool {
        unsafe {
            igIsKeyPressed(
                user_key_index,
                match repeat.into() {
                    Some(v) => v,
                    None => true,
                },
            )
        }
    }
    #[inline]
    pub fn is_key_released<'a>(&'a self, user_key_index: i32) -> bool {
        unsafe { igIsKeyReleased(user_key_index) }
    }
    #[inline]
    pub fn is_mouse_clicked<'a>(&'a self, button: i32, repeat: impl Into<Option<bool>>) -> bool {
        unsafe {
            igIsMouseClicked(
                button,
                match repeat.into() {
                    Some(v) => v,
                    None => false,
                },
            )
        }
    }
    #[inline]
    pub fn is_mouse_double_clicked<'a>(&'a self, button: i32) -> bool {
        unsafe { igIsMouseDoubleClicked(button) }
    }
    #[inline]
    pub fn is_mouse_down<'a>(&'a self, button: i32) -> bool {
        unsafe { igIsMouseDown(button) }
    }
    #[inline]
    pub fn is_mouse_dragging<'a>(
        &'a self,
        button: impl Into<Option<i32>>,
        lock_threshold: impl Into<Option<f32>>,
    ) -> bool {
        unsafe {
            igIsMouseDragging(
                match button.into() {
                    Some(v) => v,
                    None => 0,
                },
                match lock_threshold.into() {
                    Some(v) => v,
                    None => -1.0,
                },
            )
        }
    }
    #[inline]
    pub fn is_mouse_hovering_rect<'a>(&'a self, r_min: ImVec2, r_max: ImVec2, clip: impl Into<Option<bool>>) -> bool {
        unsafe {
            igIsMouseHoveringRect(
                r_min,
                r_max,
                match clip.into() {
                    Some(v) => v,
                    None => true,
                },
            )
        }
    }
    #[inline]
    pub fn is_mouse_pos_valid<'a, 'b>(&'a self, mouse_pos: impl Into<Option<&'b ImVec2>>) -> bool {
        unsafe {
            igIsMousePosValid(match mouse_pos.into() {
                Some(v) => v,
                None => ::std::ptr::null(),
            })
        }
    }
    #[inline]
    pub fn is_mouse_released<'a>(&'a self, button: i32) -> bool {
        unsafe { igIsMouseReleased(button) }
    }
    #[inline]
    pub fn is_popup_open<'a, 'b>(&'a self, str_id: &'b CStr) -> bool {
        unsafe { igIsPopupOpen(str_id.as_ptr()) }
    }
    #[inline]
    pub fn is_rect_visible<'a>(&'a self, size: ImVec2) -> bool {
        unsafe { igIsRectVisible(size) }
    }
    #[inline]
    pub fn is_rect_visible_vec2<'a>(&'a self, rect_min: ImVec2, rect_max: ImVec2) -> bool {
        unsafe { igIsRectVisibleVec2(rect_min, rect_max) }
    }
    #[inline]
    pub fn is_window_appearing<'a>(&'a self) -> bool {
        unsafe { igIsWindowAppearing() }
    }
    #[inline]
    pub fn is_window_collapsed<'a>(&'a self) -> bool {
        unsafe { igIsWindowCollapsed() }
    }
    #[inline]
    pub fn is_window_focused<'a>(&'a self, flags: impl Into<Option<ImGuiFocusedFlags>>) -> bool {
        unsafe {
            igIsWindowFocused(match flags.into() {
                Some(v) => v,
                None => ImGuiFocusedFlags::empty(),
            })
        }
    }
    #[inline]
    pub fn is_window_hovered<'a>(&'a self, flags: impl Into<Option<ImGuiHoveredFlags>>) -> bool {
        unsafe {
            igIsWindowHovered(match flags.into() {
                Some(v) => v,
                None => ImGuiHoveredFlags::empty(),
            })
        }
    }
    #[inline]
    pub fn label_text<'a, 'b, 'c>(&'a self, label: &'b CStr, fmt: &'c CStr) {
        unsafe { igLabelText(label.as_ptr(), cstr_ptr!("%s"), fmt.as_ptr()) };
    }
    // pub fn list_box_str_arr : (*const c_char, *mut c_int, *const *const c_char, c_int, c_int) -> bool
    // pub fn list_box_fn_ptr : (*const c_char, *mut c_int, extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char) -> bool, *mut c_void, c_int, c_int) -> bool
    #[inline]
    pub fn list_box_footer<'a>(&'a self) {
        unsafe { igListBoxFooter() };
    }
    #[inline]
    pub fn list_box_header_vec2<'a, 'b>(&'a self, label: &'b CStr, size: impl Into<Option<ImVec2>>) -> bool {
        unsafe {
            igListBoxHeaderVec2(
                label.as_ptr(),
                match size.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 0.0, y: 0.0 },
                },
            )
        }
    }
    #[inline]
    pub fn list_box_header_int<'a, 'b>(
        &'a self,
        label: &'b CStr,
        items_count: i32,
        height_in_items: impl Into<Option<i32>>,
    ) -> bool {
        unsafe {
            igListBoxHeaderInt(
                label.as_ptr(),
                items_count,
                match height_in_items.into() {
                    Some(v) => v,
                    None => -1,
                },
            )
        }
    }
    #[inline]
    pub fn load_ini_settings_from_disk<'a, 'b>(&'a self, ini_filename: &'b CStr) {
        unsafe { igLoadIniSettingsFromDisk(ini_filename.as_ptr()) };
    }
    #[inline]
    pub fn load_ini_settings_from_memory<'a, 'b>(&'a self, ini_data: &'b CStr, ini_size: impl Into<Option<usize>>) {
        unsafe {
            igLoadIniSettingsFromMemory(
                ini_data.as_ptr(),
                match ini_size.into() {
                    Some(v) => v,
                    None => 0,
                },
            )
        };
    }
    #[inline]
    pub fn log_buttons<'a>(&'a self) {
        unsafe { igLogButtons() };
    }
    #[inline]
    pub fn log_finish<'a>(&'a self) {
        unsafe { igLogFinish() };
    }
    #[inline]
    pub fn log_text<'a, 'b>(&'a self, fmt: &'b CStr) {
        unsafe { igLogText(cstr_ptr!("%s"), fmt.as_ptr()) };
    }
    #[inline]
    pub fn log_to_clipboard<'a>(&'a self, max_depth: impl Into<Option<i32>>) {
        unsafe {
            igLogToClipboard(match max_depth.into() {
                Some(v) => v,
                None => -1,
            })
        };
    }
    #[inline]
    pub fn log_to_file<'a, 'b>(&'a self, max_depth: impl Into<Option<i32>>, filename: impl Into<Option<&'b CStr>>) {
        unsafe {
            igLogToFile(
                match max_depth.into() {
                    Some(v) => v,
                    None => -1,
                },
                match filename.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
            )
        };
    }
    #[inline]
    pub fn log_to_tty<'a>(&'a self, max_depth: impl Into<Option<i32>>) {
        unsafe {
            igLogToTTY(match max_depth.into() {
                Some(v) => v,
                None => -1,
            })
        };
    }
    #[inline]
    pub fn menu_item_bool<'a, 'b, 'c>(
        &'a self,
        label: &'b CStr,
        shortcut: impl Into<Option<&'c CStr>>,
        selected: impl Into<Option<bool>>,
        enabled: impl Into<Option<bool>>,
    ) -> bool {
        unsafe {
            igMenuItemBool(
                label.as_ptr(),
                match shortcut.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
                match selected.into() {
                    Some(v) => v,
                    None => false,
                },
                match enabled.into() {
                    Some(v) => v,
                    None => true,
                },
            )
        }
    }
    #[inline]
    pub fn menu_item_bool_ptr<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        shortcut: &'c CStr,
        p_selected: &'d mut bool,
        enabled: impl Into<Option<bool>>,
    ) -> bool {
        unsafe {
            igMenuItemBoolPtr(
                label.as_ptr(),
                shortcut.as_ptr(),
                p_selected,
                match enabled.into() {
                    Some(v) => v,
                    None => true,
                },
            )
        }
    }
    #[inline]
    pub fn new_frame<'a>(&'a self) {
        unsafe { igNewFrame() };
    }
    #[inline]
    pub fn new_line<'a>(&'a self) {
        unsafe { igNewLine() };
    }
    #[inline]
    pub fn next_column<'a>(&'a self) {
        unsafe { igNextColumn() };
    }
    #[inline]
    pub fn open_popup<'a, 'b>(&'a self, str_id: &'b CStr) {
        unsafe { igOpenPopup(str_id.as_ptr()) };
    }
    #[inline]
    pub fn open_popup_on_item_click<'a, 'b>(
        &'a self,
        str_id: impl Into<Option<&'b CStr>>,
        mouse_button: impl Into<Option<i32>>,
    ) -> bool {
        unsafe {
            igOpenPopupOnItemClick(
                match str_id.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
                match mouse_button.into() {
                    Some(v) => v,
                    None => 1,
                },
            )
        }
    }
    #[inline]
    pub fn plot_histogram_float_ptr<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        values: &'c f32,
        values_count: i32,
        values_offset: impl Into<Option<i32>>,
        overlay_text: impl Into<Option<&'d CStr>>,
        scale_min: impl Into<Option<f32>>,
        scale_max: impl Into<Option<f32>>,
        graph_size: impl Into<Option<ImVec2>>,
        stride: impl Into<Option<i32>>,
    ) {
        unsafe {
            igPlotHistogramFloatPtr(
                label.as_ptr(),
                values,
                values_count,
                match values_offset.into() {
                    Some(v) => v,
                    None => 0,
                },
                match overlay_text.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
                match scale_min.into() {
                    Some(v) => v,
                    None => ::std::f32::MAX,
                },
                match scale_max.into() {
                    Some(v) => v,
                    None => ::std::f32::MAX,
                },
                match graph_size.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 0.0, y: 0.0 },
                },
                match stride.into() {
                    Some(v) => v,
                    None => ::std::mem::size_of::<f32>() as i32,
                },
            )
        };
    }
    // pub fn plot_histogram_fn_ptr : (*const c_char, extern "C" fn(data: *mut c_void, idx: c_int) -> c_float, *mut c_void, c_int, c_int, *const c_char, c_float, c_float, ImVec2) -> undefined
    #[inline]
    pub fn plot_lines<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        values: &'c f32,
        values_count: i32,
        values_offset: impl Into<Option<i32>>,
        overlay_text: impl Into<Option<&'d CStr>>,
        scale_min: impl Into<Option<f32>>,
        scale_max: impl Into<Option<f32>>,
        graph_size: impl Into<Option<ImVec2>>,
        stride: impl Into<Option<i32>>,
    ) {
        unsafe {
            igPlotLines(
                label.as_ptr(),
                values,
                values_count,
                match values_offset.into() {
                    Some(v) => v,
                    None => 0,
                },
                match overlay_text.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
                match scale_min.into() {
                    Some(v) => v,
                    None => ::std::f32::MAX,
                },
                match scale_max.into() {
                    Some(v) => v,
                    None => ::std::f32::MAX,
                },
                match graph_size.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 0.0, y: 0.0 },
                },
                match stride.into() {
                    Some(v) => v,
                    None => ::std::mem::size_of::<f32>() as i32,
                },
            )
        };
    }
    // pub fn plot_lines_fn_ptr : (*const c_char, extern "C" fn(data: *mut c_void, idx: c_int) -> c_float, *mut c_void, c_int, c_int, *const c_char, c_float, c_float, ImVec2) -> undefined
    #[inline]
    pub fn pop_allow_keyboard_focus<'a>(&'a self) {
        unsafe { igPopAllowKeyboardFocus() };
    }
    #[inline]
    pub fn pop_button_repeat<'a>(&'a self) {
        unsafe { igPopButtonRepeat() };
    }
    #[inline]
    pub fn pop_clip_rect<'a>(&'a self) {
        unsafe { igPopClipRect() };
    }
    #[inline]
    pub fn pop_font<'a>(&'a self) {
        unsafe { igPopFont() };
    }
    #[inline]
    pub fn pop_id<'a>(&'a self) {
        unsafe { igPopID() };
    }
    #[inline]
    pub fn pop_item_width<'a>(&'a self) {
        unsafe { igPopItemWidth() };
    }
    #[inline]
    pub fn pop_style_color<'a>(&'a self, count: impl Into<Option<i32>>) {
        unsafe {
            igPopStyleColor(match count.into() {
                Some(v) => v,
                None => 1,
            })
        };
    }
    #[inline]
    pub fn pop_style_var<'a>(&'a self, count: impl Into<Option<i32>>) {
        unsafe {
            igPopStyleVar(match count.into() {
                Some(v) => v,
                None => 1,
            })
        };
    }
    #[inline]
    pub fn pop_text_wrap_pos<'a>(&'a self) {
        unsafe { igPopTextWrapPos() };
    }
    #[inline]
    pub fn progress_bar<'a, 'b>(
        &'a self,
        fraction: f32,
        size_arg: impl Into<Option<ImVec2>>,
        overlay: impl Into<Option<&'b CStr>>,
    ) {
        unsafe {
            igProgressBar(
                fraction,
                match size_arg.into() {
                    Some(v) => v,
                    None => ImVec2 { x: -1.0, y: 0.0 },
                },
                match overlay.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
            )
        };
    }
    #[inline]
    pub fn push_allow_keyboard_focus<'a>(&'a self, allow_keyboard_focus: bool) {
        unsafe { igPushAllowKeyboardFocus(allow_keyboard_focus) };
    }
    #[inline]
    pub fn push_button_repeat<'a>(&'a self, repeat: bool) {
        unsafe { igPushButtonRepeat(repeat) };
    }
    #[inline]
    pub fn push_clip_rect<'a>(
        &'a self,
        clip_rect_min: ImVec2,
        clip_rect_max: ImVec2,
        intersect_with_current_clip_rect: bool,
    ) {
        unsafe { igPushClipRect(clip_rect_min, clip_rect_max, intersect_with_current_clip_rect) };
    }
    // pub fn push_font : (*mut ImFont) -> undefined
    #[inline]
    pub fn push_id_str<'a, 'b>(&'a self, str_id: &'b CStr) {
        unsafe { igPushIDStr(str_id.as_ptr()) };
    }
    #[inline]
    pub fn push_id_range<'a, 'b, 'c>(&'a self, str_id_begin: &'b CStr, str_id_end: &'c CStr) {
        unsafe { igPushIDRange(str_id_begin.as_ptr(), str_id_end.as_ptr()) };
    }
    // pub fn push_id_ptr : (*const c_void) -> undefined
    #[inline]
    pub fn push_id_int<'a>(&'a self, int_id: i32) {
        unsafe { igPushIDInt(int_id) };
    }
    #[inline]
    pub fn push_item_width<'a>(&'a self, item_width: f32) {
        unsafe { igPushItemWidth(item_width) };
    }
    #[inline]
    pub fn push_style_color_u32<'a>(&'a self, idx: ImGuiCol, col: u32) {
        unsafe { igPushStyleColorU32(idx, col) };
    }
    #[inline]
    pub fn push_style_color<'a>(&'a self, idx: ImGuiCol, col: ImVec4) {
        unsafe { igPushStyleColor(idx, col) };
    }
    #[inline]
    pub fn push_style_var_float<'a>(&'a self, idx: ImGuiStyleVar, val: f32) {
        unsafe { igPushStyleVarFloat(idx, val) };
    }
    #[inline]
    pub fn push_style_var_vec2<'a>(&'a self, idx: ImGuiStyleVar, val: ImVec2) {
        unsafe { igPushStyleVarVec2(idx, val) };
    }
    #[inline]
    pub fn push_text_wrap_pos<'a>(&'a self, wrap_pos_x: impl Into<Option<f32>>) {
        unsafe {
            igPushTextWrapPos(match wrap_pos_x.into() {
                Some(v) => v,
                None => 0.0,
            })
        };
    }
    #[inline]
    pub fn radio_button_bool<'a, 'b>(&'a self, label: &'b CStr, active: bool) -> bool {
        unsafe { igRadioButtonBool(label.as_ptr(), active) }
    }
    #[inline]
    pub fn radio_button_int_ptr<'a, 'b, 'c>(&'a self, label: &'b CStr, v: &'c mut i32, v_button: i32) -> bool {
        unsafe { igRadioButtonIntPtr(label.as_ptr(), v, v_button) }
    }
    #[inline]
    pub fn render<'a>(&'a self) {
        unsafe { igRender() };
    }
    #[inline]
    pub fn reset_mouse_drag_delta<'a>(&'a self, button: impl Into<Option<i32>>) {
        unsafe {
            igResetMouseDragDelta(match button.into() {
                Some(v) => v,
                None => 0,
            })
        };
    }
    #[inline]
    pub fn same_line<'a>(&'a self, pos_x: impl Into<Option<f32>>, spacing_w: impl Into<Option<f32>>) {
        unsafe {
            igSameLine(
                match pos_x.into() {
                    Some(v) => v,
                    None => 0.0,
                },
                match spacing_w.into() {
                    Some(v) => v,
                    None => -1.0,
                },
            )
        };
    }
    #[inline]
    pub fn save_ini_settings_to_disk<'a, 'b>(&'a self, ini_filename: &'b CStr) {
        unsafe { igSaveIniSettingsToDisk(ini_filename.as_ptr()) };
    }
    #[inline]
    pub fn save_ini_settings_to_memory<'a, 'b>(&'a self, out_ini_size: impl Into<Option<&'b mut usize>>) -> String {
        unsafe {
            CStr::from_ptr(igSaveIniSettingsToMemory(match out_ini_size.into() {
                Some(v) => v,
                None => ::std::ptr::null_mut(),
            }))
            .to_string_lossy()
            .into_owned()
        }
    }
    #[inline]
    pub fn selectable<'a, 'b>(
        &'a self,
        label: &'b CStr,
        selected: impl Into<Option<bool>>,
        flags: impl Into<Option<ImGuiSelectableFlags>>,
        size: impl Into<Option<ImVec2>>,
    ) -> bool {
        unsafe {
            igSelectable(
                label.as_ptr(),
                match selected.into() {
                    Some(v) => v,
                    None => false,
                },
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiSelectableFlags::empty(),
                },
                match size.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 0.0, y: 0.0 },
                },
            )
        }
    }
    #[inline]
    pub fn selectable_bool_ptr<'a, 'b, 'c>(
        &'a self,
        label: &'b CStr,
        p_selected: &'c mut bool,
        flags: impl Into<Option<ImGuiSelectableFlags>>,
        size: impl Into<Option<ImVec2>>,
    ) -> bool {
        unsafe {
            igSelectableBoolPtr(
                label.as_ptr(),
                p_selected,
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiSelectableFlags::empty(),
                },
                match size.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 0.0, y: 0.0 },
                },
            )
        }
    }
    #[inline]
    pub fn separator<'a>(&'a self) {
        unsafe { igSeparator() };
    }
    #[inline]
    pub fn set_clipboard_text<'a, 'b>(&'a self, text: &'b CStr) {
        unsafe { igSetClipboardText(text.as_ptr()) };
    }
    #[inline]
    pub fn set_color_edit_options<'a>(&'a self, flags: ImGuiColorEditFlags) {
        unsafe { igSetColorEditOptions(flags) };
    }
    #[inline]
    pub fn set_column_offset<'a>(&'a self, column_index: i32, offset_x: f32) {
        unsafe { igSetColumnOffset(column_index, offset_x) };
    }
    #[inline]
    pub fn set_column_width<'a>(&'a self, column_index: i32, width: f32) {
        unsafe { igSetColumnWidth(column_index, width) };
    }
    #[inline]
    pub fn set_cursor_pos<'a>(&'a self, local_pos: ImVec2) {
        unsafe { igSetCursorPos(local_pos) };
    }
    #[inline]
    pub fn set_cursor_pos_x<'a>(&'a self, x: f32) {
        unsafe { igSetCursorPosX(x) };
    }
    #[inline]
    pub fn set_cursor_pos_y<'a>(&'a self, y: f32) {
        unsafe { igSetCursorPosY(y) };
    }
    #[inline]
    pub fn set_cursor_screen_pos<'a>(&'a self, screen_pos: ImVec2) {
        unsafe { igSetCursorScreenPos(screen_pos) };
    }
    // pub fn set_drag_drop_payload : (*const c_char, *const c_void, size_t, ImGuiCond) -> bool
    #[inline]
    pub fn set_item_allow_overlap<'a>(&'a self) {
        unsafe { igSetItemAllowOverlap() };
    }
    #[inline]
    pub fn set_item_default_focus<'a>(&'a self) {
        unsafe { igSetItemDefaultFocus() };
    }
    #[inline]
    pub fn set_keyboard_focus_here<'a>(&'a self, offset: impl Into<Option<i32>>) {
        unsafe {
            igSetKeyboardFocusHere(match offset.into() {
                Some(v) => v,
                None => 0,
            })
        };
    }
    #[inline]
    pub fn set_mouse_cursor<'a>(&'a self, _type: ImGuiMouseCursor) {
        unsafe { igSetMouseCursor(_type) };
    }
    #[inline]
    pub fn set_next_tree_node_open<'a>(&'a self, is_open: bool, cond: impl Into<Option<ImGuiCond>>) {
        unsafe {
            igSetNextTreeNodeOpen(
                is_open,
                match cond.into() {
                    Some(v) => v,
                    None => ImGuiCond::empty(),
                },
            )
        };
    }
    #[inline]
    pub fn set_next_window_bg_alpha<'a>(&'a self, alpha: f32) {
        unsafe { igSetNextWindowBgAlpha(alpha) };
    }
    #[inline]
    pub fn set_next_window_collapsed<'a>(&'a self, collapsed: bool, cond: impl Into<Option<ImGuiCond>>) {
        unsafe {
            igSetNextWindowCollapsed(
                collapsed,
                match cond.into() {
                    Some(v) => v,
                    None => ImGuiCond::empty(),
                },
            )
        };
    }
    #[inline]
    pub fn set_next_window_content_size<'a>(&'a self, size: ImVec2) {
        unsafe { igSetNextWindowContentSize(size) };
    }
    #[inline]
    pub fn set_next_window_focus<'a>(&'a self) {
        unsafe { igSetNextWindowFocus() };
    }
    #[inline]
    pub fn set_next_window_pos<'a>(
        &'a self,
        pos: ImVec2,
        cond: impl Into<Option<ImGuiCond>>,
        pivot: impl Into<Option<ImVec2>>,
    ) {
        unsafe {
            igSetNextWindowPos(
                pos,
                match cond.into() {
                    Some(v) => v,
                    None => ImGuiCond::empty(),
                },
                match pivot.into() {
                    Some(v) => v,
                    None => ImVec2 { x: 0.0, y: 0.0 },
                },
            )
        };
    }
    #[inline]
    pub fn set_next_window_size<'a>(&'a self, size: ImVec2, cond: impl Into<Option<ImGuiCond>>) {
        unsafe {
            igSetNextWindowSize(
                size,
                match cond.into() {
                    Some(v) => v,
                    None => ImGuiCond::empty(),
                },
            )
        };
    }
    // pub fn set_next_window_size_constraints : (ImVec2, ImVec2, ImGuiSizeCallback, *mut c_void) -> undefined
    #[inline]
    pub fn set_scroll_from_pos_y<'a>(&'a self, pos_y: f32, center_y_ratio: impl Into<Option<f32>>) {
        unsafe {
            igSetScrollFromPosY(
                pos_y,
                match center_y_ratio.into() {
                    Some(v) => v,
                    None => 0.5,
                },
            )
        };
    }
    #[inline]
    pub fn set_scroll_here_y<'a>(&'a self, center_y_ratio: impl Into<Option<f32>>) {
        unsafe {
            igSetScrollHereY(match center_y_ratio.into() {
                Some(v) => v,
                None => 0.5,
            })
        };
    }
    #[inline]
    pub fn set_scroll_x<'a>(&'a self, scroll_x: f32) {
        unsafe { igSetScrollX(scroll_x) };
    }
    #[inline]
    pub fn set_scroll_y<'a>(&'a self, scroll_y: f32) {
        unsafe { igSetScrollY(scroll_y) };
    }
    // pub fn set_state_storage : (*mut ImGuiStorage) -> undefined
    #[inline]
    pub fn set_tooltip<'a, 'b>(&'a self, fmt: &'b CStr) {
        unsafe { igSetTooltip(cstr_ptr!("%s"), fmt.as_ptr()) };
    }
    #[inline]
    pub fn set_window_collapsed_bool<'a>(&'a self, collapsed: bool, cond: impl Into<Option<ImGuiCond>>) {
        unsafe {
            igSetWindowCollapsedBool(
                collapsed,
                match cond.into() {
                    Some(v) => v,
                    None => ImGuiCond::empty(),
                },
            )
        };
    }
    #[inline]
    pub fn set_window_collapsed_str<'a, 'b>(
        &'a self,
        name: &'b CStr,
        collapsed: bool,
        cond: impl Into<Option<ImGuiCond>>,
    ) {
        unsafe {
            igSetWindowCollapsedStr(
                name.as_ptr(),
                collapsed,
                match cond.into() {
                    Some(v) => v,
                    None => ImGuiCond::empty(),
                },
            )
        };
    }
    #[inline]
    pub fn set_window_focus<'a>(&'a self) {
        unsafe { igSetWindowFocus() };
    }
    #[inline]
    pub fn set_window_focus_str<'a, 'b>(&'a self, name: &'b CStr) {
        unsafe { igSetWindowFocusStr(name.as_ptr()) };
    }
    #[inline]
    pub fn set_window_font_scale<'a>(&'a self, scale: f32) {
        unsafe { igSetWindowFontScale(scale) };
    }
    #[inline]
    pub fn set_window_pos_vec2<'a>(&'a self, pos: ImVec2, cond: impl Into<Option<ImGuiCond>>) {
        unsafe {
            igSetWindowPosVec2(
                pos,
                match cond.into() {
                    Some(v) => v,
                    None => ImGuiCond::empty(),
                },
            )
        };
    }
    #[inline]
    pub fn set_window_pos_str<'a, 'b>(&'a self, name: &'b CStr, pos: ImVec2, cond: impl Into<Option<ImGuiCond>>) {
        unsafe {
            igSetWindowPosStr(
                name.as_ptr(),
                pos,
                match cond.into() {
                    Some(v) => v,
                    None => ImGuiCond::empty(),
                },
            )
        };
    }
    #[inline]
    pub fn set_window_size_vec2<'a>(&'a self, size: ImVec2, cond: impl Into<Option<ImGuiCond>>) {
        unsafe {
            igSetWindowSizeVec2(
                size,
                match cond.into() {
                    Some(v) => v,
                    None => ImGuiCond::empty(),
                },
            )
        };
    }
    #[inline]
    pub fn set_window_size_str<'a, 'b>(&'a self, name: &'b CStr, size: ImVec2, cond: impl Into<Option<ImGuiCond>>) {
        unsafe {
            igSetWindowSizeStr(
                name.as_ptr(),
                size,
                match cond.into() {
                    Some(v) => v,
                    None => ImGuiCond::empty(),
                },
            )
        };
    }
    #[inline]
    pub fn show_about_window<'a, 'b>(&'a self, p_open: impl Into<Option<&'b mut bool>>) {
        unsafe {
            igShowAboutWindow(match p_open.into() {
                Some(v) => v,
                None => ::std::ptr::null_mut(),
            })
        };
    }
    #[inline]
    pub fn show_demo_window<'a, 'b>(&'a self, p_open: impl Into<Option<&'b mut bool>>) {
        unsafe {
            igShowDemoWindow(match p_open.into() {
                Some(v) => v,
                None => ::std::ptr::null_mut(),
            })
        };
    }
    #[inline]
    pub fn show_font_selector<'a, 'b>(&'a self, label: &'b CStr) {
        unsafe { igShowFontSelector(label.as_ptr()) };
    }
    #[inline]
    pub fn show_metrics_window<'a, 'b>(&'a self, p_open: impl Into<Option<&'b mut bool>>) {
        unsafe {
            igShowMetricsWindow(match p_open.into() {
                Some(v) => v,
                None => ::std::ptr::null_mut(),
            })
        };
    }
    // pub fn show_style_editor : (*mut ImGuiStyle) -> undefined
    #[inline]
    pub fn show_style_selector<'a, 'b>(&'a self, label: &'b CStr) -> bool {
        unsafe { igShowStyleSelector(label.as_ptr()) }
    }
    #[inline]
    pub fn show_user_guide<'a>(&'a self) {
        unsafe { igShowUserGuide() };
    }
    #[inline]
    pub fn slider_angle<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v_rad: &'c mut f32,
        v_degrees_min: impl Into<Option<f32>>,
        v_degrees_max: impl Into<Option<f32>>,
        format: impl Into<Option<&'d CStr>>,
    ) -> bool {
        unsafe {
            igSliderAngle(
                label.as_ptr(),
                v_rad,
                match v_degrees_min.into() {
                    Some(v) => v,
                    None => -360.0,
                },
                match v_degrees_max.into() {
                    Some(v) => v,
                    None => 360.0,
                },
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.0f deg"),
                },
            )
        }
    }
    #[inline]
    pub fn slider_float<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut f32,
        v_min: f32,
        v_max: f32,
        format: impl Into<Option<&'d CStr>>,
        power: impl Into<Option<f32>>,
    ) -> bool {
        unsafe {
            igSliderFloat(
                label.as_ptr(),
                v,
                v_min,
                v_max,
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match power.into() {
                    Some(v) => v,
                    None => 1.0,
                },
            )
        }
    }
    #[inline]
    pub fn slider_float2<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [f32; 2],
        v_min: f32,
        v_max: f32,
        format: impl Into<Option<&'d CStr>>,
        power: impl Into<Option<f32>>,
    ) -> bool {
        unsafe {
            igSliderFloat2(
                label.as_ptr(),
                v,
                v_min,
                v_max,
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match power.into() {
                    Some(v) => v,
                    None => 1.0,
                },
            )
        }
    }
    #[inline]
    pub fn slider_float3<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [f32; 3],
        v_min: f32,
        v_max: f32,
        format: impl Into<Option<&'d CStr>>,
        power: impl Into<Option<f32>>,
    ) -> bool {
        unsafe {
            igSliderFloat3(
                label.as_ptr(),
                v,
                v_min,
                v_max,
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match power.into() {
                    Some(v) => v,
                    None => 1.0,
                },
            )
        }
    }
    #[inline]
    pub fn slider_float4<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [f32; 4],
        v_min: f32,
        v_max: f32,
        format: impl Into<Option<&'d CStr>>,
        power: impl Into<Option<f32>>,
    ) -> bool {
        unsafe {
            igSliderFloat4(
                label.as_ptr(),
                v,
                v_min,
                v_max,
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match power.into() {
                    Some(v) => v,
                    None => 1.0,
                },
            )
        }
    }
    #[inline]
    pub fn slider_int<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut i32,
        v_min: i32,
        v_max: i32,
        format: impl Into<Option<&'d CStr>>,
    ) -> bool {
        unsafe {
            igSliderInt(
                label.as_ptr(),
                v,
                v_min,
                v_max,
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%d"),
                },
            )
        }
    }
    #[inline]
    pub fn slider_int2<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [i32; 2],
        v_min: i32,
        v_max: i32,
        format: impl Into<Option<&'d CStr>>,
    ) -> bool {
        unsafe {
            igSliderInt2(
                label.as_ptr(),
                v,
                v_min,
                v_max,
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%d"),
                },
            )
        }
    }
    #[inline]
    pub fn slider_int3<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [i32; 3],
        v_min: i32,
        v_max: i32,
        format: impl Into<Option<&'d CStr>>,
    ) -> bool {
        unsafe {
            igSliderInt3(
                label.as_ptr(),
                v,
                v_min,
                v_max,
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%d"),
                },
            )
        }
    }
    #[inline]
    pub fn slider_int4<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        v: &'c mut [i32; 4],
        v_min: i32,
        v_max: i32,
        format: impl Into<Option<&'d CStr>>,
    ) -> bool {
        unsafe {
            igSliderInt4(
                label.as_ptr(),
                v,
                v_min,
                v_max,
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%d"),
                },
            )
        }
    }
    // pub fn slider_scalar : (*const c_char, ImGuiDataType, *mut c_void, *const c_void, *const c_void, *const c_char, c_float) -> bool
    // pub fn slider_scalar_n : (*const c_char, ImGuiDataType, *mut c_void, c_int, *const c_void, *const c_void, *const c_char, c_float) -> bool
    #[inline]
    pub fn small_button<'a, 'b>(&'a self, label: &'b CStr) -> bool {
        unsafe { igSmallButton(label.as_ptr()) }
    }
    #[inline]
    pub fn spacing<'a>(&'a self) {
        unsafe { igSpacing() };
    }
    // pub fn style_colors_classic : (*mut ImGuiStyle) -> undefined
    // pub fn style_colors_dark : (*mut ImGuiStyle) -> undefined
    // pub fn style_colors_light : (*mut ImGuiStyle) -> undefined
    #[inline]
    pub fn text<'a, 'b>(&'a self, fmt: &'b CStr) {
        unsafe { igText(cstr_ptr!("%s"), fmt.as_ptr()) };
    }
    #[inline]
    pub fn text_colored<'a, 'b>(&'a self, col: ImVec4, fmt: &'b CStr) {
        unsafe { igTextColored(col, cstr_ptr!("%s"), fmt.as_ptr()) };
    }
    #[inline]
    pub fn text_disabled<'a, 'b>(&'a self, fmt: &'b CStr) {
        unsafe { igTextDisabled(cstr_ptr!("%s"), fmt.as_ptr()) };
    }
    #[inline]
    pub fn text_wrapped<'a, 'b>(&'a self, fmt: &'b CStr) {
        unsafe { igTextWrapped(cstr_ptr!("%s"), fmt.as_ptr()) };
    }
    #[inline]
    pub fn tree_advance_to_label_pos<'a>(&'a self) {
        unsafe { igTreeAdvanceToLabelPos() };
    }
    #[inline]
    pub fn tree_node_str<'a, 'b>(&'a self, label: &'b CStr) -> bool {
        unsafe { igTreeNodeStr(label.as_ptr()) }
    }
    #[inline]
    pub fn tree_node_str_str<'a, 'b, 'c>(&'a self, str_id: &'b CStr, fmt: &'c CStr) -> bool {
        unsafe { igTreeNodeStrStr(str_id.as_ptr(), cstr_ptr!("%s"), fmt.as_ptr()) }
    }
    // pub fn tree_node_ptr : (*const c_void, *const c_char, ) -> bool
    #[inline]
    pub fn tree_node_ex_str<'a, 'b>(&'a self, label: &'b CStr, flags: impl Into<Option<ImGuiTreeNodeFlags>>) -> bool {
        unsafe {
            igTreeNodeExStr(
                label.as_ptr(),
                match flags.into() {
                    Some(v) => v,
                    None => ImGuiTreeNodeFlags::empty(),
                },
            )
        }
    }
    #[inline]
    pub fn tree_node_ex_str_str<'a, 'b, 'c>(
        &'a self,
        str_id: &'b CStr,
        flags: ImGuiTreeNodeFlags,
        fmt: &'c CStr,
    ) -> bool {
        unsafe { igTreeNodeExStrStr(str_id.as_ptr(), flags, cstr_ptr!("%s"), fmt.as_ptr()) }
    }
    // pub fn tree_node_ex_ptr : (*const c_void, ImGuiTreeNodeFlags, *const c_char, ) -> bool
    #[inline]
    pub fn tree_pop<'a>(&'a self) {
        unsafe { igTreePop() };
    }
    #[inline]
    pub fn tree_push_str<'a, 'b>(&'a self, str_id: &'b CStr) {
        unsafe { igTreePushStr(str_id.as_ptr()) };
    }
    // pub fn tree_push_ptr : (*const c_void) -> undefined
    #[inline]
    pub fn unindent<'a>(&'a self, indent_w: impl Into<Option<f32>>) {
        unsafe {
            igUnindent(match indent_w.into() {
                Some(v) => v,
                None => 0.0,
            })
        };
    }
    #[inline]
    pub fn v_slider_float<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        size: ImVec2,
        v: &'c mut f32,
        v_min: f32,
        v_max: f32,
        format: impl Into<Option<&'d CStr>>,
        power: impl Into<Option<f32>>,
    ) -> bool {
        unsafe {
            igVSliderFloat(
                label.as_ptr(),
                size,
                v,
                v_min,
                v_max,
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%.3f"),
                },
                match power.into() {
                    Some(v) => v,
                    None => 1.0,
                },
            )
        }
    }
    #[inline]
    pub fn v_slider_int<'a, 'b, 'c, 'd>(
        &'a self,
        label: &'b CStr,
        size: ImVec2,
        v: &'c mut i32,
        v_min: i32,
        v_max: i32,
        format: impl Into<Option<&'d CStr>>,
    ) -> bool {
        unsafe {
            igVSliderInt(
                label.as_ptr(),
                size,
                v,
                v_min,
                v_max,
                match format.into() {
                    Some(v) => v.as_ptr(),
                    None => cstr_ptr!("%d"),
                },
            )
        }
    }
    // pub fn v_slider_scalar : (*const c_char, ImVec2, ImGuiDataType, *mut c_void, *const c_void, *const c_void, *const c_char, c_float) -> bool
    #[inline]
    pub fn value_bool<'a, 'b>(&'a self, prefix: &'b CStr, b: bool) {
        unsafe { igValueBool(prefix.as_ptr(), b) };
    }
    #[inline]
    pub fn value_int<'a, 'b>(&'a self, prefix: &'b CStr, v: i32) {
        unsafe { igValueInt(prefix.as_ptr(), v) };
    }
    #[inline]
    pub fn value_uint<'a, 'b>(&'a self, prefix: &'b CStr, v: u32) {
        unsafe { igValueUint(prefix.as_ptr(), v) };
    }
    #[inline]
    pub fn value_float<'a, 'b, 'c>(&'a self, prefix: &'b CStr, v: f32, float_format: impl Into<Option<&'c CStr>>) {
        unsafe {
            igValueFloat(
                prefix.as_ptr(),
                v,
                match float_format.into() {
                    Some(v) => v.as_ptr(),
                    None => ::std::ptr::null(),
                },
            )
        };
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
