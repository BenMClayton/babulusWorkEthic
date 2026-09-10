use glfw::{Action, Context, Key};
use std::time::{Duration, Instant};

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).expect("GLFW could not be initialised");

    let (mut window, events) = glfw
        .create_window(520, 220, "Babulus Work Ethic", glfw::WindowMode::Windowed)
        .expect("A GLFW window could not be created");

    window.make_current();
    window.set_key_polling(true);
    window.set_close_polling(true);

    let started_at = Instant::now();
    let mut last_title_update = Instant::now();
    let mut key_presses = 0_u64;

    while !window.should_close() {
        window.swap_buffers();
        glfw.wait_events_timeout(1.0 / 60.0);

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                glfw::WindowEvent::Key(_, _, Action::Press, _) => {
                    key_presses += 1;
                }
                _ => {}
            }
        }

        if last_title_update.elapsed() >= Duration::from_secs(1) {
            let elapsed = started_at.elapsed().as_secs();
            window.set_title(&format!(
                "Babulus Work Ethic | active {:02}:{:02} | key presses {}",
                elapsed / 60,
                elapsed % 60,
                key_presses
            ));
            last_title_update = Instant::now();
        }
    }

    println!(
        "Session complete: {} seconds active, {} key presses inside the app window.",
        started_at.elapsed().as_secs(),
        key_presses
    );
}
