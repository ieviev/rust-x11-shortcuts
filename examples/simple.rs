use std::{ptr::{self, null_mut}, ffi::{CString, CStr}};

use rustx11shortcuts::*;
use x11::{xlib::{*, self, XGetTextProperty}, xmu::{XmuInternAtom, AtomPtr, _AtomRec}};



fn main() {
    let display = unsafe {xlib::XOpenDisplay(ptr::null())};
    let root = unsafe {xlib::XDefaultRootWindow(display)};
    let mut focuswin = 0;
    let mut focusretn = 0;
    let focus = unsafe {xlib::XGetInputFocus(display, &mut focuswin, &mut focusretn)};
    // get name of net_wm_name
    // let net_wm_name = unsafe {XmuInternAtom(display, CString::new("_NET_WM_NAME").unwrap().as_ptr())};
    
    let mut text_property = XTextProperty {
        value: null_mut(),
        encoding: 0,
        format: 0,
        nitems: 0,
    };
    
    unsafe { 
        XGetWMName(
            display,
            focuswin,
            &mut text_property,
        )
    };

    let text = unsafe { CStr::from_ptr(text_property.value as *mut i8) };
    

    
    // println!("{}", focus_window_name_str);
    println!("display: {:?}", display);
    println!("root: {:?}", root);
    println!("textprop: {:?}", text_property);
    println!("text: {:?}", text);
}



// get input focus window
fn getActiveWindow() -> Option<&'static str> {
    let display = unsafe { XOpenDisplay(ptr::null()) };
    let root = unsafe { XDefaultRootWindow(display) };
    let mut focus_window = 0;
    let mut focus_return = 0;
    let mut focus_mode = 0;
    unsafe {
        
        XGetInputFocus(display, &mut focus_window, &mut focus_return);
    }
    unsafe { XCloseDisplay(display) };

    if focus_window == 0 {
        return None;
    }
    let mut text_property = XTextProperty {
        value: null_mut(),
        encoding: 0,
        format: 0,
        nitems: 0,
    };

    
    unsafe {
        XGetWMName(display, focus_window, &mut text_property);
        
    }
    let text = unsafe { CStr::from_ptr(text_property.value as *mut i8) };
    return Some(text.to_str().unwrap());
    


}


fn shift_f() {
    println!("shift+f");
}

fn ctrl_f() {
    println!("ctrl+f");
}

fn shift_ctrl_f() {
    println!("shift+ctrl+f");
}