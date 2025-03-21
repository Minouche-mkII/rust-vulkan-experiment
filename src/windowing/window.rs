//! Created windows are centralized within Window Manager, when a window is destructed
//! implementations cannot be used anymore as the windows as been freed

use std::sync::Weak;

use super::{managed_window::ManagedWindow, window_manager};

pub struct Window {
    managed_window: Weak<ManagedWindow>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str, fullscreen: bool) -> Self {
        let managed_window = window_manager::create_new_window(width, height, title, fullscreen);
        Self {
            managed_window
        }
    }
}
