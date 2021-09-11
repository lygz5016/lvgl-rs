use crate::widgets::Checkbox;
use crate::{LvResult, NativeObject};

impl Checkbox {
    pub fn set_cheched(&mut self, checked: bool) -> LvResult<()> {
        unsafe {
            if checked {
                lvgl_sys::lv_obj_add_state(self.core.raw()?.as_mut(), LV_STATE_CHECKED)
            } else {
                lvgl_sys::lv_obj_clear_state(self.core.raw()?.as_mut(), LV_STATE_CHECKED)
            }
        }
        Ok(())
    }

    pub fn set_disabled(&mut self, disabled: bool) -> LvResult<()> {
        unsafe { lvgl_sys::lv_obj_add_state(self.core.raw()?.as_mut(), LV_STATE_DISABLED) }
        Ok(())
    }
}
