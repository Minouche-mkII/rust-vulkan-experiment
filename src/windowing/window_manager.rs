/*
 *  Most GLFW APIs calls need to be made from Main Thread
 *  window_manager ensure safety by creating a queue for instructions
 * */

use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex, Weak,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
};

use glfw::{Glfw, WindowMode};

use super::managed_window::ManagedWindow;

static APPLICATION_IS_RUNNING: AtomicBool = AtomicBool::new(true);
static INSTRUCTIONS_QUEUE: Mutex<
    VecDeque<Box<dyn FnOnce(&mut Glfw, &mut ManagedWindowList) + Send + Sync>>,
> = Mutex::new(VecDeque::new());

type ManagedWindowList = Vec<Arc<ManagedWindow>>;

pub fn start_application<F>(init_instructions: F)
where
    F: FnOnce() + Send + 'static,
{
    // Initiate GLFW API
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    let mut windows_list: ManagedWindowList = Vec::new();
    thread::spawn(init_instructions);
    main_loop(&mut glfw, &mut windows_list);
    println!("terminated");
    //TODO Take ownership of all windows context to ensure memory cleaning
    // Glfw is terminated by being dropped
}

pub fn end_application() {
    APPLICATION_IS_RUNNING.store(false, Ordering::Relaxed);
}

fn main_loop(glfw: &mut glfw::Glfw, windows_list: &mut ManagedWindowList) {
    while APPLICATION_IS_RUNNING.load(Ordering::Relaxed) {
        glfw.poll_events();
        execute_and_clean_queue(glfw, windows_list);
    }
}

fn execute_and_clean_queue(glfw: &mut Glfw, windows_list: &mut ManagedWindowList) {
    let mut queue = INSTRUCTIONS_QUEUE.lock().unwrap();
    while let Some(instruction) = queue.pop_front() {
        instruction(glfw, windows_list);
    }
}

fn add_instruction_to_queue<F>(instructions: F)
where
    F: FnOnce(&mut Glfw, &mut ManagedWindowList) + Send + Sync + 'static,
{
    let mut queue = INSTRUCTIONS_QUEUE.lock().unwrap();
    queue.push_back(Box::new(instructions));
}

pub(crate) fn create_new_window(width: u32, height: u32, title: &str, fullscreen: bool) -> Weak<ManagedWindow> {
    let title = title.to_string();
    let (transmitter, receiver) = mpsc::channel::<Weak<ManagedWindow>>();
    add_instruction_to_queue(move |glfw, windows_list| {
        let managed_window = Arc::new(initiate_glfw_window(
            width, height, &title, fullscreen, glfw,
        ));
        let weak_pointer = Arc::downgrade(&managed_window);
        windows_list.push(managed_window);
        transmitter.send(weak_pointer).unwrap();
    });
    // TODO main window checking
    return receiver.recv().unwrap();
}

///
/// Create a new GLFW window
/// Returns the pointer and event handler from the new window
///
fn initiate_glfw_window(
    width: u32,
    height: u32,
    title: &str,
    fullscreen: bool,
    glfw: &mut Glfw,
) -> ManagedWindow {
    return glfw.with_primary_monitor(move |glfw, monitor| {
        let mode = {
            if fullscreen && monitor.is_some() {
                WindowMode::FullScreen(monitor.unwrap())
            } else {
                WindowMode::Windowed
            }
        };
        let (pointer, event_receiver) = glfw.create_window(width, height, &title, mode).unwrap();
        return ManagedWindow::new(pointer, event_receiver);
    });
}
