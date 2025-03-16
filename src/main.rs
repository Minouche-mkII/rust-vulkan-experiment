
mod windowing;
use std::{thread, time::Duration};

use windowing::window_manager;

fn main() {
    window_manager::start_application(|| {
        // initialisation
        window_manager::send_hello();
        thread::sleep(Duration::from_millis(2000));
        window_manager::end_application();
    });
}
