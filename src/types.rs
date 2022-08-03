use serde::{Deserialize, Serialize};

pub mod mask {
    use crate::Mask;
    use x11::xlib::{self};
    pub const SHIFT: Mask = xlib::ShiftMask;
    pub const LOCK: Mask = xlib::LockMask;
    pub const CTRL: Mask = xlib::ControlMask;
    pub const ALT: Mask = xlib::Mod1Mask;
    pub const MOD2: Mask = xlib::Mod2Mask;
    pub const MOD3: Mask = xlib::Mod3Mask;
    pub const SUPER: Mask = xlib::Mod4Mask;
    pub const MOD5: Mask = xlib::Mod5Mask;
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
struct Binding {
    pub key: u32,
    pub mask: u32,
    pub class: Option<String>,
}
impl Binding {
    pub fn new(key: u32, mask: u32, class: Option<String>) -> Self {
        Binding { key, mask, class }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash, Serialize, Deserialize, Debug)]
pub struct KeyPress {
    pub key: u32,
    pub mask: u32,
}
impl KeyPress {
    pub fn new(key: u32, mask: u32) -> Self {
        KeyPress { key, mask }
    }
}