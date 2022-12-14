use evdev_rs::enums::EventCode::{EV_KEY, EV_SYN};
use evdev_rs::{InputEvent, ReadFlag};
use super::key_mapper::KeyMapper;
use uinput::device::Device;
use evdev_rs;
use uinput::Error;

pub fn map_events(key_mapper: KeyMapper, naga: evdev_rs::Device, input_device: &mut Device) -> Result<(), String> {
    loop {
        let (_read_status, input_event) = naga.next_event(ReadFlag::NORMAL | ReadFlag::BLOCKING)
            .map_err(|e| format!("Event read error: {}", e))?;

        process_event(key_mapper, input_event, input_device)
            .map_err(|e| format!("Process event error: {}", e))?
    }
}

fn process_event(key_mapper: KeyMapper, event: InputEvent, input_device: &mut Device) -> Result<(), Error> {
    let res = match event.event_code {
        EV_KEY(key) => {
            match key_mapper.map_key(key) {
                Some(mapped_key) => {
                    match event.value {
                        1 => input_device.press(&mapped_key)?,
                        0 => input_device.release(&mapped_key)?,
                        _ => ()
                    }
                }
                None => ()
            }
        }
        EV_SYN(_) => input_device.synchronize()?,
        _ => ()
    };

    Ok(res)
}
