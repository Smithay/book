# The registry and globals

The Wayland protocol is designed in a modular fashion: all capabilities proposed by
the server to clients are each represented by modules, that are called "globals" in
the Wayland jargon.

A global is a blueprint advertised by the Wayland server to its clients, that the
client can instantiate into a Wayland protocol object, which will expose the
appropriate requests and events for interacting with the capability they represent.
This process of advertising and instantiating is done with another special
protocol object: the registry.

## The registry

The registry is a protocol object with interface `wl_registry`. It is created from
an attached `wl_display` via its [`get_registry()`] method. Upon creation, the server will send it a stream
of events telling the client about which globals are available.

A global advertisement is composed of 3 values:

- the global name, an `u32` identifier which represents the global within the globals list
  (it is *not* the same thing as the protocol id of the objects created by instantiating this
  global)
- the global interface, a string containing the name of the interface of the protocol objects
  created from this global
- the global version, an `u32` greater or equal to 1, which is used for protocol versioning

The Wayland protocol can evolve, and the interfaces are versioned. The number the server sends
is the highest version it supports. The server must support all lower versions as well.

Upon receiving this list, the client can then instantiate the globals it wishes to use into
protocol objects using the [`bind`] method of the registry.

## The two kinds of globals

The various globals that a Wayland server can advertise can be roughly classified in two kinds,
depending on whether they can be advertised multiple times by the server.

*Singleton globals* represent a capability of the compositor. This is something the Wayland server
makes possible for clients to do. They generally exist from the start, never change and are
advertised only once.

*Multi globals* however represent some device the server has access to. For example a monitor,
or an input device. These globals can thus exist with multiplicity. For example, the
server will advertise one `wl_output` global for each monitor that is plugged in the computer,
each with a different `name`. Furthermore, these globals can appear or disappear during the
lifetime of the app, as such devices are plugged in or out.

The registry `Global` event signals a new global, while its `GlobalRemove` event signals that a
given global has been removed. When a such a global is removed, all the protocol objects derived
from it will generally become inert, and the client is then expected to do cleanup by destroying
them using the appropriate requests.

## Global helpers in `wayland-client`

Tracking the list of globals, their versions, and instantiating them requires some work. `wayland-client`
provides [`GlobalList`] helper that can automate part of this, by providing you with an initial list of
globals at the startup of your app. This allows you proceed to the initialization of your app and state
in a linear fashion, rather than doing so in the callbacks of your [`Dispatch`] implementation for registry.
This does not replace that implementation though, and you still need to provide it to handle dynamic global
creation or destruction.

Using this abstraction, we can put together a small app that connects to the wayland server,
receives the list of globals, and prints them to the console.

The main entry point for using this global helpers is the [`registry_queue_init`] function. This function
takes a reference to your [`Connection`] as argument, an will internally:

1. Create an [`EventQueue`]
2. Create a `wl_registry` object, and do an initial blocking roundtrip with the server to retrieve the list
   of globals
3. Return a [`GlobalList`] containing this list of globals, as well as the created [`EventQueue`] that you can
   then use in your app.

The created registry is registered to that event queue, and a proxy to it can be accessed via the
[`GlobalList::registry()`] method of the returned [`GlobalList`].

```rust,no_run
use wayland_client::{
    Connection, Dispatch, QueueHandle,
    globals::{registry_queue_init, Global, GlobalListContents},
    protocol::wl_registry,
};

// We need a State struct even if we don't use it
struct State;


// You need to provide a Dispatch<WlRegistry, GlobalListContents> impl for your app
impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(
        state: &mut State,
        proxy: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        // The `GlobalListContents` is a container with an up-to-date list of
        // the currently existing globals
        data: &GlobalListContents,
        conn: &Connection,
        qhandle: &QueueHandle<State>,
    ) {
        /*
         * This simple program does not handle dynamic global events,
         * so we don't do anything here.
         */
    }
}

fn main() {
    let connection = Connection::connect_to_env().unwrap();
    let (globals, queue) = registry_queue_init::<State>(&connection).unwrap();

    // Print the contents of the list
    // We cannot iterate the list directly because of thread-safety constraints,
    // so we clone it and iterate on the returned Vec
    for global in globals.contents().clone_list() {
        println!(
            "Global #{} with interface \"{}\" and version {}",
            global.name,
            global.interface,
            global.version
        );
    }
}
```

This first section gave you a general overview of how the Wayland protocol and the `wayland-client` crate
work. In the next section, we'll start to work with SCTK (Smithay's Client ToolKit), a crate designed to
provide several abstractions that simplify all the plumbing needed to create a Wayland app.

[`get_registry()`]: https://docs.rs/wayland-client/latest/wayland_client/protocol/wl_display/struct.WlDisplay.html#method.get_registry
[`bind`]: https://docs.rs/wayland-client/*/wayland_client/protocol/wl_registry/struct.WlRegistry.html#method.bind
[`GlobalList`]: https://docs.rs/wayland-client/latest/wayland_client/globals/struct.GlobalList.html
[`Dispatch`]: https://docs.rs/wayland-client/latest/wayland_client/trait.Dispatch.html
[`registry_queue_init`]: https://docs.rs/wayland-client/latest/wayland_client/globals/fn.registry_queue_init.html