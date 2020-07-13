use crate::game_state::GameState;
use gdnative::api::Area2D;
use gdnative::api::RandomNumberGenerator;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Enemy {
    sprite: Option<Ref<Sprite>>,
    explode: Option<Ref<Node2D, Unique>>,
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
    fn _ready(&self, owner: &Node2D) {
        let resource_loader = ResourceLoader::godot_singleton();
        let explosion_scene = resource_loader
            .load("res://Explosion.tscn", "PackedScene", false)
            .expect("Could not load scene");

        let explosion_scene = unsafe { explosion_scene.assume_thread_local() };

        // Optional<Ref<Node, Shared>>
        //
        let explosion_node = explosion_scene
            .cast::<PackedScene>()
            .and_then(|packed_scene| packed_scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED))
            .expect("Could not create instance of scene");

        let explosion_node = unsafe { explosion_node.assume_unique() };
        self.explode = explosion_node.try_cast::<Node2D>().ok();

        if let Some(tree) = owner.get_tree() {
            let tree = unsafe { tree.assume_safe() };
            if let Some(root) = tree.root() {
                let root = unsafe { root.assume_safe() };
                if let Some(node) = root.get_node("./rustGameState") {
                    let rust_game_state_instance = Instance::<GameState, _>::try_from_base(node);

                    let speed = match &rust_game_state_instance {
                        Ok(instance) => {
                            let instance = unsafe { instance.assume_safe() };
                            instance.map(|gs, _| gs.current_stage).unwrap_or(1)
                        }
                        Err(_) => panic!("Oh damn"),
                    };
                    self.speed = self.speed + (speed * 10)
                }
            }
        }
    }

    #[export]
    fn _process(&self, owner: &Node2D, delta: f64) {
        owner.move_local_x(-delta * self.speed as f64, false);
    }

    #[export]
    fn _enter_tree(&self, owner: &Node2D) {
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
    fn _on_area2d_area_entered(&self, owner: &Node2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.get_collision_layer_bit(3) {
            if let Some(explode) = self.explode {
                let position = owner.position();
                explode.set_position(position);
                if let Some(parent_node) = owner.get_parent() {
                    let parent_node = unsafe { parent_node.assume_safe() };
                    parent_node.add_child(explode, false);
                }

                if let Some(tree) = owner.get_tree() {
                    let tree = { tree.assume_safe() };

                    let root = tree.root().expect("couldn't find tree root?");
                    let root = { root.assume_safe() };

                    let node = root
                        .get_node("./rustGameState")
                        .expect("couldn't get node.");
                    let rsi = Instance::<GameState, _>::try_from_base(node)
                        .expect("couldn't convert instance");

                    rsi.map_mut(|gs, _| gs.increment_kills())
                        .expect("couldn't access game state");
                }
                owner.queue_free();
            }
        }
    }
}
