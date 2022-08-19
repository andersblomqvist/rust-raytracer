# Ray Tracing in one Weekend with Rust

This is an implementation of the [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) using Rust.

## Multithreading

A simple multithreading was also implemented to speed up rendering times. This was done by splitting up the canvas into $N$ equal parts, where each part contained $N$ rows to calculate. Each part wrote the pixel color to its own buffer which meant we later (in this case, when all of the threads had finished) had to stitch the canvas back together. The data communication was handled with rust's MPSC library [1]. Each producer sent their canvas segment to the main thread which stitched them together. Table 1 shows a comparison between the rendering times measured in seconds on an Intel i5 12600K at stock speeds at 16 spawned threads. The resolution is in pixels and SPP is how many samples per pixel were used.

| Resolution        | SPP | Multi-threaded (s) | Single-threaded (s) |
| ----------------- |---- |:------------------ |:------------------- |
| 256x144           | 50  | 7                  | 54                  |
| 256x144           | 500 | 72                 | 530                 |
| 1920x1080         | 250 | 2026               | inf                 |

**Table 1:** Comparison of rendering times between multi and single-threading.

[1] [Rust Module std::sync::mpsc](https://doc.rust-lang.org/std/sync/mpsc/)
