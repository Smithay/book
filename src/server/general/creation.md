# Creating the Display and event loops

wayland-server provides a way to instantiate a `Display` for use by the compositor. Simply calling the `new` function creates a display. However this display is not exposed to clients. To allow clients to connect to the `Display`, a socket needs to be created. A socket may be created through the [`Display::add_socket_auto`] which creates a socket using the first available name.

For compositors which wish to have more control, you may also specify the name of the socket to use with the [`Display::add_socket`], [`Display::add_socket_fd`] or [`Display::add_socket_from`] functions.

```rust,norun
use std::env;
use wayland_server::Display;

let display = Display::new();

let socket_name = display
    .add_socket_auto()
    .expect("Failed to add wayland socket")
    .into_string()
    .unwrap();

println!("Listening on wayland socket {}", socket_name);
env::set_var("WAYLAND_DISPLAY", &socket_name);
```

At this point a client can attempt to connect to the compositor. However you may notice using [wayland-info](https://gitlab.freedesktop.org/wayland/wayland-utils) that when running wayland-info, the application hangs with no response. The cause of this behavior is that the compositor's `Display` does not respond to any requests from the client and wayland-info will wait until it receives a response.

## The event loop

An event loop is the primary way Wayland compositors are driven. Smithay uses [calloop](https://github.com/Smithay/calloop) to listen to the file descriptor created by the the `Display`. 

A `Display` is driven through an `EventLoop` and is inserted as an event source through the event loop's handle. Callbacks are applied when the event loop dispatches events when a source has generated some new events.

Since we will need to provide the `Display` inside the callback to dispatch events, we will need to wrap the `Display` inside an `Rc<RefCell<_>>`. To further simplify future expansion, also create a structure to store state about the compositor:

```rust,no_run
use std::{cell::RefCell, env, rc::Rc, sync::{Arc, atomic::AtomicBool};
use wayland_server::Display;

struct State {
    display: Rc<RefCell<Display>>,
    /// Tracks whether the compositor should continue to run.
    running: Arc<AtomicBool>,
}
```

You will also need to create an event loop:

```rust,no_run
let mut event_loop = EventLoop::try_new().unwrap();
```

Then you will need to initialize an event source so the connection to clients works properly.

```rust,no_run
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

After initializing the event source, make sure to set up the connection inside your `main` function:

```rust,no_run
initialize_connection(&mut state, event_loop.handle());
```

Finally need to drive the event loop by dispatching all pending events to their callbacks using the [`EventLoop::dispatch`] function.

```rust,no_run
// After all the initialization in main(), we can dispatch events.
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
```

At this point, running `wayland-info` again will have the application exit successfully, but print nothing. Don't worry about nothing being printed, as long as the application exits without error you know the server is responding to your clients.

In order for your client to do much more and for `wayland-info` to print any meaningful information about your compositor, you will need to create some globals in the next section.
