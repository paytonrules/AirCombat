use crate::game_scene;
use crate::GameState;
use gdnative::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Player {
    speed: u8,
    vertical_movement: u8,
    bullet_obj: Option<gdnative::PackedScene>,
    dying: bool,
    shot_cooldown: Option<gdnative::Timer>,
    explode: Option<gdnative::Node2D>,
}

const MAX_VERTICAL_MOVEMENT: u8 = 200;
const RATE_OF_FIRE: f32 = 3.0;

#[methods]
impl Player {
    fn _init(_owner: gdnative::Node2D) -> Self {
        Player {
            speed: 150,
            vertical_movement: 0,
            bullet_obj: None,
            dying: false,
            shot_cooldown: None,
            explode: None,
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: gdnative::Node2D) {
        self.shot_cooldown = owner
            .get_node(NodePath::from_str("Timer"))
            .map(|node| node.cast::<Timer>())
            .expect("Missing 'Timer' node");

        if let Some(shot_cooldown) = self.shot_cooldown.as_mut() {
            shot_cooldown.set_wait_time((1.0 / RATE_OF_FIRE) as f64);
            shot_cooldown.set_one_shot(true);
        }

        let mut resource_loader = ResourceLoader::godot_singleton();

        self.explode = resource_loader
            .load("res://Explosion.tscn".into(), "".into(), false)
            .and_then(|res| res.cast::<PackedScene>())
            .and_then(|packed_scene| packed_scene.instance(0))
            .map(|scene| scene.cast::<Node2D>())
            .expect("Could not load explosion");

        self.bullet_obj = resource_loader
            .load("res://Bullet.tscn".into(), "".into(), false)
            .and_then(|res| res.cast::<PackedScene>())
    }

    #[export]
    unsafe fn _process(&mut self, mut owner: gdnative::Node2D, delta: f64) {
        owner.move_local_x(self.speed as f64 * delta, false);

        let position = owner.get_position();
        let bottom = owner.get_viewport_rect().size.height;
        if position.y > 1.0 && position.y <= bottom {
            owner.move_local_y(self.vertical_movement as f64 * delta, false);
        } else {
            if position.y < 1.0 {
                owner.move_local_y(10.0, false);
                self.vertical_movement = 0;
            } else if position.y > bottom {
                owner.move_local_y(-10.0, false);
                self.vertical_movement = 0;
            }
        }

        if self.dying {
            if let Some(shot_cooldown) = self.shot_cooldown {
                if shot_cooldown.get_time_left() == 0.0 {
                    let game: Instance<game_scene::GameScene> = owner
                        .get_node(NodePath::from_str("/root/GameSceneRoot"))
                        .and_then(|node| node.cast::<Node2D>())
                        .and_then(|node| Instance::try_from_base(node))
                        .expect("Could not unwrap game scene");

                    game.map(|p, _| p.player_died())
                        .expect("Player cannot die, you are a god");
                }
            }

            owner.queue_free();
            godot_print!("Dead");
        }
    }

    #[export]
    fn stop(&mut self, _owner: Node2D) {
        self.speed = 0;
    }

    #[export]
    unsafe fn explode(&mut self, owner: Node2D) {
        let rust_game_state: Instance<GameState> = owner
            .get_tree()
            .and_then(|tree| tree.get_root())
            .and_then(|root| root.get_node(NodePath::from_str("./rustGameState")))
            .and_then(|node| Instance::try_from_base(node))
            .expect("Failed to get game state instance");

        rust_game_state
            .map_mut(|gs, _| gs.increment_kills())
            .expect("Could not increment kills for some reason.");

        if let Some(mut explode) = self.explode {
            explode.set_position(owner.get_position());
            let parent = &mut owner
                .get_parent()
                .expect("Could not get parent of player object!");
            parent.add_child(explode.cast::<Node>(), false);
        }

        if let Some(mut shot_cooldown) = self.shot_cooldown {
            shot_cooldown.set_wait_time(2.5);
            shot_cooldown.start(-1.0);
        }

        if let Some(mut sprite) = owner
            .get_node(NodePath::from_str("Sprite"))
            .and_then(|node| node.cast::<Sprite>())
        {
            sprite.set_visible(false);
        }

        self.dying = true;
    }
}
