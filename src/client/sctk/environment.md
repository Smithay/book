# The Environment

SCTK provides a system that handles the listing and instanciation of globals
needed by your app: the [`Environment`]. This system is highly modular, but
SCTK also provides a preset for it, that will automatically instanciate all
the globals needed for a regular app. This preset comes in the form of two macros:
[`default_environment!`](https://docs.rs/smithay-client-toolkit/*/smithay_client_toolkit/macro.default_environment.html)
and [`new_default_environment!`](https://docs.rs/smithay-client-toolkit/*/smithay_client_toolkit/macro.new_default_environment.html).

The first one is used to declare the environment struct for your app. We will use the `desktop` preset,
and we need to also provide a name, this example will use `MyApp`, but you can use anything. The second
macro needs to be called for initializing the environment, and we need to give it the `desktop` preset
as well.

The `new_default_environment!` takes care of connecting to the Wayland socket, creating an event queue,
and initializing the environment on it. On success it returns a tuple of 3 values:

- The environment, of type `Environment<MyApp>`
- The `Display`
- The `EventQueue` the environment is associated with

The environment will then provide several methods mapping the functionality of the various underlying
globals. SCTK generally provides an higher-level interface to the underlying functionality than what
the Wayland protocol directly encodes via its globals.

The previous example of listing all globals can be reframed as such using the environment system:

```rust,no_run
use smithay_client_toolkit::{default_environment, new_default_environment};

default_environment!(MyApp, desktop);

fn main() {
    let (environment, display, event_queue) = new_default_environment!(MyApp, desktop)
        .expect("Failed to initialize the Wayland environment.");

    // environment.manager is the underlying GlobalManager
    println!("Available globals:");
    for (name, interface, version) in environment.manager.list() {
        println!("{}: {} (version {})", name, interface, version);
    }
}
```

[`Environment`]: https://docs.rs/smithay-client-toolkit/*/smithay_client_toolkit/environment/struct.Environment.html
