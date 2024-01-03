use godot::prelude::*;

struct Mutemaanpa;

#[gdextension]
unsafe impl ExtensionLibrary for Mutemaanpa {}

use godot::engine::Sprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    angular_speed: f64,
    name: String,

    #[base]
    sprite: Base<Sprite2D>,
}

use godot::engine::ISprite2D;

#[godot_api]
impl ISprite2D for Player {
    fn init(sprite: Base<Sprite2D>) -> Self {
        godot_print!("Player init");
        Self {
            angular_speed: std::f64::consts::PI,
            sprite,
            name: "Nameless One".to_string(),
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.sprite.rotate((self.angular_speed * delta) as f32);
    }
}

impl Player {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
