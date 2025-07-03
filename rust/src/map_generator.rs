use godot::classes::INode2D;
use godot::classes::Node2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct MapGenerator {
    map: Vec<Vector2>,
    #[export]
    map_type: NodeType,
    #[var]
    name: GString,
    base: Base<Node2D>,
}

#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
enum NodeType {
    FLOOR,
    WALL,
}

struct MapNode{
    position: Vector2,
    node_type: NodeType
}

#[godot_api]
impl INode2D for MapGenerator {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            map: vec![Vector2 { x: 1., y: 1. }],
            map_type: NodeType::FLOOR,
            name: GString::from("Map"),
            base,
        }
    }
}

#[godot_api]
impl MapGenerator {
    #[func]
    pub fn get_map(&mut self) -> Array<Vector2> {
        let array = array![Vector2{x: 1., y: 1.}, Vector2{x: 1., y: 1.}, Vector2{x: 1., y: 1.}];
        array
    }
}
