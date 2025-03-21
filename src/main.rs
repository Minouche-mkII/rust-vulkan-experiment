
mod windowing;
use std::{thread, time::Duration};

use windowing::{window::{self, Window}, window_manager};

fn main() {
    window_manager::start_application(|| {
        Window::new(500, 500, "Hello World", false);
    });
}
