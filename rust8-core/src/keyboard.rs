use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

#[derive(Debug)]
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
        self.sender.send(key).expect("the send didnt work, idk when it would fail but i need to add error handing to this");
        self.keypad[key as usize] = true;
    }
    pub fn release_key(&mut self, key: u8) {
        self.keypad[key as usize] = false;
    }

    pub fn new() -> Self {
        let(_sender, _receiver): (Sender<u8>, Receiver<u8>) = mpsc::channel();
        return Keyboard {
            sender: _sender,
            receiver: _receiver,
            keypad: [false; 16]
        }
    }
}
