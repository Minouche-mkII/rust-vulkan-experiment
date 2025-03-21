//! GLFW pointer and associated properties, directrly managed by window manager

use glfw::{GlfwReceiver, PWindow, WindowEvent};

type WindowEventReceiver = GlfwReceiver<(f64, WindowEvent)>;

/*
 * WARNING : VERY THREAD UNSAFE
 * Should only manipulate theses attributes within closure 
 * in WindowManager::add_instruction_to_queue
 * */
pub(crate) struct ManagedWindow {
    pointer: PWindow,
    event_receiver: WindowEventReceiver
}

/* Is unsafe because the pointer can be used on other thread but the references will only
   be used in the same thread thanks to the queue*/
impl ManagedWindow {
    pub(crate) fn new(pointer: PWindow, event_receiver: WindowEventReceiver) -> Self {
        return Self {pointer, event_receiver}
    }
}

unsafe impl Sync for ManagedWindow {}
