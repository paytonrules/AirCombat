use crate::game_scene;
use gdnative::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Player {
    speed: u8,
    vertical_movement: u8,
    bullet_obj: Option<gdnative::PackedScene>,
    dying: bool,
    shot_cooldown: Option<gdnative::Timer>,
    explode: Option<gdnative::Node>,
}

const MAX_VERTICAL_MOVEMENT: u8 = 200;
const RATE_OF_FIRE: f32 = 3.0;

#[methods]
impl Player {
    fn _init(_owner: gdnative::Node2D) -> Self {
        Player {
            speed: 0,
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
            .map(|scene| scene.instance(0))
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
            owner.move_local_y(MAX_VERTICAL_MOVEMENT as f64 * delta, false);
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
}
