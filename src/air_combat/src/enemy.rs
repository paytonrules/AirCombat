use crate::game_state::load_game_state;
use crate::util::create_node_from_scene;
use gdnative::api::Area2D;
use gdnative::api::RandomNumberGenerator;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Enemy {
    sprite: Option<Ref<Sprite>>,
    explode: Option<Ref<Node2D>>,
    speed: u16,
}

const SPRITES: &'static [&'static str] = &[
    "enemy1_blue.png",
    "enemy1_green.png",
    "enemy1_red.png",
    "enemy1_yellow.png",
    "enemy2_blue.png",
    "enemy2_pink.png",
    "enemy2_red.png",
    "enemy2_yellow.png",
];

#[methods]
impl Enemy {
    fn new(_owner: &Node2D) -> Self {
        Enemy {
            speed: 100,
            sprite: None,
            explode: None,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        self.explode = create_node_from_scene::<Node2D>("res://Explosion.tscn");

        if let Some(rust_game_state) = load_game_state(owner) {
            self.speed =
                self.speed + (rust_game_state.map(|gs, _| gs.current_stage).unwrap_or(1) * 10)
        }
    }

    #[export]
    fn _process(&self, owner: &Node2D, delta: f64) {
        owner.move_local_x(-delta * self.speed as f64, false);
    }

    #[export]
    fn _enter_tree(&mut self, owner: &Node2D) {
        let generator = RandomNumberGenerator::new();
        generator.randomize();
        let sprite = Sprite::new().into_shared();
        let sprite = unsafe { sprite.assume_safe() };
        let sprite_name = format!(
            "res://assets/graphics/enemies/{}",
            SPRITES[generator.randi() as usize % SPRITES.len()]
        );
        let resource_loader = ResourceLoader::godot_singleton();
        let texture = resource_loader
            .load(sprite_name, "", false)
            .and_then(|res| res.cast::<Texture>())
            .expect("Couldn't load sprite texture");

        sprite.set_texture(texture);
        self.sprite = Some(sprite.claim());
        owner.add_child(sprite, false);
    }

    #[export]
    fn _on_area2d_area_entered(&mut self, owner: &Node2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.get_collision_layer_bit(3) {
            if let Some(explode) = self.explode {
                let explode = unsafe { explode.assume_safe() };
                let position = owner.position();
                explode.set_position(position);
                if let Some(parent_node) = owner.get_parent() {
                    let parent_node = unsafe { parent_node.assume_safe() };
                    parent_node.add_child(explode, false);
                }
            }

            let rust_game_state = load_game_state(owner).expect("couldn't access game state");
            rust_game_state
                .map_mut(|gs, _| gs.increment_kills())
                .expect("couldn't access game state");
            owner.queue_free();
        }
    }
}
