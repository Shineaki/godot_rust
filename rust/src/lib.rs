use godot::prelude::*;
mod player;
mod map_generator;
mod my_map;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}