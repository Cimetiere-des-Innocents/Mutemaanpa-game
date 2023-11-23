use godot::prelude::*;

struct Mutemaanpa;

#[gdextension]
unsafe impl ExtensionLibrary for Mutemaanpa {}

use godot::engine::Sprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    #[base]
    sprite: Base<Sprite2D>,
}

use godot::engine::ISprite2D;

#[godot_api]
impl ISprite2D for Player {
    fn init(sprite: Base<Sprite2D>) -> Self {
        godot_print!("Player init");
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            sprite,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.sprite.rotate((self.angular_speed * delta) as f32);
    }
}
