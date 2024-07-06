#![allow(dead_code)]
#![allow(unused_imports)]
extern crate x11;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint};
use std::process::Command;
use x11::xlib::{self, Window};
pub mod types;
use crate::types::*;
mod x;
use x::{pass_through, swallow_event};
pub type Mask = c_uint;

#[inline]
pub fn get_ones_indexes(n: u32) -> Vec<u32> {
    let mut result = vec![];
    let mut n = n; // Clone to mut
    let mut index = 0;
    while n != 0 {
        if n & 1 == 1 {
            result.push(index);
        }

        n >>= 1;
        index += 1;
    }

    result
}

#[inline]
pub fn map_indexes_combination(indexes: &Vec<u32>, combination: u32) -> u32 {
    let mut result = 0;
    let combination_indexes = get_ones_indexes(combination);
    for index in combination_indexes {
        match indexes.get(index as usize) {
            Some(v) => result |= 1 << v,
            None => panic!("Something went wrong mapping a combination"),
        };
    }

    result
}

pub fn total_combinations(indexes_len: usize) -> usize {
    1 << indexes_len
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Callback {
    pub remap: Option<KeyPress>,
    pub command: Option<Vec<String>>,
}

impl Callback {
    pub fn remap(remap: KeyPress) -> Self {
        Callback {
            remap: Some(remap),
            command: None,
        }
    }
    pub fn command(command: Vec<String>) -> Self {
        Callback {
            remap: None,
            command: Some(command),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationBindings {
    pub wm_class: String,
    pub bindings: Vec<(KeyPress, Callback)>,
}

pub struct Remap {
    display: *mut xlib::Display,
    root: xlib::Window,
    always_ignore: c_uint,
    application_bindings: HashMap<String, ApplicationBindings>,
    active_bindings: HashMap<KeyPress, Callback>,
    active_win: u64,
    active_wm_class: String,
    atom_active_window: u64,
}

impl Remap {
    pub fn new() -> Self {
        unsafe {
            let display = x::get_default_display();
            let root = x::get_root(display);
            xlib::XSelectInput(display, root, xlib::PropertyChangeMask);
            let atom_net_active_window_str = CString::new("_NET_ACTIVE_WINDOW").unwrap();
            let atom_active_window = x::get_atom(display, atom_net_active_window_str.as_ptr());
            Remap {
                display,
                root,
                always_ignore: 0,
                active_win: 0,
                application_bindings: HashMap::new(),
                active_wm_class: String::new(),
                active_bindings: HashMap::new(),
                atom_active_window,
            }
        }
    }

    pub fn always_ignore(&mut self, masks: c_uint) {
        self.always_ignore = masks;
    }

    pub fn register(&mut self, app_bindings: ApplicationBindings) {
        for (binding, _) in app_bindings.bindings.iter() {
            let ignore_indexes = get_ones_indexes(self.always_ignore);
            for i in 0..total_combinations(ignore_indexes.len()) {
                let combination = map_indexes_combination(&ignore_indexes, i as u32);
                unsafe {
                    x::grab_with_mask(
                        self.display,
                        self.root,
                        binding.key,
                        binding.mask | combination,
                    );
                }
            }
        }
        self.application_bindings
            .insert(app_bindings.wm_class.clone(), app_bindings);
    }

    fn on_window_change(&mut self, window: xlib::Window) {
        let class = x::get_wm_class(self.display, window);
        class.map(|c| unsafe {
            let active = CStr::from_ptr(c.res_class).to_str().unwrap();
            println!("{active}");
            self.active_wm_class = active.to_string();
            self.active_bindings.clear();
            self.application_bindings.get(active).map(|app| {
                let numbind = app.bindings.len();
                println!("binds: {numbind}");
                for (binding, f) in app.bindings.iter() {
                    self.active_bindings.insert(binding.clone(), f.clone());
                }
            });
        });
    }

    pub fn listen(&mut self) {
        let mut event = xlib::XEvent { pad: [0; 24] };
        loop {
            x::next_event(self.display, &mut event);
            let event_type = event.get_type();
            let mut mask = x::get_state((&mut event).as_mut());
            mask &= !self.always_ignore;

            let callback = match event_type {
                xlib::KeyPress => {
                    let key = x::get_keysym((&mut event).as_mut()) as c_uint;
                    let binding = KeyPress::new(key, mask);
                    self.active_bindings.get(&binding)
                }
                xlib::PropertyNotify => {
                    let pe = xlib::XPropertyEvent::from(event);
                    // dbg!(pe);
                    match pe.atom {
                        _ if pe.atom == self.atom_active_window => {
                            x::get_active_window_id(
                                self.display,
                                self.atom_active_window,
                                pe.window,
                            )
                            .map(|win| {
                                self.on_window_change(win);
                            });
                        }
                        _ => {}
                    }
                    None
                }
                _ => None,
            };
            // dbg!(event_type);
            match callback {
                None => pass_through(self.display, &mut event),
                Some(f) => {
                    f.command.as_ref().map(|args| {
                        // dbg!("executing command");
                        let cmd = &args[0];
                        let arg = &args[1..];
                        Command::new(cmd).args(arg).spawn().unwrap();
                    });
                    match &f.remap {
                        None => swallow_event(self.display, &mut event),
                        Some(k) => {
                            unsafe {
                                x::send_key_event(
                                    self.display,
                                    self.root,
                                    event.key.subwindow,
                                    k.clone(),
                                );
                            }
                            swallow_event(self.display, &mut event)
                        }
                    }
                }
            }
        }
    }
}
