use crate::support::Animation;
use crate::widgets::Bar;
use crate::{LvResult, NativeObject};

impl Bar {
    /// Set a new value on the bar
    pub fn set_value(&mut self, value: i16, anim: Animation) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_bar_set_value(self.core.raw()?.as_mut(), value.into(), anim.into());
        }
        Ok(())
    }
}
