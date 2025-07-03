use godot::classes::ITileMapLayer;
use godot::classes::TileMapLayer;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
struct MyRustMap {
    map: Vec<Vector2>,
    #[export]
    map_type: NodeType,
    #[var]
    name: GString,
    base: Base<TileMapLayer>,
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
impl ITileMapLayer for MyRustMap {
    fn init(base: Base<TileMapLayer>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        Self {
            map: vec![Vector2 { x: 1., y: 1. }],
            map_type: NodeType::FLOOR,
            name: GString::from("Map"),
            base,
        }
    }
    fn ready(&mut self,){
        let x_min = -20;
        let y_min = -20;
        let x_max = 20;
        let y_max = 20;
        let mut map_array: Array<Vector2i> = Array::new();
        for j in y_min..y_max {
            for i in x_min..x_max{
                if i32::abs(i) > 5 || i32::abs(j) > 5 {
                    map_array.push(Vector2i{x: i, y: j});
                }
            }
        }
        // let cells: Array<Vector2i> = array![Vector2i{x: 1, y: 1}, Vector2i{x: 1, y: 2}, Vector2i{x: 2, y: 1}, Vector2i{x: 2, y: 2}];

        // Call the function
        self.base_mut().set_cells_terrain_connect(
            &map_array,
            0,
            0,
        );
    }
}

#[godot_api]
impl MyRustMap {
    #[func]
    pub fn get_map(&mut self) -> Array<Vector2> {
        let array = array![Vector2{x: 1., y: 1.}, Vector2{x: 1., y: 1.}, Vector2{x: 1., y: 1.}];
        array
    }
}
