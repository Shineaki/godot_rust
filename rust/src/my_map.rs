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
    floor_map: Option<Gd<TileMapLayer>>,
    shadow_map: Option<Gd<TileMapLayer>>,
    wall_map: Option<Gd<TileMapLayer>>,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for MapGeneratorNode {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        Self {
            map: Map::new_map_rooms_and_corridors(),
            floor_map: None,
            shadow_map: None,
            wall_map: None,
            base,
        }
    }
    fn ready(&mut self) {
        self.floor_map = self.base().try_get_node_as::<TileMapLayer>("FloorMap");
        self.shadow_map = self.base().try_get_node_as::<TileMapLayer>("ShadowMap");
        self.wall_map = self.base().try_get_node_as::<TileMapLayer>("WallMap");
    }
}

#[godot_api]
impl MapGeneratorNode {
    #[func]
    pub fn get_spawn_point(&self) -> Vector2 {
        let center_ref = self.map.rooms[0].center();
        Vector2 {
            x: (center_ref.0 as f32) * 16.0 + 8.0,
            y: (center_ref.1 as f32) * 16.0 + 8.0,
        }
    }

    #[func]
    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        self.map.blocked[self.map.xy_idx(x, y)]
    }

    #[func]
    pub fn generate_floors(&mut self) {
        let child_node = self.floor_map.as_mut().unwrap();
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
    pub fn init_enemies(&mut self) {}

    #[func]
    pub fn init_shadows(&mut self) {
        let child_node = self.shadow_map.as_mut().unwrap();
        // Extra coverage around map
        let mut shadow_array: Array<Vector2i> = Array::new();
        for y in -12..=MAPHEIGHT as i32 + 12 {
            for x in -12..=MAPWIDTH as i32 + 12 {
                shadow_array.push(Vector2i { x: x, y: y });
            }
        }
        child_node.set_cells_terrain_connect(&shadow_array, 0, 0);
    }

    #[func]
    pub fn generate_shadows(&mut self, player_pos: Vector2i) {
        self.map.update_revealed((player_pos.x, player_pos.y));
        let child_node = self.shadow_map.as_mut().unwrap();
        let mut shadow_array: Array<Vector2i> = Array::new();

        for y in player_pos.y - 8..=player_pos.y + 8 {
            for x in player_pos.x - 8..=player_pos.x + 8 {
                if x < 0 || x >= MAPWIDTH as i32 || y < 0 || y >= MAPHEIGHT as i32 {
                    continue;
                } else {
                    child_node.erase_cell(Vector2i { x: x, y: y });
                    if self.map.visible_tiles[self.map.xy_idx(x, y)] != true {
                        shadow_array.push(Vector2i { x: x, y: y });
                    }
                }
            }
        }
        child_node.set_cells_terrain_connect(&shadow_array, 0, 0);
    }

    // #[func]
    // pub fn update_minimap(&self, player_pos: Vector2i) {
    //     let mut minimap = self
    //         .base()
    //         .try_get_node_as::<TileMapLayer>("../UI/MarginContainer/PanelContainer/TileMapLayer")
    //         .unwrap();
    //     let mut wall_array: Array<Vector2i> = Array::new();
    //     let mut floor_array: Array<Vector2i> = Array::new();
    //     let mut minimap_x = 0;
    //     let mut minimap_y = 0;
    //     for y in player_pos.y - 18..=player_pos.y + 18 {
    //         for x in player_pos.x - 30..=player_pos.x + 30 {
    //             minimap.erase_cell(Vector2i {
    //                 x: minimap_x,
    //                 y: minimap_y,
    //             });
    //             if x < 0 || x >= MAPWIDTH as i32 || y < 0 || y >= MAPHEIGHT as i32 {
    //                 minimap_x += 1;
    //                 continue;
    //             }
    //             let c_pos = self.map.xy_idx(x, y);
    //             if self.map.revealed_tiles[c_pos] == true {
    //                 if self.map.tiles[c_pos] == TileType::Wall {
    //                     wall_array.push(Vector2i {
    //                         x: minimap_x,
    //                         y: minimap_y,
    //                     });
    //                 } else {
    //                     floor_array.push(Vector2i {
    //                         x: minimap_x,
    //                         y: minimap_y,
    //                     });
    //                 }
    //             }
    //             minimap_x += 1;
    //         }
    //         minimap_y += 1;
    //         minimap_x = 0;
    //     }
    //     minimap.set_cells_terrain_connect(&wall_array, 0, 0);
    //     minimap.set_cells_terrain_connect(&floor_array, 0, 1);
    //     // minimap.set_ce
    // }

    #[func]
    pub fn generate_walls(&mut self) {
        let child_node = self.wall_map.as_mut().unwrap();
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
        for y in -20..=MAPHEIGHT as i32 + 20 {
            for x in -20..=MAPWIDTH as i32 + 20 {
                if x < 0 || x >= MAPWIDTH as i32 || y < 0 || y >= MAPHEIGHT as i32 {
                    wall_array.push(Vector2i { x: x, y: y });
                }
            }
        }
        child_node.set_cells_terrain_connect(&wall_array, 0, 0);
        self.map.populate_blocked();
    }
}
