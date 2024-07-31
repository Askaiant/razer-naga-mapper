use evdev_rs::enums::EV_KEY;
use evdev_rs::enums::EV_KEY::*;
use uinput::event::keyboard::Key;


#[derive(Copy, Clone)]
pub struct KeyMapper {
    keys: [Key; 12]
}

const KEYSET_1: [Key; 12] = [Key::F1, Key::F2, Key::F3, Key::F4, Key::F5, Key::F6, Key::F7, Key::F8, Key::F9, Key::F10, Key::Minus, Key::Equal];
#[allow(dead_code)]
const KEYSET_2: [Key; 12] = [Key::Home, Key::Up, Key::PageUp, Key::Left, Key::Delete, Key::Right, Key::End, Key::Down, Key::PageDown, Key::Insert, Key::Minus, Key::Equal];

impl KeyMapper {
    pub fn default() -> KeyMapper {
        KeyMapper {
            keys: KEYSET_1
        }
    }

    pub fn map_key(self, key: EV_KEY) -> Option<Key> {
        return key_index(key)
            .and_then(|i| self.keys.get(i))
            .map(|k| *k);
    }
}

fn key_index(key: EV_KEY) -> Option<usize> {
    println!("{:?}", key);
    match key {
        KEY_KP1 => Some(0),
        KEY_KP2 => Some(1),
        KEY_KP3 => Some(2),
        KEY_KP4 => Some(3),
        KEY_KP5 => Some(4),
        KEY_KP6 => Some(5),
        KEY_KP7 => Some(6),
        KEY_KP8 => Some(7),
        KEY_KP9 => Some(8),
        KEY_KP0 => Some(9),
        KEY_MINUS => Some(10),
        KEY_EQUAL => Some(11),
        _ => None
    }
}
