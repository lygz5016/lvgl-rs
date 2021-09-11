use crate::widgets::Arc;

impl Arc {
    // /// Set the start angle, for the given arc part.
    // /// 0 degrees for the right, 90 degrees for the bottom, etc.
    // pub fn set_start_angle(&mut self, angle: u16, part: ArcPart) -> LvResult<()> {
    //     match part {
    //         ArcPart::Background => unsafe {
    //             lvgl_sys::lv_arc_set_bg_start_angle(self.core.raw()?.as_mut(), angle)
    //         },
    //         ArcPart::Indicator => unsafe {
    //             lvgl_sys::lv_arc_set_start_angle(self.core.raw()?.as_mut(), angle)
    //         },
    //     }
    //     Ok(())
    // }
    //
    // /// Set the end angle, for the given arc part.
    // /// 0 degrees for the right, 90 degrees for the bottom, etc.
    // pub fn set_end_angle(&self, angle: u16, part: ArcPart) -> LvResult<()> {
    //     match part {
    //         ArcPart::Background => unsafe {
    //             lvgl_sys::lv_arc_set_bg_end_angle(self.core.raw()?.as_mut(), angle)
    //         },
    //         ArcPart::Indicator => unsafe {
    //             lvgl_sys::lv_arc_set_end_angle(self.core.raw()?.as_mut(), angle)
    //         },
    //     }
    //     Ok(())
    // }
    //
    // /// Rotate the arc, `angle` degrees clockwise.
    // pub fn set_rotation(&mut self, angle: u16) -> LvResult<()> {
    //     unsafe {
    //         lvgl_sys::lv_arc_set_rotation(self.core.raw()?.as_mut(), angle);
    //     }
    //     Ok(())
    // }
}
