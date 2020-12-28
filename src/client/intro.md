# Wayland apps

This part of the book is dedicated to Wayland client apps, so writing programs that can run in
a Wayland environment: display something to the screen, receive user input and process it. This
include classic apps wich display a window with their content, but can also include programs such
as desktop components (widgets, bars, animated backgrounds, lockscreens, ...).

The first section will be dedicated to general principles of the wayland protocol, as well as the
main objects of the [wayland-client](https://crates.io/crates/wayland-client) crate. The following
sections will explore in more details the actual process of writing an app.
