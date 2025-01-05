//  maschine.rs: user-space drivers for native instruments USB HIDs
//  Copyright (C) 2015 William Light <wrl@illest.net>
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU Lesser General Public License as
//  published by the Free Software Foundation, either version 3 of the
//  License, or (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU Lesser General Public License for more details.
//
//  You should have received a copy of the GNU Lesser General Public
//  License along with this program.  If not, see
//  <http://www.gnu.org/licenses/>.

mod handler;
mod osc;
mod utils;
extern crate base64;
extern crate image;
extern crate png;
extern crate rusttype;

use std::env;
use std::path::Path;

use std::net::UdpSocket;

extern crate nix;

extern crate hsl;
extern crate tinyosc;

use nix::fcntl::{O_NONBLOCK, O_RDWR};
use nix::{fcntl, sys};

extern crate alsa_seq;
extern crate midi;

use alsa_seq::*;

// use devices::mk2::Mikro;
use handler::MHandler;

mod base;
mod devices;

use base::maschine::ScreenInput;
use base::Maschine;
use utils::{find_hidraw, usage, PAD_RELEASED_BRIGHTNESS};

fn main() {
    // Try to find the hidraw device
    let hidraw_path = match find_hidraw() {
        Ok(Some(path)) => {
            println!("Found Maschine device: {}", path);
            path
        }
        Ok(None) => {
            println!("No Maschine device found.");
            usage(&env::args().next().unwrap());
            panic!("missing hidraw device path");
        }
        Err(err) => {
            usage(&env::args().next().unwrap());
            panic!("error finding hidraw device: {}", err);
        }
    };

    // let args: Vec<_> = env::args().collect();

    // if args.len() < 2 {
    //     usage(&args[0]);
    //     panic!("missing hidraw device path");
    // }

    let dev_fd = match fcntl::open(
        Path::new(&hidraw_path),
        O_RDWR | O_NONBLOCK,
        sys::stat::Mode::empty(),
    ) {
        Err(err) => panic!("couldn't open {}: {}", hidraw_path, err.errno().desc()),
        Ok(file) => file,
    };

    let osc_socket = UdpSocket::bind("127.0.0.1:42434").unwrap();

    let seq_handle = SequencerHandle::open("maschine.rs", HandleOpenStreams::Output).unwrap();
    let seq_port = seq_handle
        .create_port(
            "Pads MIDI",
            PortCapabilities::PORT_CAPABILITY_READ | PortCapabilities::PORT_CAPABILITY_SUBS_READ,
            PortType::MidiGeneric,
        )
        .unwrap();

    let mut device = devices::mk2::Mikro::new(dev_fd);

    let mut handler = MHandler::new(&seq_handle, &seq_port, &osc_socket);

    device.clear_screen();

    // Using a file path
    let file_source = ScreenInput::FilePath("picturetest.png".to_string());
    device.write_screen(file_source);

    device.clear_screen();
    // Using a base64 encoded string
    let base64_data = "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";

    let data_source = ScreenInput::ImageData(base64_data.to_string());
    device.write_screen(data_source);
    device.clear_screen();

    // Using a text string
    let text_source = ScreenInput::Text("Hello, World!".to_string());
    device.write_screen(text_source);
    //
    //
    //Trying to draw stuff here
    // if args.len() < 3 {
    //     device.write_screen();
    // } else {
    //     println!("RUNNING!")
    // }
    //println!("{}", std::env::current_dir().unwrap().display());
    for i in 0..16 {
        device.set_pad_light(i, handler.pad_color(), PAD_RELEASED_BRIGHTNESS);
    }

    handler::ev_loop(&mut device, &mut handler);
}
