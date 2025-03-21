
mod windowing;

use windowing::{application, window::Window, window_manager};

fn main() {
    application::start_application(|| {
        Window::new(500, 500, "Hello World", false).kill();
    });
}
