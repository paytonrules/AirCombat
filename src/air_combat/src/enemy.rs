use crate::game_state::GameState;
use gdnative::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Enemy {
    sprite: Option<Sprite>,
    explode: Option<Node2D>,
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
    fn _init(_owner: gdnative::Node2D) -> Self {
        Enemy {
            speed: 100,
            sprite: None,
            explode: None,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: gdnative::Node2D) {
        if let Some(node) = &mut owner.get_node(NodePath::from_str("./Area2D")) {
            let godot_object = &owner.to_object();
            node.connect(
                "area_entered".into(),
                Some(*godot_object),
                "_on_area2d_area_entered".into(),
                VariantArray::new(),
                0,
            )
            .expect("Couldn't connect area_entered");
        }

        let mut resource_loader = ResourceLoader::godot_singleton();
        self.explode = resource_loader
            .load("res://Explosion.tscn".into(), "".into(), false)
            .and_then(|res| res.cast::<PackedScene>())
            .and_then(|packed_scene| packed_scene.instance(0))
            .map(|scene| scene.cast::<Node2D>())
            .expect("Could not load explosion");

        let rust_game_state: Instance<GameState> = owner
            .get_tree()
            .and_then(|tree| tree.get_root())
            .and_then(|root| root.get_node(NodePath::from_str("./rustGameState")))
            .and_then(|node| Instance::try_from_base(node))
            .expect("Failed to get game state instance");

        self.speed = self.speed + rust_game_state.map(|gs, _| gs.current_stage).unwrap_or(1) * 10;
    }

    #[export]
    unsafe fn _process(&self, mut owner: Node2D, delta: f64) {
        owner.move_local_x(-delta * self.speed as f64, false);
    }

    #[export]
    unsafe fn _enter_tree(&mut self, mut owner: Node2D) {
        let mut generator = RandomNumberGenerator::new();
        generator.randomize();
        let mut sprite = Sprite::new();
        let sprite_name = format!(
            "res://assets/graphics/enemies/{}",
            SPRITES[generator.randi() as usize % SPRITES.len()]
        );
        let mut resource_loader = ResourceLoader::godot_singleton();
        let texture = resource_loader
            .load(sprite_name.into(), "".into(), false)
            .and_then(|res| res.cast::<Texture>());

        sprite.set_texture(texture);
        owner.add_child(Some(sprite.to_node()), false);
        self.sprite = Some(sprite);
    }

    #[export]
    unsafe fn _on_area2d_area_entered(&self, mut owner: Node2D, area: Area2D) {
        if area.get_collision_layer_bit(3) {
            if let Some(mut explode) = self.explode {
                let position = owner.get_position();
                explode.set_position(position);
                if let Some(mut parent_node) = owner.get_parent() {
                    parent_node.add_child(Some(explode.to_node()), false);
                }
                let rust_game_state: Instance<GameState> = owner
                    .get_tree()
                    .and_then(|tree| tree.get_root())
                    .and_then(|root| root.get_node(NodePath::from_str("./rustGameState")))
                    .and_then(|node| Instance::try_from_base(node))
                    .expect("Failed to get game state instance");

                rust_game_state
                    .map_mut(|gs, _| gs.increment_kills())
                    .expect("Couldn't access gamestate!");
                owner.queue_free();
            }
        }
    }
}
