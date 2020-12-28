# Exercise: an image viewer

This exercise aims at allowing you to put together all the things we have seen in this introduction
to SCTK. The goal is to write an app that loads an image from disk and displays it as the only
content of its window. It should react to resizing by stretching the image so that it fills the
size given to the window.

## Using the `image` crate

Manipulating images is not the main goal of this exercise, so you can just use the `image` crate to
do it for you. In particular:

- [`image::open`](https://docs.rs/image/0.23.8/image/fn.open.html) can load an image file from disk
- [`image::imageops::resize`](https://docs.rs/image/0.23.8/image/imageops/fn.resize.html) can be used
  to resize the image accordingly

## General advice

To organize your program, you'll probably find it useful to split the actual drawing code into its
own function, taking as argument the `&mut MemPool` to use, the `&WlSurface` to which commit the buffer,
the requested dimensions, and the image.

The `xdg_shell` global, used by `Window`, requires you to wait until you have received at least one
`Configure` event before drawing your content. Committing a buffer before you've received it will be
considered as a protocol error by the server, which will cause it to kill your connection.

Be careful about the pixel format, `image` provides you with the pixels in RGBA format, but the Wayland
server will expect them as ARGB.

## Solution

A solution to this exercise with detailed comments can be found as the
[`image_viewer.rs` example of SCTK](https://github.com/Smithay/client-toolkit/blob/master/examples/image_viewer.rs).
