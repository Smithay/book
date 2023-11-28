This is the full example code of the section on creation of a `Display` and event loops. You can read an explanation of the code [here](./creation.md).

```rust
/// Internal compositor state.
///
/// For now this does very little other than keep the `Display` alive and indicate whether the
/// event loop should continue.
#[derive(Debug)]
struct State {
    pub display: Rc<RefCell<Display>>,
    /// Whether the event loop should continue being driven.
    pub continue_loop: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create the display.
    let display = Rc::new(RefCell::new(Display::new()));
    let mut event_loop = calloop::EventLoop::try_new()?;

    // Insert the wayland source so the display may be notified when a client has sent a request.
    insert_wayland_source(event_loop.handle(), &*display.borrow())?;

    // Expose a socket to allow clients to connect to the compositor.
    let socket_name = display
        .borrow_mut()
        .add_socket_auto()
        .expect("Failed to add wayland socket")
        .into_string()
        .unwrap();

    println!("Listening on wayland socket {}", socket_name);
    env::set_var("WAYLAND_DISPLAY", &socket_name);

    let mut state = State {
        display,
        continue_loop: true,
    };

    // Signal used to shut down the event loop.
    let signal = event_loop.get_signal();

    // Run the event loop
    event_loop.run(None, &mut state, |state| {
        if !state.continue_loop {
            // Signal the event loop to stop.
            signal.stop();
            return;
        }

        let display = state.display.clone();
        // Send events to clients.
        // You must flush events to clients or else the clients may never send new requests.
        display.borrow_mut().flush_clients(state);
    })?;

    Ok(())
}

/// Inserts an event source into the event loop that is notified when a client has sent a request.
fn insert_wayland_source(
    handle: calloop::LoopHandle<'static, State>,
    display: &Display,
) -> Result<(), Box<dyn Error>> {
    handle.insert_source(
        calloop::generic::Generic::from_fd(
            display.get_poll_fd(), // The file descriptor which indicates there are pending messages
            calloop::Interest::READ,
            calloop::Mode::Level
        ),
        // This callback is invoked when the poll file descriptor has had activity, indicating there are
        // pending messages.
        move |_, _, state: &mut State| {
            let display = state.display.clone();
            let mut display = display.borrow_mut();

            // Display::dispatch will process any queued up requests and send those events to any objects
            // created on the server.
            if let Err(e) = display.dispatch(Duration::from_millis(0), state) {
                state.continue_loop = false;
                Err(e)
            } else {
                Ok(calloop::PostAction::Continue)
            }
        },
    )?;

    Ok(())
}
```
