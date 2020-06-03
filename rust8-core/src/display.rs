use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;


pub struct Display {
    sender: Sender<[bool; 64 * 32]>,
    receiver: Receiver<[bool; 64 * 32]>,
    pub vram: [bool; 64 * 32],
    pub dirty: bool,
}

impl Display {
    pub fn receive_change(&mut self) -> [bool; 64 * 32] {
        let resp = self.receiver.recv().unwrap();
        self.dirty = false;
        resp
    }

    pub fn update_display(&mut self) {
        self.sender.send(self.vram).expect("the send didnt work, idk when it would fail but i need to add error handing to this");
        self.dirty = true;
    }

    pub fn new() -> Self {
        let(_sender, _receiver): (Sender<[bool; 64 * 32]>, Receiver<[bool; 64 * 32]>) = mpsc::channel();
        return Display {
            sender: _sender,
            receiver: _receiver,
            vram: [false; 64 * 32],
            dirty: false,
        }
    }

}
