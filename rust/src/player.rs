use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            base,
        }
    }

    // fn physics_process(&mut self, delta: f64) {
    //     // if self.moving {
    //     //     let velocity = self.base().get_global_position().direction_to(Vector2 {
    //     //         x: self.target_position.x as f32 * 16.0 + 8.0,
    //     //         y: self.target_position.y as f32 * 16.0 + 8.0,
    //     //     }) * self.speed as f32;
    //     //     self.base_mut().translate(velocity * delta as f32);
    //     // }
    // }

    // fn process(&mut self, delta: f64) {
    //     // if self.moving {
    //     //     if self.target_position == self.global_to_tile(&self.base().get_position()) {
    //     //         self.moving = false;
    //     //     } else {
    //     //         return;
    //     //     }
    //     // }
    //     let input = Input::singleton();
    //     let dir: Vector2 = input.get_vector("Move_Left", "Move_Right", "Move_Up", "Move_Down");
    //     self.move_player(&dir);
    //     self.base_mut().translate(dir);
    // }
}

// impl Player {
//     pub fn global_to_tile(&self, pos: &Vector2) -> Vector2i {
//         Vector2i {
//             x: (pos.x / 16.0).round() as i32,
//             y: (pos.y / 16.0).round() as i32,
//         }
//     }
//     pub fn move_player(&mut self, dir: &Vector2){
//         let glob_pos = self.base().get_global_position();
//         self.base_mut().set_global_position(
//             glob_pos + Vector2{x: dir.x * 16.0, y: dir.y * 16.0}
//         );
//     }
// }
