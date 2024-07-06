use std::{ptr::{self, null_mut, null}, ffi::{CString, CStr}, borrow::BorrowMut};

use rustx11shortcuts::*;
use std::os::raw::{c_int, c_uint};
use x11::{xlib::{*, self, XGetTextProperty}, xmu::{XmuInternAtom, AtomPtr, _AtomRec}};

// helpers

pub unsafe fn get_default_display() -> *mut xlib::Display {
    xlib::XOpenDisplay(ptr::null())
}

pub unsafe fn get_root(display: *mut xlib::Display) -> xlib::Window {
    xlib::XDefaultRootWindow(display)
}

pub unsafe fn next_event(display: *mut xlib::Display, event: &mut xlib::XEvent) {
    xlib::XNextEvent(display, event);
}

pub fn get_keysym(press: &mut xlib::XKeyEvent) -> xlib::KeySym {
    unsafe { xlib::XLookupKeysym(press, 0) }
}

pub fn get_state(press: &mut xlib::XKeyEvent) -> c_uint {
    press.state
}

fn listen(display: *mut _XDisplay) {
    let mut event = xlib::XEvent { pad: [0; 24] };
    let root = unsafe {xlib::XDefaultRootWindow(display)};
    let mut swa: XSetWindowAttributes = XSetWindowAttributes { 
        background_pixmap: 0,
        background_pixel: 0,
        border_pixmap: 0,
        border_pixel: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0, 
        backing_pixel: 0, 
        save_under: 0, 
        event_mask: xlib::FocusChangeMask, 
        do_not_propagate_mask: 0, 
        override_redirect: 0, 
        colormap: 0, 
        cursor: 0 
    };
    unsafe {
        // subscribe to events
        let a = 1;
        
        // xlib_binding::grab_with_mask(self.display, self.root, key, mask | combination);
        let result = xlib::XChangeWindowAttributes(
            display, 
            root,
            0, // valuemask
            swa.borrow_mut()
        );
        let b = 1;
    }
    loop {
        println!("hello");
        unsafe {
            next_event(display, &mut event);
        }
        println!("got_ev");
        let event_type = event.get_type();
        // let trigger = Trigger::from_type(event_type);
        // let key = key_upper(xlib_binding::get_keysym((&mut event).as_mut()) as c_uint);
        // let mut mask = xlib_binding::get_state((&mut event).as_mut());
        // Remove ignored masks
        // mask &= !self.always_ignore;

        // let binding = Binding::new(key, trigger, mask);

        let callback = match event_type {
            xlib::FocusIn => { println!("evtype1: {:?}", event_type); Some(1)},
            // xlib::KeyPress => self.press_bindings.get(&binding),
            // xlib::KeyRelease => self.release_bindings.get(&binding),
            _ => {
                println!("evtype: {:?}", event_type);
                None
            }
        };

        // if let Some(f) = callback {
        //     f();
        // }
    }
}


fn main() {
    let display = unsafe {xlib::XOpenDisplay(ptr::null())};
    // let root = unsafe {xlib::XDefaultRootWindow(display)};
    let mut focuswin = 0;
    let mut focusretn = 0;
    // let focus = unsafe {xlib::XGetInputFocus(display, &mut focuswin, &mut focusretn)};
    listen(display);
    
        
    let a = 1;
        
  
    
    // get name of net_wm_name
    // let net_wm_name = unsafe {XmuInternAtom(display, CString::new("_NET_WM_NAME").unwrap().as_ptr())};
    
    // let mut text_property = XTextProperty {
    //     value: null_mut(),
    //     encoding: 0,
    //     format: 0,
    //     nitems: 0,
    // };
    
    // unsafe { 
    //     XGetWMName(
    //         display,
    //         focuswin,
    //         &mut text_property,
    //     )
    // };

    // let text = unsafe { CStr::from_ptr(text_property.value as *mut i8) };
    

    
    // // println!("{}", focus_window_name_str);
    // println!("display: {:?}", display);
    // println!("root: {:?}", root);
    // println!("textprop: {:?}", text_property);
    // println!("text: {:?}", text);
}


