use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_int, c_uint, c_ulong};
use std::ptr::slice_from_raw_parts;
use std::{mem, ptr};
use x11::keysym::XK_1;
use x11::xlib::{self, AnyKey, AnyModifier, Mod1Mask, Window};
use x11::xlib_xcb::XGetXCBConnection;

use crate::KeyPress;

pub mod atom {
    // pub const NET_ACTIVE_WINDOW: u64 = 335;
}

pub unsafe fn grab_all_keys(display: *mut xlib::Display, root: xlib::Window) -> c_int {
    // let keycode = xlib::XKeysymToKeycode(display, XK_1.into()) as c_int;
    xlib::XGrabKey(
        display,
        // keycode,
        AnyKey,
        AnyModifier,
        // xlib::Mod1Mask,     // mask
        root,               // window
        true as c_int,      // owner events
        xlib::GrabModeSync, // pointer mode
        xlib::GrabModeSync, // keyboard mode
    )
}

pub unsafe fn grab_with_mask(
    display: *mut xlib::Display,
    root: xlib::Window,
    key: c_uint,
    mask: c_uint,
) {
    let key = key as xlib::KeySym;
    let keycode = xlib::XKeysymToKeycode(display, key) as c_int;
    xlib::XGrabKey(
        display,
        keycode,
        // AnyKey,
        mask,
        // mask,
        root,
        true as c_int,
        xlib::GrabModeSync,
        xlib::GrabModeSync,
    );
}

pub unsafe fn get_default_display() -> *mut xlib::Display {
    xlib::XOpenDisplay(ptr::null())
}

pub unsafe fn get_root(display: *mut xlib::Display) -> xlib::Window {
    xlib::XDefaultRootWindow(display)
}

pub fn next_event(display: *mut xlib::Display, event: &mut xlib::XEvent) {
    unsafe {
        xlib::XNextEvent(display, event);
    }
}

pub fn get_keysym(press: &mut xlib::XKeyEvent) -> xlib::KeySym {
    unsafe { xlib::XLookupKeysym(press, 0) }
}

pub fn get_state(press: &mut xlib::XKeyEvent) -> c_uint {
    press.state
}

pub fn send_key_event(
    display: *mut xlib::Display,
    root: Window,
    window: Window,
    keypress: KeyPress,
) {
    unsafe {
        let keyevent = xlib::XKeyEvent {
            type_: xlib::KeyPress,
            serial: 0,
            send_event: 1,
            display,
            window,
            root,
            subwindow: 0,
            time: 0,
            x: 0,
            y: 0,
            x_root: 0,
            y_root: 0,
            state: keypress.mask,
            keycode: xlib::XKeysymToKeycode(display, keypress.key.into()) as u32,
            same_screen: 1,
        };
        let mut event = xlib::XEvent { pad: [0; 24] };
        event.key = keyevent;
        xlib::XSendEvent(
            display,
            event.key.subwindow,
            true as c_int,
            xlib::KeyPressMask,
            &mut event as *mut xlib::XEvent,
        );
    }
}

pub fn get_active_window_id(display: *mut xlib::Display, atom_active_window:u64, root_window: c_ulong) -> Option<u64> {
    unsafe {
        let mut actual_type_return: u64 = 0;
        let mut actual_format_return: i32 = 0;
        let mut nitems_return: u64 = 0;
        let mut bytes_after_return: u64 = 0;
        let uninit = mem::MaybeUninit::uninit();
        let mut prop_return: *mut u8 = uninit.assume_init();
        let r = xlib::XGetWindowProperty(
            display,
            root_window as u64,
            atom_active_window,
            0,
            i64::MAX,
            0,
            0,
            &mut actual_type_return,
            &mut actual_format_return,
            &mut nitems_return,
            &mut bytes_after_return,
            &mut prop_return,
        );
        if r != 0 || actual_format_return == 0 {
            None
        } else {
            let pt = mem::transmute::<*mut u8, *const u64>(prop_return);
            Some(*pt as u64)
        }
    }
}

pub fn pass_through(display: *mut xlib::Display, event: &mut xlib::XEvent) {
    unsafe {
        xlib::XAllowEvents(display, xlib::ReplayKeyboard, event.key.time);
        xlib::XFlush(display);
    }
}

pub fn swallow_event(display: *mut xlib::Display, event: &mut xlib::XEvent) {
    unsafe {
        xlib::XAllowEvents(display, xlib::AsyncKeyboard, event.key.time);
        // burp
    }
}

pub fn get_property(display: *mut xlib::Display, atom: Window, window: Window) -> Option<Vec<u64>> {
    unsafe {
        let mut actual_type_return: u64 = 0;
        let mut actual_format_return: i32 = 0;
        let mut nitems_return: u64 = 0;
        let mut bytes_after_return: u64 = 0;
        let ui = mem::MaybeUninit::uninit();
        let mut prop_return: *mut u8 = ui.assume_init();

        let r = xlib::XGetWindowProperty(
            display,
            window as u64,
            atom as u64,
            0,
            i64::MAX,
            0,
            0,
            &mut actual_type_return,
            &mut actual_format_return,
            &mut nitems_return,
            &mut bytes_after_return,
            &mut prop_return,
        );

        if r != 0 || actual_format_return == 0 {
            // dbg!(actual_type_return);
            None
        } else {
            Some(
                std::slice::from_raw_parts(
                    mem::transmute::<*mut u8, *const u64>(prop_return),
                    nitems_return as usize,
                )
                .iter()
                .map(|&c| c as u64)
                .collect(),
            )
        }
    }
}

pub fn print_window_attributes(display: *mut xlib::Display, window: Window) {
    unsafe {
        let uninit = MaybeUninit::uninit();
        let mut attributes: xlib::XWindowAttributes = uninit.assume_init();
        let result = xlib::XGetWindowAttributes(display, window, &mut attributes);
        if result == 0 {
            println!("Failed to get window attributes");
        } else {
            println!("Window attributes:");
            println!("x: {}", attributes.x);
            println!("y: {}", attributes.y);
            println!("width: {}", attributes.width);
            println!("height: {}", attributes.height);
            println!("border_width: {}", attributes.border_width);
            println!("depth: {}", attributes.depth);
            // println!("visual: {}", attributes.visual);
            println!("root: {}", attributes.root);
            println!("class: {}", attributes.class);
            println!("bit_gravity: {}", attributes.bit_gravity);
            println!("win_gravity: {}", attributes.win_gravity);
            println!("backing_store: {}", attributes.backing_store);
            println!("backing_planes: {}", attributes.backing_planes);
            println!("backing_pixel: {}", attributes.backing_pixel);
            println!("save_under: {}", attributes.save_under);
            println!("colormap: {}", attributes.colormap);
            println!("map_installed: {}", attributes.map_installed);
            println!("map_state: {}", attributes.map_state);
            println!("all_event_masks: {}", attributes.all_event_masks);
            println!("your_event_mask: {}", attributes.your_event_mask);
            println!(
                "do_not_propagate_mask: {}",
                attributes.do_not_propagate_mask
            );
            println!("override_redirect: {}", attributes.override_redirect);
            // println!("screen: {}", attributes.screen);
        }
    }
}

pub fn get_wm_class(display: *mut xlib::Display, window: Window) -> Option<xlib::XClassHint> {
    unsafe {
        let uninit = MaybeUninit::uninit();
        let mut class_hint: xlib::XClassHint = uninit.assume_init();
        let result = xlib::XGetClassHint(display, window, &mut class_hint);
        if 0 == result {
            None
        } else {
            Some(class_hint)
        }
    }
}

pub unsafe fn get_parent(display: *mut xlib::Display, window: Window) -> Option<Window> {
    let mut root = MaybeUninit::<c_ulong>::uninit();
    let mut parent = MaybeUninit::<c_ulong>::uninit();
    let mut children = MaybeUninit::<*mut c_ulong>::uninit();
    let mut nchildren = MaybeUninit::<c_uint>::uninit();
    if xlib::XQueryTree(
        display,
        window,
        root.as_mut_ptr(),
        parent.as_mut_ptr(),
        children.as_mut_ptr(),
        nchildren.as_mut_ptr(),
    ) == xlib::True
    {
        dbg!(children);
        Some(Window::from(*parent.as_ptr()))
    } else {
        None
    }
}

pub(crate) fn get_atom(display: *mut xlib::_XDisplay, as_ptr: *const i8) -> u64 {
    unsafe { xlib::XInternAtom(display, as_ptr, 0) }
}
