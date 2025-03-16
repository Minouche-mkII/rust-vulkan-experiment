
/*
 *  Most GLFW APIs calls need to be made from Main Thread
 *  window_manager ensure safety by creating a queue for instructions
 * */

use std::{sync::{atomic::{AtomicBool, Ordering}, Mutex}, thread};

static APPLICATION_IS_RUNNING : AtomicBool = AtomicBool::new(true);
static INSTRUCTIONS_QUEUE : Mutex<Vec<Box<dyn FnOnce(&mut glfw::Glfw) + Send + Sync>>> = Mutex::new(Vec::new());

pub fn start_application<F>(init_instructions: F)
where F: FnOnce() + Send + 'static
{   
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    thread::spawn(init_instructions);

    while APPLICATION_IS_RUNNING.load(Ordering::Relaxed) {
        glfw.poll_events();

        execute_and_clean_queue(&mut glfw);   
    }

    println!("terminated");

    //TODO Take ownership of all windows context to ensure memory cleaning
    
    // Glfw is terminated by being dropped
}

pub fn end_application() {
    APPLICATION_IS_RUNNING.store(false, Ordering::Relaxed);
}

fn execute_and_clean_queue (glfw: &mut glfw::Glfw) {
    let mut queue = INSTRUCTIONS_QUEUE.lock().unwrap();
    while let Some(instruction) = queue.pop() {
        instruction(glfw);
        // TODO Changer en queue
    }
}

fn add_instruction_to_queue<F>(instructions: F)
where F: FnOnce(&mut glfw::Glfw) + Send + Sync + 'static
{
    let mut queue = INSTRUCTIONS_QUEUE.lock().unwrap();
    queue.push(Box::new(instructions));
}

pub fn send_hello() {
    add_instruction_to_queue(|_glfw| {
        println!("hello");
    });
}
