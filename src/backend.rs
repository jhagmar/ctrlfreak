pub mod xcb_backend;

use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Key {
    Alt,
    Ctrl,
    Shift,
    Super,
}

pub type KeyMap = HashMap<Key, bool>;

pub trait Backend {
    fn get(&self) -> Result<KeyMap>;
    fn poll(&self) -> Result<KeyMap>;
}
