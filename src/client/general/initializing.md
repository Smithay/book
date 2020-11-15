# Initializing an app

As described previously, once initialized, `wayland-client` provides you with
a [`Display`] object, representing your wayland connection. This is *not* a
proxy for the initial `WlDisplay` protocol object, but it derefs to
`&Proxy<WlDisplay>`, for convenience.

The first things to do are thus to create an event queue and attach the `WlDisplay`
proxy to it, so that objects created from it will be attached to this event queue.

The skeleton of a Wayland app may thus look like this:

```rust,no_run
use wayland_client::Display;

fn main() {
    let display = Display::connect_to_env().expect("Failed to find a Wayland socket.");

    let mut event_queue = display.create_event_queue();
    let attached_display = display.attach(event_queue.token());

    /*
    * Proceed to initialize the rest of your app
    */

    // and the main loop
    loop {
        event_queue.dispatch(&mut global_state, |_,_,_| panic!("Orphan event"))
            .expect("Wayland connection lost.");

        /*
        * Additionnal global processing
        */
    }
}
```

With that in place, we can now proceed to the final core concept of the protocol: the globals
and the registry.

[`Display`]: https://docs.rs/wayland-client/*/wayland_client/struct.Display.html
