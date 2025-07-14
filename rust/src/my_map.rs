use godot::classes::image::Format;
use godot::classes::{INode2D, Image, ImageTexture, Sprite2D};
use godot::classes::{Node2D, TileMapLayer};
use godot::prelude::*;

use crate::rltk_map::Map;
use crate::rltk_map::TileType;
use crate::rltk_map::{MAPHEIGHT, MAPWIDTH};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct MapGeneratorNode {

    // A git okes

    map: Map,
    floor_map: Option<Gd<TileMapLayer>>,
    wall_map: Option<Gd<TileMapLayer>>,
    fov: Option<Gd<Sprite2D>>,
    fog_image: Gd<Image>,
    fog_image_texture: Gd<ImageTexture>,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for MapGeneratorNode {
    fn init(base: Base<Node2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        Self {
            map: Map::new_map_rooms_and_corridors(),
            floor_map: None,
            wall_map: None,
            fov: None,
            fog_image: Image::new_gd(),
            fog_image_texture: ImageTexture::new_gd(),
            base,
        }
    }
    fn ready(&mut self) {
        self.floor_map = self.base().try_get_node_as::<TileMapLayer>("FloorMap");
        self.wall_map = self.base().try_get_node_as::<TileMapLayer>("WallMap");
        self.fov = self.base().try_get_node_as::<Sprite2D>("../FOW");
        self.fog_image = Image::create(100, 80, false, Format::RGBAH).expect("Couldn't create fog image");
        self.fog_image.fill(Color::BLACK);
    }
}

#[godot_api]
impl MapGeneratorNode {
    pub fn update_fog(&mut self, pos: Vector2i){
        for y in -8..=8 {
            for x in -8..=8 {
                let global_pos = Vector2i {
                    x: pos.x + x,
                    y: pos.y + y,
                };
                if self.is_visible(global_pos.x, global_pos.y) {
                    // let float_val = std::cmp::min(y.abs() * x.abs(), 1);
                    self.fog_image.set_pixel(global_pos.x, global_pos.y, Color { r: 1., g: 1., b: 1., a: 1. });
                } else if self.is_explored(global_pos.x, global_pos.y) {
                    self.fog_image.set_pixel(global_pos.x, global_pos.y, Color { r: 0.5, g: 0.5, b: 0.5, a: 1. });
                }
            }
        }
        self.fog_image_texture = ImageTexture::create_from_image(&self.fog_image).expect("Couldn't create fog texture");
    }

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
        if x < 0 || x >= MAPWIDTH as i32 || y < 0 || y >= MAPHEIGHT as i32 {
            return true;
        }
        self.map.blocked[self.map.xy_idx(x, y)]
    }

    #[func]
    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= MAPWIDTH as i32 || y < 0 || y >= MAPHEIGHT as i32 {
            return false;
        }
        self.map.visible_tiles[self.map.xy_idx(x, y)]
    }

    #[func]
    pub fn is_explored(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= MAPWIDTH as i32 || y < 0 || y >= MAPHEIGHT as i32 {
            return false;
        }
        self.map.revealed_tiles[self.map.xy_idx(x, y)]
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
    pub fn generate_shadows(&mut self, player_pos: Vector2i) {
        self.map.update_revealed((player_pos.x, player_pos.y));
        self.update_fog(player_pos);
        self.fov.as_mut().expect("Couldn't get FOV").set_texture(&self.fog_image_texture);
    }

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
        // for y in 0..MAPHEIGHT as i32 {
        //     for x in 0..MAPWIDTH as i32 {
        //         if x < 0 || x >= MAPWIDTH as i32 || y < 0 || y >= MAPHEIGHT as i32 {
        //             wall_array.push(Vector2i { x: x, y: y });
        //         }
        //     }
        // }
        child_node.set_cells_terrain_connect(&wall_array, 0, 0);
        self.map.populate_blocked();
    }
}
