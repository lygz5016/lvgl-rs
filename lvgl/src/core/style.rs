use crate::Box;
use crate::{Color, LvResult, State};
use core::mem;
use cstr_core::CStr;

pub enum Themes {
    Pretty,
}

pub struct Style {
    pub(crate) raw: Box<lvgl_sys::lv_style_t>,
}

impl Style {
    /*pub fn set_value_str(&mut self, state: State, value: &CStr) -> LvResult<()> {
        unsafe {
            lvgl_sys::_lv_style_set_ptr(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_VALUE_STR,
                value.as_ptr() as *mut cty::c_void,
            );
        }
        Ok(())
    }*/

    pub fn reset(&mut self) {
        unsafe { lvgl_sys::lv_style_reset(self.raw.as_mut()) }
    }

    pub fn register_prop() -> lvgl_sys::lv_style_prop_t {
        unsafe { lvgl_sys::lv_style_register_prop() }
    }

    pub fn remove_prop(&mut self, prop: lvgl_sys::lv_style_prop_t) -> bool {
        unsafe { lvgl_sys::lv_style_remove_prop(self.raw.as_mut(), prop) }
    }

    pub fn raw_as_mut(&mut self) -> *mut lvgl_sys::lv_style_t {
        self.raw.as_mut()
    }
}

impl Default for Style {
    fn default() -> Self {
        let raw = unsafe {
            let mut style = mem::MaybeUninit::<lvgl_sys::lv_style_t>::uninit();
            lvgl_sys::lv_style_init(style.as_mut_ptr());
            Box::new(style.assume_init())
        };
        Self { raw }
    }
}

// Auto-gen code, please look into lvgl-codegen for any changes.
/*impl Style {
    pub fn set_radius(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_RADIUS,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_clip_corner(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_CLIP_CORNER,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_size(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SIZE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_transform_width(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_TRANSFORM_WIDTH,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_transform_height(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_TRANSFORM_HEIGHT,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_transform_angle(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_TRANSFORM_ANGLE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_transform_zoom(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_TRANSFORM_ZOOM,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_opa_scale(&mut self, state: State, value: Opacity) {
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_OPA_SCALE,
                value.into(),
            );
        }
    }

    pub fn set_pad_top(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_PAD_TOP,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_pad_bottom(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_PAD_BOTTOM,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_pad_left(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_PAD_LEFT,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_pad_right(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_PAD_RIGHT,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_pad_inner(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_PAD_INNER,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_margin_top(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_MARGIN_TOP,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_margin_bottom(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_MARGIN_BOTTOM,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_margin_left(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_MARGIN_LEFT,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_margin_right(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_MARGIN_RIGHT,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_bg_blend_mode(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_BG_BLEND_MODE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_bg_main_stop(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_BG_MAIN_STOP,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_bg_grad_stop(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_BG_GRAD_STOP,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_bg_grad_dir(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_BG_GRAD_DIR,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_bg_color(&mut self, state: State, value: Color) {
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_BG_COLOR,
                value.raw,
            );
        }
    }

    pub fn set_bg_grad_color(&mut self, state: State, value: Color) {
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_BG_GRAD_COLOR,
                value.raw,
            );
        }
    }

    pub fn set_bg_opa(&mut self, state: State, value: Opacity) {
        unsafe {
            lvgl_sys::_lv_style_set_opa(self.raw.as_mut(), lvgl_sys::LV_STYLE_BG_OPA, value.into());
        }
    }

    pub fn set_border_width(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_BORDER_WIDTH,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_border_side(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_BORDER_SIDE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_border_blend_mode(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_BORDER_BLEND_MODE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_border_post(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_BORDER_POST,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_border_color(&mut self, state: State, value: Color) {
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_BORDER_COLOR,
                value.raw,
            );
        }
    }

    pub fn set_border_opa(&mut self, state: State, value: Opacity) {
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_BORDER_OPA,
                value.into(),
            );
        }
    }

    pub fn set_outline_width(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_OUTLINE_WIDTH,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_outline_pad(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_OUTLINE_PAD,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_outline_blend_mode(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_OUTLINE_BLEND_MODE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_outline_color(&mut self, state: State, value: Color) {
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_OUTLINE_COLOR,
                value.raw,
            );
        }
    }

    pub fn set_outline_opa(&mut self, state: State, value: Opacity) {
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_OUTLINE_OPA,
                value.into(),
            );
        }
    }

    pub fn set_shadow_width(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SHADOW_WIDTH,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_shadow_ofs_x(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SHADOW_OFS_X,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_shadow_ofs_y(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SHADOW_OFS_Y,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_shadow_spread(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SHADOW_SPREAD,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_shadow_blend_mode(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SHADOW_BLEND_MODE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_shadow_color(&mut self, state: State, value: Color) {
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SHADOW_COLOR,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_shadow_opa(&mut self, state: State, value: Opacity) {
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SHADOW_OPA,
                value.into(),
            );
        }
    }

    pub fn set_pattern_repeat(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_PATTERN_REPEAT,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_pattern_blend_mode(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_PATTERN_BLEND_MODE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_pattern_recolor(&mut self, state: State, value: Color) {
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_PATTERN_RECOLOR,
                value.raw,
            );
        }
    }

    pub fn set_pattern_opa(&mut self, state: State, value: Opacity) {
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_PATTERN_OPA,
                value.into(),
            );
        }
    }

    pub fn set_pattern_recolor_opa(&mut self, state: State, value: Opacity) {
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_PATTERN_RECOLOR_OPA,
                value.into(),
            );
        }
    }

    pub fn set_value_letter_space(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_VALUE_LETTER_SPACE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_value_line_space(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_VALUE_LINE_SPACE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_value_blend_mode(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_VALUE_BLEND_MODE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_shadow_ofs_x(&mut self, value: lv_coord_t) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_OFS_X,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_shadow_ofs_y(&mut self, value: lv_coord_t) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_OFS_Y,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_value_align(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::lv_style_prop_t_LV_STYLE_ALIGN,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_value_color(&mut self, state: State, value: Color) {
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_VALUE_COLOR,
                value.raw,
            );
        }
    }

    pub fn set_value_opa(&mut self, state: State, value: Opacity) {
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_VALUE_OPA,
                value.into(),
            );
        }
    }

    pub fn set_text_letter_space(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_TEXT_LETTER_SPACE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_text_line_space(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_TEXT_LINE_SPACE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_text_decor(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_TEXT_DECOR,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_text_blend_mode(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_TEXT_BLEND_MODE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_text_color(&mut self, state: State, value: Color) {
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_TEXT_COLOR,
                value.raw,
            );
        }
    }

    pub fn set_text_sel_color(&mut self, state: State, value: Color) {
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_TEXT_SEL_COLOR,
                value.raw,
            );
        }
    }

    pub fn set_text_opa(&mut self, state: State, value: Opacity) {
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_TEXT_OPA,
                value.into(),
            );
        }
    }

    pub fn set_line_width(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_LINE_WIDTH,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_line_blend_mode(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_LINE_BLEND_MODE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_line_dash_width(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_LINE_DASH_WIDTH,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_line_dash_gap(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_LINE_DASH_GAP,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_line_rounded(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_LINE_ROUNDED,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_line_color(&mut self, state: State, value: Color) {
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_LINE_COLOR,
                value.raw,
            );
        }
    }

    pub fn set_line_opa(&mut self, state: State, value: Opacity) {
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_LINE_OPA,
                value.into(),
            );
        }
    }

    pub fn set_image_blend_mode(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_IMAGE_BLEND_MODE,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_image_recolor(&mut self, state: State, value: Color) {
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_IMAGE_RECOLOR,
                value.raw,
            );
        }
    }

    pub fn set_image_opa(&mut self, state: State, value: lv_opa_t) {
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                lvgl_sys::lv_style_prop_t_LV_STYLE_IMG_OPA,
                lvgl_sys::lv_style_value_t { num: value as i32},
            );
        }
    }

    pub fn set_image_recolor_opa(&mut self, state: State, value: Opacity) {
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_IMAGE_RECOLOR_OPA,
                value.into(),
            );
        }
    }

    pub fn set_transition(&mut self, state: State, value: *const lv_style_transition_dsc_t) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_TRANSITION,
                lvgl_sys::lv_style_value_t { ptr: value as *const cty::c_void},
            );
        }
    }

    pub fn set_scale_width(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SCALE_WIDTH,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_scale_border_width(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SCALE_BORDER_WIDTH,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_scale_end_border_width(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SCALE_END_BORDER_WIDTH,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_scale_end_line_width(&mut self, state: State, value: i16) {
        unsafe {
            lvgl_sys::lv_style_set_prop(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SCALE_END_LINE_WIDTH,
                lvgl_sys::lv_style_value_t { num: value as i32 },
            );
        }
    }

    pub fn set_scale_grad_color(&mut self, state: State, value: Color) {
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SCALE_GRAD_COLOR,
                value.raw,
            );
        }
    }

    pub fn set_scale_end_color(&mut self, state: State, value: Color) {
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                lvgl_sys::LV_STYLE_SCALE_END_COLOR,
                value.raw,
            );
        }
    }
}
*/
