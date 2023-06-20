# Objects

The Wayland protocol is an object-oriented protocol. This means that, as the client and
server exchange messages, they build an abstract state made of several objects, which
are represented by a numeric identifier. You can figure them as being scopes for messages:
each message is associated with one of these protocol object. Each object has an "interface",
which is a definition of which messages can be associated with it.

The messages are named "requests" when they are sent by the client to the server, and "events"
when they are sent by the server to the client. An object interface is thus the list of which
requests and which events can be associated with objects of this interface.

For example, an object representing a keyboard would receive the events from the server informing
about which keys are pressed by the user. An object representing a window would send requests
to update its content, and receive events informing about user interaction (such as resizing).

At the beginning, the protocol state contains a single object. Its identifier is 1, and its
interface is `wl_display`. From this object the rest of the protocol state is setup by
the client.

## Objects and proxies

The protocol objects are created and destroyed by messages. A request sent by the client can
have for effect the creation of a new object in the state, or the destruction of an object.
Most of the time, creation and destruction of objects is done by the client, but there are
exceptions. For this reason, protocol objects are not directly represented by Rust structs,
but instead one layer of indirection is added. `wayland-client` provides you with Rust objects
that we call "proxies".

Each proxy represents a protocol object, but several proxies can represent the same object. You
can think of proxies as something akin `Rc<_>` or `Arc<_>`: they are an handle to the underlying
protocol object. Furthermore, their lifetime is not tied to the protocol object's lifetime: they
can outlive it. Once a protocol object has been destroyed, the proxies associated to it will
become inert.

Each kind of protocol object is represented by a different Rust type, all of them implementing the
[`Proxy`] trait. Sending requests to the server is done by invoking methods on that object.

## User data

`wayland-client` makes it possible to associate some data with a protocol object. This data is set
when the object is created, and can be accessed from any proxy representing this object via the
[`Proxy::data()`] method. This user data mechanism comes with two important limitations:

- You can only get an immutable `&` reference to the stored value. This means that if you need this
  value to be writable, you need to handle interior mutability by yourself.
- The [`Proxy::data()`] is generic, and you need to know beforehand what is the type of the stored
  value. If the wrong type is provided as parameter, the function will return `None`.

Lets now discuss the mechanism by which `wayland-client` handles the stream of events from the server
and allows you to process it: event queues and dispatching.

[`Proxy`]: https://docs.rs/wayland-client/latest/wayland_client/trait.Proxy.html
[`Proxy::data()`]: https://docs.rs/wayland-client/latest/wayland_client/trait.Proxy.html#tymethod.data

