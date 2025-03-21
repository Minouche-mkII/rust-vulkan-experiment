/*
 *  Most GLFW APIs calls need to be made from Main Thread
 *  window_manager ensure safety by creating a queue for instructions
 * */

use std::{
    collections::VecDeque,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
};

use glfw::{Glfw, WindowMode};

use super::window_manager::{ManagedWindow, WindowManager};

static APPLICATION_IS_RUNNING: AtomicBool = AtomicBool::new(true);
static INSTRUCTIONS_QUEUE: Mutex<
    VecDeque<Box<dyn FnOnce(&mut Glfw, &mut WindowManager) + Send + Sync>>,
> = Mutex::new(VecDeque::new());

pub fn start_application<F>(init_instructions: F)
where
    F: FnOnce() + Send + 'static,
{
    // Initiate GLFW API
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    let mut window_manager = WindowManager::new();
    thread::spawn(init_instructions);
    main_loop(&mut glfw, &mut window_manager);
    println!("terminated");
    //TODO Take ownership of all windows context to ensure memory cleaning
    // Glfw is terminated by being dropped
}

pub fn end_application() {
    APPLICATION_IS_RUNNING.store(false, Ordering::Relaxed);
}

fn main_loop(glfw: &mut glfw::Glfw, windows_manager: &mut WindowManager) {
    while APPLICATION_IS_RUNNING.load(Ordering::Relaxed) {
        glfw.poll_events();
        execute_and_clean_queue(glfw, windows_manager);
    }
}

fn execute_and_clean_queue(glfw: &mut Glfw, windows_manager: &mut WindowManager) {
    let mut queue = INSTRUCTIONS_QUEUE.lock().unwrap();
    while let Some(instruction) = queue.pop_front() {
        instruction(glfw, windows_manager);
    }
}

fn add_instruction_to_queue<F>(instructions: F)
where
    F: FnOnce(&mut Glfw, &mut WindowManager) + Send + Sync + 'static,
{
    let mut queue = INSTRUCTIONS_QUEUE.lock().unwrap();
    queue.push_back(Box::new(instructions));
}

fn using_managed_window<F>(window_id: i32, instructions : F)
where 
    F: FnOnce(&mut Glfw, &mut ManagedWindow) + Send + Sync + 'static,
{
    add_instruction_to_queue(move |glfw, window_manager| {
        let managed_window = window_manager.get(window_id);
        instructions(glfw, managed_window);
    });
}

// return window id
pub(crate) fn create_new_window(width: u32, height: u32, title: &str, fullscreen: bool) -> i32 {
    let title = title.to_string();
    let (transmitter, receiver) = mpsc::channel::<i32>();
    add_instruction_to_queue(move |glfw, windows_manager| {
        let managed_window = initiate_glfw_window(width, height, &title, fullscreen, glfw);
        let window_id = windows_manager.add_and_get_id(managed_window);
        transmitter.send(window_id).unwrap();
    });
    // TODO main window checking
    return receiver.recv().unwrap();
}

pub(crate) fn destroy_window(window_id: i32) {
    add_instruction_to_queue(move |glfw, managed_windows| {
        managed_windows.remove(window_id);
        // The windows is destroyed when being drop
    });
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
