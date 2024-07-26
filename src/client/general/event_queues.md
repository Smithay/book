# Event queues and dispatching

If the [`Connection`] is the heart of your Wayland app, the [`EventQueue`] will be its
backbone. As described in the previous section, messages are sent in both directions,
and so far we only discussed how to send requests (using the methods of proxies),
but not how to receive events. This is done via a dispatching mechanism powered by the
event queues.

## State and dispatching

Generally, an app built using `wayland-client` will be structured as a central type,
which we refer to as the *state* of the app. This type needs to provide several methods
that are used as callbacks by `wayland-client` to deliver events from the server.

Providing these methods is done by implementing the [`Dispatch`] trait on your state. This
is a parametric trait with two type parameters: the first one is a Wayland interface, and
the second is the type of the user data you want to associate with this kind of object.

For example, to handle events sent to a `WlSurface` object, with user data `MyUserData`, you
would provide the following implementation:

```rust,no_run
impl Dispatch<WlSurface, MyUserData> for Mystate {
    fn event(
        state: &mut Self,
        proxy: &WlSurface,
        event: <WlSurface as Proxy>::Event,
        data: &MyUserData,
        conn: &Connection
        qhandle: &QueueHandle<MyState>
    ) {
        /*
         * Here is your logic for handling the event
         */
    }
}
```

Let's explain the arguments of this method one by one:

1. As you can see, the method does not take `&mut Self` as argument, but instead `state: &mut Self`. This is
   related to a mechanism of `wayland-client` that allows composition by delegating `Dispatch` implementations
   to fields of your state struct. This is detailed below.
2. The second argument is a proxy representing the Wayland object with which this event is associated
3. The first argument is an enum representing the content of the event. You are thus expected to match it
   in order to process it.
4. The fourth argument is a reference to the user data associated with this object.
5. The fifth argument is a reference to the underlying [`Connection`]. Most of the time you will not need to
   use it, but it some circumstances it can be needed.
6. The last argument is a handle to the [`EventLoop`] currently handling this object. You will need it if you
   want to invoke requests that create new objects.
  
Note that the event enums are marked as `#[non_exhaustive]`, as new events can be introduced in later
revisions of the Wayland protocol. The Wayland protocol has a versioning system for objects (which this is
details later in this book), as a result your app will not receive these new events unless you explicitly opt
in by requiring a higher version from the server. Nevertheless, `wayland-client` still needs to statically
support them. These enums being non-exhaustive thus allows the crate to handle new versions of the protocol
without it being a breaking change.

As a result, when you implement the [`Dispatch`] trait you need to keep an eye on which version of the object
you're going to work with, and construct your match accordingly. Having a catch-all `_ => unreachable!()` arm
is here an easy way to ensure you are not ignoring events that you can actually receive.

Another important point to note is that the [`Dispatch`] trait is parameterized by the user data type. This
means that you can provide two different implementations for the same Wayland interface, and the event queue
will invoke one or the other depending on which type of user data was provided when the object was created.

## Event queues

In some cases however, having a single state may be limiting. Some apps have their logic clearly separated in
independent parts, and may want to have these parts run concurrently on multiple threads. To handle this,
`wayland-client` has a concept of [`EventQueue`]. Most apps will be fine with a single event queue, but the
principle is the same with one or more.

Event queues are created from the [`Connection`] with the [`Connection::new_event_queue()`] method. Both this
method and the [`EventQueue`] struct have a type parameter, which is the type of your state struct. This means
that an event queue can only be used with a single type as state. This allows `wayland-client` to statically
ensure that your app provides all the needed [`Dispatch`] implementations.

Once the event queue is created, you can retrieve its handle with the [`EventQueue::handle()`] method. This
handle is required by all methods that create a new Wayland object, and allows you to specify which event queue
(and thus which state) should handle the events for this object.

For a simple app where the Wayland connection is the only source of events to process, the main structure of
the program would then look like this:

```rust,no_run
use wayland_client::Connection;

let connection = Connection::connect_to_env().unwrap();
let mut event_queue = connection.new_event_queue();

/*
 * Here the initialization code of your app...
 */

// And the main loop:
//
// This assumes that the `state` struct contains an `exit` boolean field, that is
// set to true when the app decided it should exit.
while !state.exit {
    event_queue.blocking_dispatch(&mut state).expect("Wayland connection lost!");
}
```

The [`EventQueue::blocking_dispatch()`] method will put your app to sleep and wait until new events from the
server are available, dispatch them to the appropriate [`Dispatch`] methods of the `state`, and then return.

If it returns an error, then the Wayland connection has already been closed and can no longer be used. This
would typically happen in two main situations: either your app triggered a protocol error and the connection
was killed by the server, or the server has shut down.

While this simple structure is sufficient for this introduction, more advanced programs generally don't want
to just sleep waiting for events. For example a game or a video player needs to continue to update its
contents even if no event occurs. [`EventQueue`] provides other methods to dispatch events in a non-blocking
way, see its API documentation for more details.

## Dispatch delegation

`wayland-client` also provides means to provide a generic implementation of [`Dispatch`] that downstream
crates or modules may use in a composition-like fashion. SCTK heavily uses this mechanism to provide generic
and modular helpers, so while you'll probably not need to *implement* such helpers at first, it is important
to have a general idea of how they work to use them.

The general structure of this mechanism is as follows:

1. You provide a sub-state type that contains the data necessary to handle the events of the subset of
   interfaces your helper should manage.
2. You provide generic implementations of the [`Dispatch`] trait on this sub-state type, by explicitly making
   its third type parameter as generic (rather than letting it default to `Self`)
3. The downstream module will then have your sub-state as a field of its state struct, and use the
   [`delegate_dispatch!`] macro to delegate its [`Dispatch`] implementation to your helper.

See the API documentation of [`Dispatch`] and [`delegate_dispatch!`] for more details and examples.

With all this context given, we are now ready to initialize our first app!

[`Connection`]: https://docs.rs/wayland-client/latest/wayland_client/struct.Connection.html
[`EventQueue`]: https://docs.rs/wayland-client/*/wayland_client/struct.EventQueue.html
[`Dispatch`]: https://docs.rs/wayland-client/latest/wayland_client/trait.Dispatch.html
[`Connection::new_event_queue()`]: https://docs.rs/wayland-client/latest/wayland_client/struct.Connection.html#method.new_event_queue
[`EventQueue::handle()`]: https://docs.rs/wayland-client/0.30.2/wayland_client/struct.EventQueue.html#method.handle
[`EventQueue::blocking_dispatch()`]: https://docs.rs/wayland-client/0.30.2/wayland_client/struct.EventQueue.html#method.blocking_dispatch
[`delegate_dispatch!`]: https://docs.rs/wayland-client/0.30.2/wayland_client/macro.delegate_dispatch.html