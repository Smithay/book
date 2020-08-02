# The registry and globals

The Wayland protocol is designed in a modular fashion: all capabilities proposed by
the server to clients are each represented by modules, that are called "globals" in
the Wayland jargon.

A global is a blueprint advertized by the Wayland server to its clients, that the
client can instanciate into a Wayland protocol object, which will expose the
appropriate requests and events for interacting with the capability they represent.
This process of advertizing and instanciating is done with an other special
protocol object: the registry.

## The registry

The registry is a protocol object with interface `wl_registry`. It is created from
an attached `wl_display` via its
[`get_registry()`](https://docs.rs/wayland-client/*/wayland_client/protocol/wl_display/struct.WlDisplay.html#method.get_registry)
method. Upon creation, the server will send it a stream of events telling the client
about which globals are available.

A global advertisement is composed of 3 values:

- the global name, an `u32` identifier which represents the global within the globals list
  (it is *not* the same thing as the protocol id of the objects created by instanciating this
  global)
- the global interface, a string containing the name of the interface of the protocol objects
  created from this global
- the global version, an `u32` greater or equal to 1, which is used for protocol versionning

The Wayland protocol can evolve, and the interfaces are versionned. The number the server sends
is the highest version it supports. The server must support all lower versions as well.

Upon receiving this list, the client can then instanciate the globals it whishes to use into
protocol objects using the
[`bind`](https://docs.rs/wayland-client/0.27.0/wayland_client/protocol/wl_registry/struct.WlRegistry.html#method.bind)
method of the registry.

## The two kinds of globals

The various globals that a Wayland server can advertize can be roughtly classified in two kinds,
depending on whether they can be advertized multiple times by the server.

*Singleton globals* represent a capability of the compositor. This is something the Wayland server
makes possible for clients to do. They generally exist from the start, never change and are
advertized only once.

*Multi globals* however represent some device the server has access to. For example a monitor,
or an input device. These globals can thus exist with multiplicity. For example, the
server will advertized one `wl_output` global for each monitor that is plugged in the computer,
each with a different `name`. Furthermore, these globals can appear or dissapear during the
lifetime of the app, as such devices are plugged in or out.

The registry `Global` event signals a new global, while its `GlobalRemove` event signals that a
given global has been removed. When a such a global is removed, all the protocol objects derived
from it will generally become inert, and the client is then expected to do cleanup by destroying
them using the appropriate requests.

## The GlobalManager

Tracking the list of globals, their versions, and instanciating them requires some work that can be
automated away, as such, `wayland-client` provides an abstraction which simplifies this work for you,
the [`GlobalManager`](https://docs.rs/wayland-client/0.27.0/wayland_client/struct.GlobalManager.html).
SCTK further provides other abstractions on top of it, for more convenience, which will be presented
in the following sections. The rest of the client-side half of book will be dedicated to understanding
the different globals and how to use them.

Before jumping to that, lets put all this together with a small app that connects to the wayland server,
receives the list of globals, and prints them to the console:

```rust
use wayland_client::{Display, GlobalManager};

fn main() {
    // Connect to the server
    let display = Display::connect_to_env().unwrap();

    // Create the event queue
    let mut event_queue = display.create_event_queue();
    // Attach the display
    let attached_display = display.attach(event_queue.token());

    // We use the GlobalManager convenience provided by the crate, it covers
    // most classic use cases and avoids us the trouble to manually implement
    // the registry
    let globals = GlobalManager::new(&attached_display);

    // sync_roundtrip is a special kind of dispatching for the event queue.
    // Rather than just blocking once waiting for replies, it'll block
    // in a loop until the server has signalled that it has processed and
    // replied accordingly to all requests previously sent by the client.
    //
    // In our case, this allows us to be sure that after this call returns,
    // we have received the full list of globals.
    event_queue.sync_roundtrip(
        // we don't use a global state for this example
        &mut (),
        // The only object that can receive events is the WlRegistry, and the
        // GlobalManager already takes care of assigning it to a callback, so
        // we cannot receive orphan events at this point
        |_, _, _| unreachable!()
    ).unwrap();

    // GlobalManager::list() provides a list of all globals advertized by the
    // server
    println!("Available globals:");
    for (name, interface, version) in globals.list() {
        println!("{}: {} (version {})", name, interface, version);
    }
}
```