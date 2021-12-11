use crate::lv_core::style::Style;
use crate::Box;
use crate::{Align, LvError, LvResult};
use core::ptr;

/// Represents a native LVGL object
pub trait NativeObject {
    /// Provide common way to access to the underlying native object pointer.
    fn raw(&self) -> LvResult<ptr::NonNull<lvgl_sys::lv_obj_t>>;
}

/// Generic LVGL object.
///
/// This is the parent object of all widget types. It stores the native LVGL raw pointer.
pub struct Obj {
    // We use a raw pointer here because we do not control this memory address, it is controlled
    // by LVGL's global state.
    raw: *mut lvgl_sys::lv_obj_t,
}

impl NativeObject for Obj {
    fn raw(&self) -> LvResult<ptr::NonNull<lvgl_sys::lv_obj_t>> {
        if let Some(non_null_ptr) = ptr::NonNull::new(self.raw) {
            Ok(non_null_ptr)
        } else {
            Err(LvError::InvalidReference)
        }
    }
}

/// A wrapper for all LVGL common operations on generic objects.
pub trait Widget: NativeObject {
    type SpecialEvent;
    type Part: Into<i32>;

    /// Construct an instance of the object from a raw pointer.
    ///
    /// # Safety
    /// Provided the LVGL library can allocate memory this should be safe.
    ///
    unsafe fn from_raw(raw_pointer: ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self;

    fn set_parent<P>(&mut self, parent: &mut P) -> LvResult<()>
    where
        P: NativeObject,
    {
        unsafe {
            lvgl_sys::lv_obj_set_parent(self.raw()?.as_mut(), parent.raw()?.as_mut());
        }
        Ok(())
    }

    fn move_foreground(&mut self) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_obj_move_foreground(self.raw()?.as_mut());
        }
        Ok(())
    }

    fn move_background(&mut self) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_obj_move_background(self.raw()?.as_mut());
        }
        Ok(())
    }

    fn add_style(
        &self,
        style: &mut Style,
        selector: lvgl_sys::lv_style_selector_t,
    ) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_obj_add_style(self.raw()?.as_mut(), style.raw_as_mut(), selector);
        };
        Ok(())
    }

    fn set_pos(&mut self, x: i16, y: i16) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_obj_set_pos(
                self.raw()?.as_mut(),
                x as lvgl_sys::lv_coord_t,
                y as lvgl_sys::lv_coord_t,
            );
        }
        Ok(())
    }

    fn set_size(&mut self, w: i16, h: i16) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_obj_set_size(
                self.raw()?.as_mut(),
                w as lvgl_sys::lv_coord_t,
                h as lvgl_sys::lv_coord_t,
            );
        }
        Ok(())
    }

    fn set_width(&mut self, w: u32) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_obj_set_width(self.raw()?.as_mut(), w as lvgl_sys::lv_coord_t);
        }
        Ok(())
    }

    fn set_height(&mut self, h: u32) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_obj_set_height(self.raw()?.as_mut(), h as lvgl_sys::lv_coord_t);
        }
        Ok(())
    }
    fn set_align(&mut self, align: Align) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_obj_set_align(self.raw()?.as_mut(), align.into());
        }
        Ok(())
    }

    fn align(&mut self, align: Align, x_ofs: i32, y_ofs: i32) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_obj_align(
                self.raw()?.as_mut(),
                align.into(),
                x_ofs as lvgl_sys::lv_coord_t,
                y_ofs as lvgl_sys::lv_coord_t,
            );
        }
        Ok(())
    }

    fn align_to<C>(&mut self, base: &mut C, align: Align, x_ofs: i32, y_ofs: i32) -> LvResult<()>
    where
        C: NativeObject,
    {
        unsafe {
            lvgl_sys::lv_obj_align_to(
                self.raw()?.as_mut(),
                base.raw()?.as_ref(),
                align.into(),
                x_ofs as lvgl_sys::lv_coord_t,
                y_ofs as lvgl_sys::lv_coord_t,
            );
        }
        Ok(())
    }
}

impl Widget for Obj {
    type SpecialEvent = ();
    type Part = Part;

    unsafe fn from_raw(raw: ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self {
        Self { raw: raw.as_ptr() }
    }
}

impl Default for Obj {
    fn default() -> Self {
        Self {
            raw: unsafe { lvgl_sys::lv_obj_create(ptr::null_mut()) },
        }
    }
}
impl Drop for Obj {
    fn drop(&mut self) {
        if let Ok(s) = &mut self.raw() {
            unsafe { lvgl_sys::lv_obj_del(s.as_mut()) }
        }
    }
}

macro_rules! define_object {
    ($item:ident) => {
        define_object!($item, event = (), part = $crate::Part);
    };
    ($item:ident, event = $event_type:ty) => {
        define_object!($item, event = $event_type, part = $crate::Part);
    };
    ($item:ident, part = $part_type:ty) => {
        define_object!($item, event = (), part = $part_type);
    };
    ($item:ident, part = $part_type:ty, event = $event_type:ty) => {
        define_object!($item, event = $event_type, part = $part_type);
    };
    ($item:ident, event = $event_type:ty, part = $part_type:ty) => {
        pub struct $item {
            core: $crate::Obj,
        }

        unsafe impl Send for $item {}

        impl $item {
            pub fn on_event(
                &mut self,
                f: lv_event_cb_t,
                filter: i32,
                user_data: &mut cty::c_void,
            ) -> $crate::LvResult<()> {
                use $crate::NativeObject;
                unsafe {
                    let mut raw = self.raw()?;
                    let obj = raw.as_mut();
                    lvgl_sys::lv_obj_add_event_cb(obj, f, filter, user_data);
                }
                Ok(())
            }
        }

        impl $crate::NativeObject for $item {
            fn raw(&self) -> $crate::LvResult<core::ptr::NonNull<lvgl_sys::lv_obj_t>> {
                self.core.raw()
            }
        }

        impl $crate::Widget for $item {
            type SpecialEvent = $event_type;
            type Part = $part_type;

            unsafe fn from_raw(raw_pointer: core::ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self {
                Self {
                    core: $crate::Obj::from_raw(raw_pointer),
                }
            }
        }
    };
}

bitflags! {
    /// Possible states of a widget. OR-ed values are possible
    pub struct State: u16 {
        /// Normal, released
        const DEFAULT  = lvgl_sys::LV_STATE_DEFAULT as u16;
        /// Toggled or checked
        const CHECKED  = lvgl_sys::LV_STATE_CHECKED as u16;
        /// Focused via keypad or encoder or clicked via touchpad/mouse
        const FOCUSED  = lvgl_sys::LV_STATE_FOCUSED as u16;
        /// Edit by an encoder
        const EDITED   = lvgl_sys::LV_STATE_EDITED as u16;
        /// Hovered by mouse (not supported now)
        const HOVERED  = lvgl_sys::LV_STATE_HOVERED as u16;
        /// Pressed
        const PRESSED  = lvgl_sys::LV_STATE_PRESSED as u16;
        /// Disabled or inactive
        const DISABLED = lvgl_sys::LV_STATE_DISABLED as u16;
        /// Custom state
        const USER1 = lvgl_sys::LV_STATE_USER_1 as u16;
        /// Custom state
        const USER2 = lvgl_sys::LV_STATE_USER_2 as u16;
        /// Custom state
        const USER3 = lvgl_sys::LV_STATE_USER_3 as u16;
        /// Custom state
        const USER4 = lvgl_sys::LV_STATE_USER_4 as u16;
        /// Special value can be used in some functions to target all states
        const ANY = lvgl_sys::LV_STATE_ANY as u16;
    }
}

impl State {
    pub(crate) fn get_bits(&self) -> u16 {
        self.bits
    }
}

/// The widgets are built from multiple parts. For example a Base object uses the main and scrollbar parts but a Slider uses the main, the indicator and the knob parts. Parts are similar to pseudo-elements in CSS.
pub enum Part {
    /// A background like rectangle*/``
    Main,
    /// The scrollbar(s)
    SCROLLBAR,
    /// Indicator, e.g. for slider, bar, switch, or the tick box of the checkbox
    INDICATOR,
    /// Like a handle to grab to adjust the value*/
    KNOB,
    ///  Indicate the currently selected option or section
    SELECTED,
    /// Used if the widget has multiple similar elements (e.g. table cells)*/
    ITEMS,
    /// Ticks on scales e.g. for a chart or meter
    TICKS,
    /// Mark a specific place e.g. text area's or chart's cursor
    CURSOR,
    /// Custom parts can be added from here.
    CustomFirst,
    ///
    All,
}

impl Into<i32> for Part {
    fn into(self) -> i32 {
        match self {
            Part::Main => lvgl_sys::LV_PART_MAIN,
            Part::SCROLLBAR => lvgl_sys::LV_PART_SCROLLBAR,
            Part::INDICATOR => lvgl_sys::LV_PART_INDICATOR,
            Part::KNOB => lvgl_sys::LV_PART_KNOB,
            Part::SELECTED => lvgl_sys::LV_PART_SELECTED,
            Part::ITEMS => lvgl_sys::LV_PART_ITEMS,
            Part::TICKS => lvgl_sys::LV_PART_TICKS,
            Part::CURSOR => lvgl_sys::LV_PART_CURSOR,
            Part::CustomFirst => lvgl_sys::LV_PART_CUSTOM_FIRST,
            Part::All => lvgl_sys::LV_PART_ANY,
        }
    }
}
