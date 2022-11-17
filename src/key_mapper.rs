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
    match key {
        KEY_1 => Some(0),
        KEY_2 => Some(1),
        KEY_3 => Some(2),
        KEY_4 => Some(3),
        KEY_5 => Some(4),
        KEY_6 => Some(5),
        KEY_7 => Some(6),
        KEY_8 => Some(7),
        KEY_9 => Some(8),
        KEY_0 => Some(9),
        KEY_MINUS => Some(10),
        KEY_EQUAL => Some(11),
        _ => None
    }
}
