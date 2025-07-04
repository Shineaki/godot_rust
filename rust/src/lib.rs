use godot::prelude::*;
mod player;
mod map_generator;
mod my_map;
mod rltk_map;
mod rect;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}