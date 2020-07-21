use crate::game_state::GameState;
use crate::player::Player;
use gdnative::api::{AnimationPlayer, Area2D, Camera2D, RandomNumberGenerator};
use gdnative::prelude::*;

#[derive(PartialEq)]
enum State {
    Loading,
    Running,
    GameOver,
}
#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GameScene {
    state: State,
    enemy_obj: Option<Ref<PackedScene>>,
    player: Option<Ref<Node2D>>,
    stage_label: Option<Ref<Label>>,
}

#[methods]
impl GameScene {
    fn new(_owner: &Node2D) -> Self {
        godot_print!("New game scene!");
        GameScene {
            state: State::Loading,
            enemy_obj: None,
            player: None,
            stage_label: None,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node2D) {
        let resource_loader = ResourceLoader::godot_singleton();
        let enemy_scene = resource_loader
            .load("res://Enemy.tscn", "PackedScene", false)
            .expect("Could not load enemy scene");
        let enemy_scene = unsafe { enemy_scene.assume_safe().claim() };
        self.enemy_obj = enemy_scene.cast::<PackedScene>();

        if let Some(tree) = owner.get_tree() {
            let tree = unsafe { tree.assume_safe() };

            let root = tree.root().expect("couldn't find tree root?");
            let root = unsafe { root.assume_safe() };

            let node = root
                .get_node("./rustGameState")
                .map(|node| unsafe { node.assume_unique() })
                .expect("couldn't get node.");

            let rsi =
                Instance::<GameState, _>::try_from_base(node).expect("couldn't convert instance");

            let label_text = rsi
                .map_mut(|gs, _| format!("Stage {}", gs.current_stage))
                .expect("Couldn't build label text");

            if let Some(stage_label) = owner.get_node("./Label") {
                let stage_label = unsafe { stage_label.assume_safe() };

                if let Some(stage_label) = stage_label.cast::<Label>() {
                    stage_label.set_text(label_text);
                    self.stage_label = Some(stage_label.claim());
                }
            }

            let animation_player = owner
                .get_node("./AnimationPlayer")
                .expect("Couldn't find animation player");
            let animation_player = unsafe { animation_player.assume_safe() };
            if let Some(animation_player) = animation_player.cast::<AnimationPlayer>() {
                animation_player.play("Stage Display", -1.0, 1.0, false);
            }
        }
    }

    #[export]
    fn _process(&self, owner: &Node2D, _delta: f64) {
        if let Some(tree) = owner.get_tree() {
            let tree = unsafe { tree.assume_safe() };

            let root = tree.root().expect("couldn't find tree root?");
            let root = unsafe { root.assume_safe() };

            let node = root
                .get_node("./rustGameState")
                .map(|node| unsafe { node.assume_unique() })
                .expect("couldn't get node.");

            let rsi =
                Instance::<GameState, _>::try_from_base(node).expect("couldn't convert instance");

            let hud_kills = owner
                .get_node("./HUD/Kills")
                .expect("Could not load HUD/Kills node");
            let hud_kills = unsafe { hud_kills.assume_safe() };
            let hud_kills = hud_kills
                .cast::<Label>()
                .expect("Could not cast hud kills to Label");

            let hud_text = rsi
                .map_mut(|gs, _| format!("Kills: {}", gs.kills))
                .expect("Couldn't create hud text");
            hud_kills.set_text(hud_text);
        }
    }

    #[export]
    fn start_animation_done(&mut self, owner: &Node2D) {
        self.stage_label.map(|label| {
            let label = unsafe { label.assume_safe() };
            label.set_visible(false)
        });

        let resource_loader = ResourceLoader::godot_singleton();
        let player = resource_loader
            .load("res://PlayerRoot.tscn", "PackedScene", false)
            .and_then(|res| {
                let res = unsafe { res.assume_thread_local() };
                res.cast::<PackedScene>()
            })
            .and_then(|packed_scene| packed_scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED))
            .and_then(|scene| {
                let scene = unsafe { scene.assume_safe() };
                scene.cast::<Node2D>()
            })
            .expect("Could not load player scene");

        player.set_position(euclid::Vector2D::new(300.0, 720.0 / 2.0));

        let cam = Camera2D::new();
        cam.set_position(euclid::Vector2D::new(360.0, 0.0));
        cam.make_current();
        player.add_child(cam, false);

        owner.add_child(player, false);
        self.spawn_enemies(owner);
        self.state = State::Running;
        self.player = Some(player.claim());
    }

    #[export]
    fn _on_area2d_area_entered(&self, owner: &Node2D, area: Ref<Area2D>) {
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
                Instance::<GameState, _>::from_base(node)
            })
            .expect("Failed to get game state instance");

        let area = unsafe { area.assume_safe() };
        if area.get_collision_layer_bit(4) {
            if self.state == State::Running {
                let player_instance = self
                    .player
                    .and_then(|pl| {
                        let pl = unsafe { pl.assume_unique() };
                        Instance::<Player, _>::from_base(pl)
                    })
                    .expect("Could not covert player to player instance!");

                player_instance
                    .map_mut(|pi, _| pi.speed = 0)
                    .expect("Couldn't set player speed!");

                rust_game_state
                    .map_mut(|gs, _| gs.advance_to_next_stage())
                    .expect("Could not advance to next stage!");

                let tree = owner.get_tree().expect("Could not load tree");
                let tree = unsafe { tree.assume_safe() };
                tree.reload_current_scene()
                    .expect("Could not reload current scene");
            }
        }
    }

    pub fn player_died(&mut self) {
        if let Some(player) = self.player {
            let player = unsafe { player.assume_safe() };
            for var in player.get_children().iter() {
                let child = var.try_to_object::<Node>();
                child.map(|node| {
                    let node = unsafe { node.assume_safe() };
                    node.queue_free()
                });
            }
            if let Some(owner) = player.owner() {
                let owner = unsafe { owner.assume_safe() };
                owner.remove_child(player);
                if let Some(label) = owner.get_node("./Label").and_then(|node| {
                    let node = unsafe { node.assume_safe() };
                    node.cast::<Label>()
                }) {
                    let label = unsafe { label.assume_unique() };
                    label.set_text("Game Over");
                    label.set_visible(true);
                    label.set_position(
                        euclid::Vector2D::new(1280.0 / 2.0 - 200.0, 720.0 / 2.0),
                        false,
                    );
                }
            }
        }
        self.state = State::GameOver;
    }

    fn spawn_enemy(&mut self, owner: &Node2D, x: f32, y: f32) {
        if let Some(enemy_obj) = self.enemy_obj.take() {
            let enemy_obj = unsafe { enemy_obj.assume_safe() };
            let enemy = enemy_obj
                .instance(0)
                .and_then(|node| {
                    let node = unsafe { node.assume_safe() };
                    node.cast::<Node2D>()
                })
                .expect("Could not create enemy instance!");
            enemy.set_position(euclid::Vector2D::new(x, y));
            owner.add_child(enemy, false);
            self.enemy_obj.replace(enemy_obj.claim());
        }
    }

    fn spawn_enemies(&mut self, owner: &Node2D) {
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

        let generator = RandomNumberGenerator::new();
        generator.randomize();
        let current_stage = rust_game_state
            .map(|gs, _| gs.current_stage)
            .expect("Couldn't get current stage");

        for _ in 0..=10 + current_stage {
            let bottom = owner.get_viewport_rect().size.height;
            self.spawn_enemy(
                owner,
                (700 + (generator.randi() % 5000)) as f32,
                generator.randi() as f32 % bottom,
            );
        }
    }
}
