/*
 *  Most GLFW APIs calls need to be made from Main Thread
 *  window_manager ensure safety by creating a queue for instructions
 * */

use std::{
    collections::VecDeque,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

use glfw::{GlfwReceiver, PWindow, WindowEvent, WindowMode};

static APPLICATION_IS_RUNNING: AtomicBool = AtomicBool::new(true);
static INSTRUCTIONS_QUEUE: Mutex<VecDeque<Box<dyn FnOnce(&mut glfw::Glfw) + Send + Sync>>> =
    Mutex::new(VecDeque::new());
//static WINDOWS_LIST: Vec<(PWindow, GlfwReceiver<(f64, WindowEvent)>)>;

pub fn start_application<F>(init_instructions: F)
where
    F: FnOnce() + Send + 'static,
{
    // Initiate GLFW API
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    thread::spawn(init_instructions);
    main_loop(&mut glfw);
    println!("terminated");
    //TODO Take ownership of all windows context to ensure memory cleaning
    // Glfw is terminated by being dropped
}

pub fn end_application() {
    APPLICATION_IS_RUNNING.store(false, Ordering::Relaxed);
}

fn main_loop(glfw: &mut glfw::Glfw) {
    while APPLICATION_IS_RUNNING.load(Ordering::Relaxed) {
        glfw.poll_events();
        execute_and_clean_queue(glfw);
    }
}

fn execute_and_clean_queue(glfw: &mut glfw::Glfw) {
    let mut queue = INSTRUCTIONS_QUEUE.lock().unwrap();
    while let Some(instruction) = queue.pop_front() {
        instruction(glfw);
    }
}

fn add_instruction_to_queue<F>(instructions: F)
where
    F: FnOnce(&mut glfw::Glfw) + Send + Sync + 'static,
{
    let mut queue = INSTRUCTIONS_QUEUE.lock().unwrap();
    queue.push_back(Box::new(instructions));
}

fn initiate_window(width: u32, height: u32, title: &str, fullscreen: bool) {
    let title = title.to_string();
    add_instruction_to_queue(move |glfw| {
        glfw.with_primary_monitor(move |glfw, monitor| {
            let mode = {
                if fullscreen && monitor.is_some() {
                    WindowMode::FullScreen(monitor.unwrap())
                } else {
                    WindowMode::Windowed
                }
            };
            glfw.create_window(width, height, &title, mode);
        })
    });
}
