# Creating the Display and event loops

wayland-server provides a way to instantiate a `Display` for use by the compositor. A display may be created using `Display::new`. A display is not immediately is not exposed to clients. To allow clients to connect to the `Display`, a socket needs to be created. A socket may be created through the `Display::add_socket_auto` which creates a socket using the first available name.

For compositors which wish to have more control, you may also specify the name of the socket to use with the `Display::add_socket`, `Display::add_socket_fd` or `Display::add_socket_from` functions.

```rust,norun
use std::env;
use wayland_server::Display;

// Create the display.
let display = Display::new();

// Expose a socket to allow clients to connect to the compositor.
let socket_name = display
    .add_socket_auto()
    .expect("Failed to add wayland socket")
    .into_string()
    .unwrap();

println!("Listening on wayland socket {}", socket_name);
env::set_var("WAYLAND_DISPLAY", &socket_name);
```

At this point a client can attempt to connect to the compositor. However you may notice when using [wayland-info](https://gitlab.freedesktop.org/wayland/wayland-utils) that the application hangs with no response. The cause of this behavior is that the compositor's `Display` does not process any client requests or respond with events being sent to the clients unless the display is driven.

## The event loop

An event loop is the primary way Wayland compositors are driven. Smithay uses [calloop](https://github.com/Smithay/calloop) to listen to the file descriptor created by the the `Display`.

The Smithay book only covers some aspects of calloop at a very high level, see the [calloop book](https://smithay.github.io/calloop/) for more details.

A `Display` is driven using an `calloop::EventLoop`. Callbacks are dispatched when an event source has pending events. An event loop may be created using `calloop::EventLoop::try_new()`.

Since calloop passes around a state object that may be used to store a compositor's internal state, it is best to define a structure to hold compositor state.

```rust,no_run
use std::{cell::RefCell, env, rc::Rc, sync::{Arc, atomic::AtomicBool};
use wayland_server::Display;

struct State {
    display: Rc<RefCell<Display>>,
    /// Whether the event loop should continue being driven.
    continue_loop: bool,
}
```

Since we will need to provide the display inside the callback to dispatch events but also allow the state to access the display inside callbacks, we wrap the display inside an `Rc<RefCell<_>>` to allow access to the display inside compositor state and callbacks.

Now that we can drive the event loop and provide access to the display while dispatching events on the event loop, let us now handle requests from clients. The function below will create an event source to be inserted in the event loop.

```rust,no_run
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
        // This callback is invoked when the poll file descriptor has had activity, indicating there are pending messages.
        move |_, _, state: &mut State| {
            let display = state.display.clone();
            let mut display = display.borrow_mut();

            // Display::dispatch will process any queued up requests and send those events to any objects created on the server.
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

And then invoke the function to listen to client requests.

```rust,no_run
insert_wayland_source(&mut state, event_loop.handle());
```

Finally run the event loop to start the compositor.

```rust,no_run
// Signal used to shut down the event loop.
let signal = event_loop.get_signal();

// Event loop
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
```

At this point, running `wayland-info` again will have the application exit successfully, but print nothing. Don't worry about nothing being printed, as long as the application exits without error you know the server is responding to your clients.

The next step towards displaying something on screen in a compositor is creating some globals to be advertised to clients in the next section.
