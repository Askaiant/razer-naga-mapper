use evdev_rs::enums::EV_KEY;
use evdev_rs::enums::EV_KEY::*;
use uinput::event::keyboard::Key;
use uinput::event::keyboard::KeyPad;
use uinput::event::Keyboard;

#[derive(Copy, Clone)]
pub struct KeyMapper {
    keys: [Keyboard; 12],
}

#[allow(dead_code)]
const KEYSET_1: [Keyboard; 12] = [
    Keyboard::Key(Key::F1),
    Keyboard::Key(Key::F2),
    Keyboard::Key(Key::F3),
    Keyboard::Key(Key::F4),
    Keyboard::Key(Key::F5),
    Keyboard::Key(Key::F6),
    Keyboard::Key(Key::F7),
    Keyboard::Key(Key::F8),
    Keyboard::Key(Key::F9),
    Keyboard::Key(Key::F10),
    Keyboard::Key(Key::Minus),
    Keyboard::Key(Key::Equal),
];

#[allow(dead_code)]
const KEYSET_2: [Keyboard; 12] = [
    Keyboard::Key(Key::Home),
    Keyboard::Key(Key::Up),
    Keyboard::Key(Key::PageUp),
    Keyboard::Key(Key::Left),
    Keyboard::Key(Key::Delete),
    Keyboard::Key(Key::Right),
    Keyboard::Key(Key::End),
    Keyboard::Key(Key::Down),
    Keyboard::Key(Key::PageDown),
    Keyboard::Key(Key::Insert),
    Keyboard::Key(Key::Minus),
    Keyboard::Key(Key::Equal),
];

const KEYSET_3: [Keyboard; 12] = [
    Keyboard::KeyPad(KeyPad::_1),
    Keyboard::KeyPad(KeyPad::_2),
    Keyboard::KeyPad(KeyPad::_3),
    Keyboard::KeyPad(KeyPad::_4),
    Keyboard::KeyPad(KeyPad::_5),
    Keyboard::KeyPad(KeyPad::_6),
    Keyboard::KeyPad(KeyPad::_7),
    Keyboard::KeyPad(KeyPad::_8),
    Keyboard::KeyPad(KeyPad::_9),
    Keyboard::KeyPad(KeyPad::_0),
    Keyboard::KeyPad(KeyPad::Minus),
    Keyboard::KeyPad(KeyPad::Equal),
];
impl KeyMapper {
    pub fn default() -> KeyMapper {
        KeyMapper { keys: KEYSET_3 }
    }

    pub fn map_key(self, key: EV_KEY) -> Option<Keyboard> {
        key_index(key).and_then(|i| self.keys.get(i)).map(|k| *k)
    }
}

fn key_index(key: EV_KEY) -> Option<usize> {
    if cfg!(debug_assertions) {
        println!("{:?}", key);
    }
    match key {
        KEY_KP1 | KEY_1 => Some(0),
        KEY_KP2 | KEY_2 => Some(1),
        KEY_KP3 | KEY_3 => Some(2),
        KEY_KP4 | KEY_4 => Some(3),
        KEY_KP5 | KEY_5 => Some(4),
        KEY_KP6 | KEY_6 => Some(5),
        KEY_KP7 | KEY_7 => Some(6),
        KEY_KP8 | KEY_8 => Some(7),
        KEY_KP9 | KEY_9 => Some(8),
        KEY_KP0 | KEY_0 => Some(9),
        KEY_MINUS => Some(10),
        KEY_EQUAL => Some(11),
        _ => None,
    }
}
