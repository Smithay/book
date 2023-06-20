# Initializing an app

As described previously, once initialized, `wayland-client` provides you with
a [`Connection`] object, representing your wayland connection.

From this connection, we can then recover the [`WlDisplay`] proxy, which represents the initial
`wl_display` object, the starting point of your Wayland interactions.

We also need to create and [`EventQueue`], which will be needed to process the events from all our objects.

The skeleton of a Wayland app may thus look like this:

```rust,no_run
use wayland_client::Connection;

fn main() {
    let connection = Connection::connect_to_env()
                        .expect("Failed to find a Wayland socket.");

    // initialize your State struct
    let my_state = State::new();

    let mut event_queue = connection.new_event_queue();
    let display = connection.display();

    /*
    * Proceed to initialize the rest of your app
    */

    // And the main loop:
    //
    // This assumes that the `state` struct contains an `exit` boolean field,
    // that is set to true when the app decided it should exit.
    while !state.exit {
        event_queue
            .blocking_dispatch(&mut state)
            .expect("Wayland connection lost!");
    }
}
```

With that in place, we can now proceed to the last core concept of the protocol: the globals and the
registry.

[`Connection`]: https://docs.rs/wayland-client/latest/wayland_client/struct.Connection.html
[`WlDisplay`]: https://docs.rs/wayland-client/0.30.2/wayland_client/protocol/wl_display/struct.WlDisplay.html
[`EventQueue`]: https://docs.rs/wayland-client/*/wayland_client/struct.EventQueue.html