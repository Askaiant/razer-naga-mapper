mod input_device;
mod key_mapper;
mod event_mapper;

use std::fs::{File, read_dir};
use evdev_rs::{Device, DeviceWrapper, GrabMode};

// static RAZER_VENDOR_ID: u16 = 0x1532;
// static RAZER_NAGA_PRODUCT_IDS: [u16; 1] = [0x0096];

const VERSION: &'static str = env!("CARGO_PKG_VERSION");


fn get_naga_device() -> Result<Device, String> {
    let paths = read_dir("/dev/input")
        .map_err(|e| format!("Problem reading input devices dir: {}", e)).unwrap();

    for path_result in paths {
        let file = match File::open(path_result.unwrap().path()) {
            Ok(f) => f,
            Err(e) => {
                println!("Error opening file: {}", e);
                continue;
            }
        };

        let mut device = match Device::new_from_file(file) {
            Ok(d) => d,
            Err(e) => {
                println!("Error creating device: {}", e);
                continue;
            }
        };

        if device.name().unwrap_or("").eq("Razer Razer Naga X")
            && device.phys().unwrap().ends_with("/input2") {
            device.grab(GrabMode::Grab).map_err(|e| format!("Could not grab device: {}", e))?;
            return Ok(device);
        }
    }

    return Err("No device found :(".to_string());
}

fn main() {
    println!("Razer naga mapper!, Version: {}\n", VERSION);

    let mapped_keys = key_mapper::KeyMapper::default();

    let mut virtual_device = input_device::create().unwrap();
    loop {
        let device = get_naga_device();

        match device {
            Ok(dev) => {
                let res = event_mapper::map_events(mapped_keys, dev, &mut virtual_device);
                match res.err() {
                    Some(e) => eprintln!("Error mapping events: {}", e),
                    None => eprintln!("Map events returned Ok which was not expected")
                }
            }
            Err(e) => eprintln!("Error getting device: {}", e)
        }
    }
}


