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
}
