# Babulus Work Ethic

A small Rust/GLFW activity-measurement experiment. The current prototype times
an explicit work session and counts key presses received by its own window,
showing both values live in the title bar.

The project deliberately does **not** install a global keyboard hook, inspect
other applications, run at startup, or transmit/persist activity data. That
keeps the prototype easy to review and avoids collecting sensitive input.

## Run

Requirements:

- Rust 1.70 or newer
- CMake and a C/C++ toolchain supported by the `glfw` crate

```sh
cargo run
```

Press any keys while the window is focused to update the counter. Press `Esc`
or close the window to end the session and print a summary.

## What this demonstrates

- a native GLFW event loop in Rust;
- explicit input-event handling;
- lightweight session metrics using monotonic time; and
- privacy-aware scoping for activity-tracking software.

## Status

This is a bounded desktop prototype, not a background productivity monitor.
Possible future work includes opt-in local persistence, pause/resume controls,
and charts that aggregate session totals without recording typed content.
