use godot::prelude::*;
use rltk::field_of_view;
use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator};
use std::cmp::{max, min};

use crate::rect::Rect;

pub const MAPWIDTH: usize = 100;
pub const MAPHEIGHT: usize = 80;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(GodotConvert, Var, Export, PartialEq, Copy, Clone)]
#[godot(via = GString)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>, // TODO: Change to Godot rect
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        x.wrapping_add(y.wrapping_mul(self.width)) as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }
    pub fn update_revealed(&mut self, play_pos: (i32, i32)) {
        let mut visible_tiles = field_of_view(Point::new(play_pos.0, play_pos.1), 6, self);
        visible_tiles.retain(|p| p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height);
        for t in self.visible_tiles.iter_mut() {
            *t = false
        }
        for vis in visible_tiles.iter() {
            let idx: usize = self.xy_idx(vis.x, vis.y);
            self.revealed_tiles[idx] = true;
            self.visible_tiles[idx] = true;
        }
    }

    /// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
    /// This gives a handful of random rooms and corridors joining them together.
    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; MAPCOUNT],
            rooms: Vec::new(),
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            revealed_tiles: vec![false; MAPCOUNT],
            visible_tiles: vec![false; MAPCOUNT],
            blocked: vec![false; MAPCOUNT],
        };

        const MAX_ROOMS: i32 = 50;
        const MIN_SIZE: i32 = 5;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.range(2, map.width - w - 2);// rng.roll_dice(1, map.width - w - 1) - 2;
            let y = rng.range(2, map.height - h - 2); // rng.roll_dice(1, map.height - h - 1) - 2;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }
        // Delete single vertical walls
        for y in 1..MAPHEIGHT as i32 - 1 {
            for x in 1..MAPWIDTH as i32 - 1 {
                let c_coord = map.xy_idx(x, y);
                if map.tiles[c_coord] == TileType::Wall {
                    if (map.tiles[map.xy_idx(x - 1, y)] == TileType::Floor
                        && map.tiles[map.xy_idx(x + 1, y)] == TileType::Floor)
                        || (map.tiles[map.xy_idx(x, y - 1)] == TileType::Floor
                            && map.tiles[map.xy_idx(x, y + 1)] == TileType::Floor)
                    {
                        map.tiles[c_coord] = TileType::Floor;
                    }
                }
            }
        }
        map
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - w) + 1, 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + w) + 1, 1.45));
        }

        exits
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}
