use godot::classes::INode2D;
use godot::classes::{Node2D, TileMapLayer};
use godot::prelude::*;

use crate::rltk_map::Map;
use crate::rltk_map::TileType;
use crate::rltk_map::{MAPHEIGHT, MAPWIDTH};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct MapGeneratorNode {
    map: Map,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for MapGeneratorNode {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        Self {
            map: Map::new_map_rooms_and_corridors(),
            base,
        }
    }
}

#[godot_api]
impl MapGeneratorNode {
    #[func]
    pub fn get_spawn_point(&self) -> Vector2 {
        let center_ref = self.map.rooms.get(0).unwrap().center();
        Vector2 {
            x: (center_ref.0 as f32) * 16.0,
            y: (center_ref.1 as f32) * 16.0,
        }
    }

    #[func]
    pub fn generate_floors(&mut self) {
        let mut child_node = self
            .base()
            .try_get_node_as::<TileMapLayer>("FloorMap")
            .unwrap();
        let mut floor_array: Array<Vector2i> = Array::new();
        let mut xx = 0;
        let mut yy = 0;
        for tile in self.map.tiles.iter() {
            let c_coord = Vector2i { x: xx, y: yy };
            if *tile == TileType::Floor {
                floor_array.push(c_coord);
                if xx - 1 >= 0 {
                    let left_pos = self.map.xy_idx(xx - 1, yy);
                    let left_neighbor = self.map.tiles.get(left_pos).unwrap();
                    if *left_neighbor == TileType::Wall {
                        floor_array.push(Vector2i { x: xx - 1, y: yy });
                    }
                }
                if xx + 1 < MAPWIDTH as i32 {
                    let right_pos = self.map.xy_idx(xx + 1, yy);
                    let right_neighbor = self.map.tiles.get(right_pos).unwrap();
                    if *right_neighbor == TileType::Wall {
                        floor_array.push(Vector2i { x: xx + 1, y: yy });
                    }
                }
            }
            xx += 1;
            if xx > MAPWIDTH as i32 - 1 {
                xx = 0;
                yy += 1;
            }
        }
        child_node.set_cells_terrain_connect(&floor_array, 0, 0);
    }

    #[func]
    pub fn generate_walls(&mut self) {
        let mut child_node = self
            .base()
            .try_get_node_as::<TileMapLayer>("WallMap")
            .unwrap();
        let mut wall_array: Array<Vector2i> = Array::new();
        let mut xx = 0;
        let mut yy = 0;
        for tile in self.map.tiles.iter() {
            let c_coord = Vector2i { x: xx, y: yy };
            if *tile == TileType::Wall {
                wall_array.push(c_coord);
            }
            xx += 1;
            if xx > MAPWIDTH as i32 - 1 {
                xx = 0;
                yy += 1;
            }
        }
        // Extra coverage around map
        for y in -12..=MAPHEIGHT as i32 + 12 {
            for x in -12..=MAPWIDTH as i32 + 12 {
                if x < 0 || x >= MAPWIDTH as i32 || y < 0 || y >= MAPHEIGHT as i32 {
                    wall_array.push(Vector2i { x: x, y: y });
                }
            }
        }
        child_node.set_cells_terrain_connect(&wall_array, 0, 0);
    }
}
