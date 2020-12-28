# Getting started with SCTK

In the previous section we explored the general structure of the Wayland protocol,
as well as how a Wayland app is initialized using the display and the registry to
get the list of globals.

We will now introduce [SCTK](https://docs.rs/smithay-client-toolkit) (short for
*Smithay Client Toolkit*), a toolkit crate that provides various abstractions to
simplify large parts of the plumbing required to make a Wayland app. This section
will cover the steps to setup an app using SCTK, create a window and draw to it.
