# Introdcution
SimpleGUIToolkit or SGT is a Rust crossplatform GUI toolkit which is inspired by many different things really. It's basically a project for me to understand the lower level stuff that goes on with GUI and Graphics Programming.

# Goals
## ECS Architecture
- ECS stands for Entity Component System.
- Can use [specs](https://github.com/amethyst/specs) crate.
​
## Cross Platform
- Use platform native technologies for rendering.
- Can use [wgpu](https://github.com/gfx-rs/wgpu) crate.
​
## Low Level Access
- Should be low level enough to use to build web browsers and other lower level applications.
- Can have maybe widgets to directly give access to drawing using wgpu/opengl.
​
## Performance
- Should be fast.
- Use Rust's async/await syntax.
- Have support for multi-threaded async, not single threaded.
- Use async channels for all communication between widgets.
