use godot::classes::image::Format;
use godot::classes::{INode2D, Image, ImageTexture, Sprite2D};
use godot::classes::{Node2D, TileMapLayer};
use godot::prelude::*;
#[derive(GodotClass)]
#[class(base=Node2D)]
struct EnemyNode {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for EnemyNode {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        Self {
            base,
        }
    }
}