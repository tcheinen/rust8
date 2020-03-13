use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub struct Keyboard {
    sender: Sender<u8>,
    receiver: Receiver<u8>,
    pub keypad: [bool; 16],
}

impl Keyboard {
    pub fn wait_for_keypress(&self) -> u8 {
        self.receiver.recv().unwrap()
    }

    pub fn press_key(&mut self, key: u8) {
        self.sender.send(key);
        self.keypad[key as usize] = true;
    }
    pub fn release_key(&mut self, key: u8) {
        self.keypad[key as usize] = false;
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        let(_sender, _receiver): (Sender<u8>, Receiver<u8>) = mpsc::channel();
        return Keyboard {
            sender: _sender,
            receiver: _receiver,
            keypad: [false; 16]
        }
    }
}