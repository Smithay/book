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

## The four forms of a proxy

A proxy for a protocol object can be manipulated under four forms:

- As direct proxy, as a bare Wayland type, something like `WlFoo`. This form is the
  closest to the protocol object, and its methods map to the requests of the interface of
  this object.
- As a `Proxy<WlFoo>`. In this form, you can manipulate the proxy as a proxy, rather than
  as a protocol object. This lets you do things like check if the underlying object is still
  alive, retrieve its ID, or access the user data associated with it (see below).
- As an `Attached<WlFoo>`. This form is very similar to the bare `WlFoo`, but it additionally
  allows you to send requests that create new objects (doing so with a bare `WlFoo` will panic).
- As a `Main<WlFoo>`. This form is similar to `Attached<WlFoo>`, but additionally allows you to
  change the event handler associated with the Wayland object.

We will soon explain the role of event handlers and `Main<_>` and `Attached<_>` proxies in the
context of event queues, but before that, a quick word about user data.

## User data

`wayland-client` makes it possible to associate some data with a protocol object, via the
[`Proxy<_>::user_data()`](https://docs.rs/wayland-client/*/wayland_client/struct.Proxy.html#method.user_data)
method. This method gives you access to a `&UserData` value, which holds the associated data.
The important part is that *all proxies associated to the same protocol object give you access
to the same `&UserData`*. The value you associate with it is thus attached to the protocol
object, rather than the proxy. This mechanism is largely used across wayland apps, as it is
not rare to have several proxies for the same object.

The [`UserData`](https://docs.rs/wayland-client/*/wayland_client/struct.UserData.html) type is a
set-once style container: you can only set its value once, and then you can get a `&`-reference
to its contents provided you know the type of the stored value. If you need the associated data
to be mutable, you need to handle interior-mutability (by storing a `RefCell` or a `Mutex` for
example).

Lets now discuss event queues.
