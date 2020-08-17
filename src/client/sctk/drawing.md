# Drawing to a Window

Drawing to a surface is done by attaching buffers to it, though the [`WlSurface::attach()`] method.
This method takes as argument an optional [`WlBuffer`], and coordinates of this buffer relative to the
current content. This last bit allows to control in which direction the rectangle of the surface should
grow or shrink, but is pretty anecdotical. Most of the tme you will just set these to `(0, 0)`.

The `Option<&WlBuffer>` is the main part of drawing. A buffer defines the totality of the contents of
a surface, including its size. It is mostly a reference to an array of pixels that the Wayland server
will use to fill the surface contents. Updating the contents of a surface amounts to attaching a new
buffer to it, replacing the previous one. Attaching `None` to a surface erases its content, causing the
surface to be hidden by the server.

The act of drawing is this creating a [`WlBuffer`] refering to an array of pixels with the appropriate
content. There are two main approaches for producing such a buffer: drawing to shared memory, or via
OpenGL/Vulkan. We will now focus on the first method, leaving OpenGL and Vulkan for later.

## Shared Memory

The principle of shared memory for drawing content is that the client first creates a memory-backed file
(for example using [`memfd_create`]), and shares its file descriptor with the server. The client can
then draw its content by writing to the file, and creates buffer pointing to the appropriate part of the
file, similarly to a rust slice pointing to part of a `Vec`. The client can thus write the content of
several different buffers to the same file, avoiding the need to open many file descriptors (file descriptor
leaks are a real thing!).

When a buffer has been attached to a surface and submitted to the compositor, its associated pixel array
should no longer be changed until it has finished reading it, or graphical glitches may occur. As a result,
clients are encouraged to do double-buffering: maintaining two shared memory files, and drawing to one while
the other is in use by the server.

As one can expect, the capability to create buffers backend by shared memory is represented, by a global:
[`wl_shm`]. However as previously, SCTK provides an abstraction to make handling such shared memory easier:
the [`DoubleMemPool`]. As its name can let you guess, it also manages double-buffering.

We can create a [`DoubleMemPool`] from the environment using the [`Environment::create_double_pool()`] method:

```rust,no_run
let double_pool = environment.create_double_pool(|_| {})
    .expect("Failed to create a memory-backed file.");
```

The [`DoubleMemPool`] keeps track automatically of which of its underlying memory pool is still being used by the
server, allowing us to use the other. If at some point we try to draw and both are still in use, [`DoubleMemPool`]
will not let us access any pool. If that happens, the callback we gave to [`Environment::create_double_pool()`] will
be invoked as soon as one pool is free again and we can draw. We will not be doing such reactive drawing in this
example, so we just pass an empty callback.

## Drawing on the memory pool

When we want to draw, we can start by invoking [`DoubleMemPool::pool()`], which gives us access to a free
[`MemPool`] or `None` if both are currently in use. Once we have it, we can write to it like any file, it
implements the [`std::io::Write`] and [`std::io::Seek`] traits.

For this example, let's just fill the window with red. We will be writing the pixel contents in ARGB8888 format.
This means that each pixel will be represented by 4 bytes: Alpha, Red, Green and Blue values. Our fully opaque
red is thus `#FFD00000`, or `&[0xFF, 0xD0, 0, 0]`.

To draw this content, we shall first ensure that the [`MemPool`] is large enough to store those contents,
then seek back to the beginning of the file, and write enough of these pixels to fill the whole surface.

```rust,no_run
use std::io::{Write, Seek, SeekFrom, BufWriter}
// Only try to draw if there is a free pool
if let Some(pool) = double_pool.pool() {
    // number of pixels in the surface
    let pxcount = width * height;
    // number of bytes corresponding to these pixels
    let bytecount = 4*pxcount;

    // Resize the pool accordingly. It is important to use this method
    // rather than just let the memory file grow by writing to it, because
    // it tells the server about the new size of the file. Note that a
    // memory pool can never shrink, so if the size we request is smaller than
    // its current size, this call will do nothing.
    pool.resize(bytecount).unwrap();

    // Now, seek to the beggining of the memory file, to overwrite its contents
    pool.seek(SeekFrom::Start(0)).unwrap();

    // Finally do the actual drawing. We use a BufWriter to increase performance
    {
        let mut writer = BufWriter::new(&mut *pool);
        for _ in 0..pxcount {
            writer.write_all(&[0xFF, 0xD0, 0x00, 0x00]).unwrap();
        }
        writer.flush().unwrap();
    }
}
```

## Creating and attaching the buffer

Once the [`MemPool`] contains the correct pixels, we can create a buffer from it using the [`MemPool::buffer()`] method.
This method requires several arguments to correctly define the buffer relative to the memory pool:

- `offset`: how many bytes from the start of the pool does the buffer start
- `width`: the width of the buffer in pixels
- `height`: the height of the buffer in pixels
- `stride`: the number of bytes between the start of each line
- `format`: the pixel format used for interpreting the bytes (in our case `Format::Argb8888`)

The pixel stream from the memory pool is thus interpreted by the server as one line after another, from left to right
and from top to bottom. In our case, the `offset` is 0, as we wrote our content at the start of the pool. The width
and height are the dimensions of the content we've drawn. Here `stride` is just `width * 4`, as there are 4 bytes per pixel.

We can thus create our buffer using:

```rust,no_run
use smithay_client_toolkit::wl_shm::Format;

let buffer = pool.buffer(0, width, height, 4*height, Format::Argb8888);
```

And finally, the last remaining thing to do is to attach this buffer to the surface, declare the damage, and commit the surface.
Declaring the damage tells the server which part of the content actually changed since the last submitted buffer. This allows
it to optimize its drawing process to only update parts of the screen that need updating. For this example, we will simply
declare the whole buffer as damaged.

```rust,no_run
surface.attach(Some(&buffer), 0, 0);
surface.damage_buffer(0, 0, width, height);
surface.commit();
```

The `commit()` call is necessary because the state of the surface is double-buffered. We are changing two properties of the
surface here: its attached buffer, and the parts that are damaged. Once we've sent these requests the server will keep these
new properties on hold, and apply them atomically when we send the `commit` request.

With this, you are now able to display content on a Wayland window, congratulations! To wrap all this together, the next
page will be an exercice to code a simple image viewer app: it'll display an image and stretch its content when resized.

[`WlSurface::attach()`]: https://docs.rs/wayland-client/*/wayland_client/protocol/wl_surface/struct.WlSurface.html#method.attach
[`WlBuffer`]: https://docs.rs/wayland-client/*/wayland_client/protocol/wl_buffer/index.html
[`memfd_create`]: https://man7.org/linux/man-pages/man2/memfd_create.2.html
[`wl_shm`]: https://docs.rs/wayland-client/*/wayland_client/protocol/wl_shm/index.html
[`DoubleMemPool`]: https://docs.rs/smithay-client-toolkit/0.10.0/smithay_client_toolkit/shm/struct.DoubleMemPool.html
[`Environment::create_double_pool()`]: https://docs.rs/smithay-client-toolkit/0.10.0/smithay_client_toolkit/environment/struct.Environment.html#method.create_double_pool
[`DoubleMemPool::pool()`]: https://docs.rs/smithay-client-toolkit/0.10.0/smithay_client_toolkit/shm/struct.DoubleMemPool.html
[`MemPool`]: https://docs.rs/smithay-client-toolkit/0.10.0/smithay_client_toolkit/shm/struct.MemPool.html
[`std::io::Write`]: https://doc.rust-lang.org/stable/std/io/trait.Write.html
[`std::io::Seek`]: https://doc.rust-lang.org/stable/std/io/trait.Seek.html
[`MemPool::buffer()`]: https://docs.rs/smithay-client-toolkit/0.10.0/smithay_client_toolkit/shm/struct.MemPool.html#method.buffer