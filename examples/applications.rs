#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(trait_alias)]
use crate::types::*;
use rustx11shortcuts::*;
use std::{
    fs::File,
    io::{IsTerminal, Read, Write},
    process::Command,
};
use x11::keysym::*;

#[macro_export]
macro_rules! kp {
    ( $sym:expr, $mask:expr ) => {{
        KeyPress::new($sym, $mask)
    }};
}

#[macro_export]
macro_rules! some_kp {
    ( $sym:expr, $mask:expr ) => {{
        Callback::remap(KeyPress::new($sym, $mask))
    }};
}

// remap keypress
#[macro_export]
macro_rules! remap {
    ( $src_k:expr, $src_mask:expr, $tgt_key:expr, $tgt_mask:expr ) => {{
        (
            KeyPress::new($src_k, $src_mask),
            some_kp!($tgt_key, $tgt_mask),
        )
    }};
}

// map keypress to command
#[macro_export]
macro_rules! cmd {
    // command as vec of strs
    ( $sym:expr, $mask:expr, $cmd:expr ) => {{
        (
            KeyPress::new($sym, $mask),
            Callback::command($cmd.iter().map(|s| s.to_string()).collect::<Vec<String>>()),
        )
    }};
}

fn sample_bindings() -> Vec<ApplicationBindings> {
    let b_firefox_aurora: ApplicationBindings = ApplicationBindings {
        wm_class: String::from("firefox-aurora"),
        bindings: vec![
            // remap alt 1-4 to ctrl 1-4
            remap!(XK_1, mask::ALT, XK_1, mask::CTRL),
            remap!(XK_2, mask::ALT, XK_2, mask::CTRL),
            remap!(XK_3, mask::ALT, XK_3, mask::CTRL),
            remap!(XK_4, mask::ALT, XK_4, mask::CTRL),
        ],
    };

    let b_nemo: ApplicationBindings = ApplicationBindings {
        wm_class: String::from("Nemo"),
        bindings: vec![(cmd!(XK_q, mask::ALT, ["notify-send", "Nemo", "Alt+q pressed"]))],
    };
    vec![b_firefox_aurora, b_nemo]
}

fn main() {
    let stdin = std::io::stdin();
    let mut bindings: Vec<ApplicationBindings> = vec![];
    if stdin.is_terminal() {
        println!("using sample bindings");
        bindings.extend(sample_bindings());
    } else {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        let stdin_bindings: Vec<ApplicationBindings> = serde_json::from_str(&buffer).unwrap();
        println!("from json: {buffer}");
        bindings.extend(stdin_bindings);
    }

    let mut handler = Remap::new();
    handler.always_ignore(mask::LOCK | mask::MOD2 | mask::MOD3);

    for app in bindings {
        let wmclass = &app.wm_class;
        println!("registering: {wmclass}");
        handler.register(app);
    }
    handler.listen();
}
