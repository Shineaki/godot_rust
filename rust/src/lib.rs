use godot::prelude::*;
mod player;
mod my_map;
mod rltk_map;
mod rect;
mod enemy;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}