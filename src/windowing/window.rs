//! Created windows are centralized within application, when a window is destructed
//! implementations cannot be used anymore after the windows has been killed

use super::application;

pub struct Window {
    managed_window_id: i32 
}

// TODO Handle 
impl Window {
    pub fn new(width: u32, height: u32, title: &str, fullscreen: bool) -> Self {
        let managed_window_id = application::create_new_window(width, height, title, fullscreen);
        Self {
            managed_window_id
        }
    }
    pub fn kill(self: Self) {
        application::destroy_window(self.managed_window_id);
    }
}
