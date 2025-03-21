//! GLFW pointer and associated properties, directrly managed by window manager

use std::collections::HashMap;

use glfw::{GlfwReceiver, PWindow, WindowEvent};


type WindowEventReceiver = GlfwReceiver<(f64, WindowEvent)>;


pub(crate) struct ManagedWindow {
    pointer: PWindow,
    event_receiver: WindowEventReceiver
}

impl ManagedWindow {
    pub(crate) fn new(pointer: PWindow, event_receiver: WindowEventReceiver) -> Self {
        return Self {pointer, event_receiver}
    }
}

pub(crate) struct WindowManager {
    managed_window_list: HashMap<i32, ManagedWindow>,
    last_id: i32
}

impl WindowManager {
    pub(crate) fn new() -> Self {
        let managed_window_list = HashMap::new();
        return Self {managed_window_list, last_id: 0};
    }

    pub(crate) fn add_and_get_id(self: &mut Self, managed_window: ManagedWindow) -> i32 {
        let id = self.last_id;
        self.managed_window_list.insert(id, managed_window);
        self.last_id += 1;
        return id;
    }

    pub(crate) fn remove(self: &mut Self, id: i32) {
        self.managed_window_list.remove(&id);
    }

    pub(crate) fn get(self: &mut Self, id: i32) -> &mut ManagedWindow {
        return self.managed_window_list.get_mut(&id).unwrap();
    }
}
