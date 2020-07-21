use crate::{game_scene, game_state::GameState};
use euclid::Vector2D;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Player {
    pub speed: u8,
    vertical_movement: i16,
    bullet_obj: Option<Ref<PackedScene, Shared>>,
    dying: bool,
    shot_cooldown: Option<Ref<Timer, Shared>>,
    explode: Option<Ref<Node2D, Shared>>,
}

const MAX_VERTICAL_MOVEMENT: i16 = 200;
const RATE_OF_FIRE: f32 = 3.0;

#[methods]
impl Player {
    fn new(_owner: &Node2D) -> Self {
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
    fn _ready(&mut self, owner: &Node2D) {
        let shot_cooldown = owner.get_node("Timer").expect("Node Should Exist");
        let shot_cooldown = unsafe { shot_cooldown.assume_safe() };
        let shot_cooldown = shot_cooldown
            .cast::<Timer>()
            .expect("Node should cast to timer");

        shot_cooldown.set_wait_time((1.0 / RATE_OF_FIRE) as f64);
        shot_cooldown.set_one_shot(true);

        let resource_loader = ResourceLoader::godot_singleton();
        self.explode = resource_loader
            .load("res://Explosion.tscn", "PackedScene", false)
            .and_then(|res| res.cast::<PackedScene>())
            .and_then(|packed_scene| {
                let packed_scene = unsafe { packed_scene.assume_thread_local() };
                packed_scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED)
            })
            .and_then(|scene| {
                let scene = unsafe { scene.assume_safe() };
                scene.cast::<Node2D>()
            })
            .map(|node| unsafe { node.assume_shared() });

        self.bullet_obj = resource_loader
            .load("res://Bullet.tscn", "PackedScene", false)
            .and_then(|res| res.cast::<PackedScene>());

        self.shot_cooldown = Some(shot_cooldown.claim());
    }

    #[export]
    fn _process(&mut self, owner: &Node2D, delta: f64) {
        owner.move_local_x(self.speed as f64 * delta, false);

        let position = owner.position();
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
                let shot_cooldown = unsafe { shot_cooldown.assume_safe() };
                if shot_cooldown.time_left() == 0.0 {
                    let game = owner
                        .get_node("/root/GameSceneRoot")
                        .and_then(|node| {
                            let node = unsafe { node.assume_safe() };
                            node.cast::<Node2D>()
                        })
                        .map(|node| unsafe { node.assume_unique() })
                        .and_then(|node| Instance::<game_scene::GameScene, _>::from_base(node))
                        .expect("Could not unwrap game scene");

                    game.map_mut(|p, _| p.player_died())
                        .expect("Player cannot die, you are a god");
                }
            }

            owner.queue_free();
        }
    }

    #[export]
    fn _input(&mut self, owner: &Node2D, event: Ref<InputEvent>) {
        let event = unsafe { event.assume_safe() };
        if event.is_action("PLAYER_UP") {
            if self.vertical_movement >= -MAX_VERTICAL_MOVEMENT {
                self.vertical_movement -= 10
            }
        }

        if event.is_action("PLAYER_DOWN") {
            if self.vertical_movement <= MAX_VERTICAL_MOVEMENT {
                self.vertical_movement += 10
            }
        }

        if event.is_action("PLAYER_SHOOT") {
            if let Some(shot_cooldown) = self.shot_cooldown {
                let shot_cooldown = unsafe { shot_cooldown.assume_safe() };
                if shot_cooldown.time_left() == 0.0 {
                    if let Some(bullet_scene) = self.bullet_obj.take() {
                        let bullet_scene = unsafe { bullet_scene.assume_safe() };
                        let bullet = bullet_scene
                            .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
                            .and_then(|b| {
                                let b = unsafe { b.assume_unique() };
                                b.cast::<Node2D>()
                            })
                            .expect("Could not intantiate bullet scene!");

                        bullet.set_position(owner.position());
                        let position = owner.position();
                        bullet.set_position(Vector2D::new(position.x, position.y + 20.0));

                        if let Some(root_scene) = owner.get_node("/root/GameSceneRoot") {
                            let root_scene = unsafe { root_scene.assume_safe() };
                            root_scene.add_child(bullet, false);
                        }
                        shot_cooldown.start(-1.0);
                        self.bullet_obj.replace(bullet_scene.claim());
                    }
                }
            }
        }
    }

    #[export]
    fn stop(&mut self, _owner: &Node2D) {
        self.speed = 0;
    }

    #[export]
    fn explode(&mut self, owner: &Node2D) {
        let rust_game_state = owner
            .get_tree()
            .and_then(|tree| {
                let tree = unsafe { tree.assume_safe() };

                tree.root()
            })
            .and_then(|root| {
                let root = unsafe { root.assume_safe() };
                root.get_node("./rustGameState")
            })
            .and_then(|node| {
                let node = unsafe { node.assume_unique() };
                Instance::<GameState, _>::try_from_base(node).ok()
            })
            .expect("Failed to get game state instance");

        rust_game_state
            .map_mut(|gs, _| gs.increment_kills())
            .expect("Could not increment kills for some reason.");

        if let Some(explode) = self.explode.take() {
            let explode = unsafe { explode.assume_safe() };
            explode.set_position(owner.position());
            let parent = &owner
                .get_parent()
                .map(|p| unsafe { p.assume_safe() })
                .expect("Could not get parent of player object!");
            parent.add_child(explode, false);
            self.explode.replace(explode.claim());
        }

        if let Some(shot_cooldown) = self.shot_cooldown {
            let shot_cooldown = unsafe { shot_cooldown.assume_safe() };
            shot_cooldown.set_wait_time(2.5);
            shot_cooldown.start(-1.0);
        }

        if let Some(sprite) = owner
            .get_node("Sprite")
            .map(|node| unsafe { node.assume_safe() })
            .and_then(|node| node.cast::<Sprite>())
        {
            sprite.set_visible(false);
        }

        self.dying = true;
    }

    #[export]
    fn _on_area2d_area_entered(&mut self, owner: &Node2D, area: Ref<gdnative::api::Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.get_collision_layer_bit(2) {
            self.explode(owner);
        }
    }
}
