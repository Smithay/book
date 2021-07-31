This is the full example code of the section on creation of a `Display` and event loops. You can read about the full details [here](./creation.md).

```rust,no_run
use std::{
    cell::RefCell,
    env,
    rc::Rc,
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    time::Duration
};
use calloop::{EventLoop, Interest, LoopHandle, Mode, PostAction, generic::Generic},
use wayland_server::Display;

struct State {
    display: Rc<RefCell<Display>>,
    running: Arc<AtomicBool>,
}

fn main() {
    // Create the display.
    let display = Rc::new(RefCell::new(Display::new()));
    // Event loop to drive the display
    let mut event_loop = EventLoop::try_new().unwrap();
    let running = Arc::new(AtomicBool::new(true));

    // Create the state objects to track the server side state.
    let mut state = State {
        display: display.clone(),
        running: running.clone(),
    };

    initialize_connection(&mut state, event_loop.handle());

    let socket_name = state
        .display
        .borrow_mut()
        .add_socket_auto()
        .expect("Failed to add wayland socket")
        .into_string()
        .unwrap();

    println!("Listening on wayland socket {}", socket_name);
    env::set_var("WAYLAND_DISPLAY", &socket_name);

    while state.running.load(Ordering::SeqCst) {
        // Dispatch events so that pending events get processed by their callbacks.
        if event_loop
            .dispatch(Duration::from_millis(16), &mut state)
            .is_err()
        {
            state.running.store(false, Ordering::SeqCst);
        } else {
            // Finally we need to flush events to the clients so clients may respond to server events.
            // Failure to do this will mean the client will wait indefinitely.
            display.borrow_mut().flush_clients(&mut state);
        }
    }
}

fn initialize_connection(state: &mut State, handle: LoopHandle<'static, State>) {
    handle.insert_source(
        Generic::from_fd(state.display.borrow().get_poll_fd(), Interest::READ, Mode::Level),
        move |_, _, state: &mut State| {
            let display = state.display.clone();
            let mut display = display.borrow_mut();

            if let Err(e) = display.dispatch(Duration::from_millis(0), state) {
                state.running.store(false, Ordering::SeqCst);
                Err(e)
            } else {
                Ok(PostAction::Continue)
            }
        }
    )
    .expect("Failed to initialize wayland event source");
}
```
