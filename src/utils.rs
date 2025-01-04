use midi::*;
use std::fs;
use std::io;
use std::path::Path;

pub const PAD_RELEASED_BRIGHTNESS: f32 = 0.015;

#[allow(dead_code)]
pub enum PressureShape {
    Linear,
    Exponential(f32),
    Constant(f32),
}

pub const PAD_NOTE_MAP: [U7; 16] = [12, 13, 14, 15, 8, 9, 10, 11, 4, 5, 6, 7, 0, 1, 2, 3];

pub fn usage(prog_name: &String) {
    println!("usage: {} <hidraw device>", prog_name);
}

pub fn find_hidraw() -> io::Result<Option<String>> {
    for entry in fs::read_dir("/dev")? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().and_then(|f| f.to_str());

        if let Some(name) = file_name {
            if name.starts_with("hidraw") {
                let uevent_path = Path::new("/sys/class/hidraw")
                    .join(name)
                    .join("device")
                    .join("uevent");

                if let Ok(uevent_contents) = fs::read_to_string(&uevent_path) {
                    for line in uevent_contents.lines() {
                        if line.starts_with("HID_NAME=") {
                            let device_name = &line[9..]; // Equivalent to trim_start_matches("HID_NAME=")
                            if device_name.contains("Maschine") {
                                return Ok(Some(path.to_string_lossy().into_owned()));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}
