use gdnative::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GameState {
    kills: u16,
    current_stage: u16,
}

#[methods]
impl GameState {
    fn _init(_owner: gdnative::Node) -> Self {
        GameState {
            kills: 0,
            current_stage: 1,
        }
    }

    #[export]
    fn reset(&mut self, _owner: gdnative::Node) {
        self.kills = 0;
        self.current_stage = 1;
    }

    #[export]
    fn current_stage(&self, _owner: gdnative::Node) -> u16 {
        self.current_stage
    }

    #[export]
    fn kills(&self, _owner: gdnative::Node) -> u16 {
        self.kills
    }

    #[export]
    fn advance_to_next_stage(&mut self, _owner: gdnative::Node) {
        self.current_stage += 1;
    }

    #[export]
    fn increment_kills(&mut self, _owner: gdnative::Node) {
        self.kills += 1;
    }
}

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Bullet;

#[methods]
impl Bullet {
    fn _init(_owner: gdnative::Node2D) -> Self {
        Bullet
    }

    #[export]
    unsafe fn _ready(&self, owner: gdnative::Node2D) {
        if let Some(node) = &mut owner.get_node(NodePath::from_str("./Area2D")) {
            let godot_object = &owner.to_object();
            node.connect(
                "area_entered".into(),
                Some(*godot_object),
                "_on_area2d_area_entered".into(),
                VariantArray::new(),
                0,
            )
            .expect("Couldn't connect area_enetered to Area2D");
        }
    }

    #[export]
    unsafe fn _on_area2d_area_entered(&self, mut owner: gdnative::Node2D, area: gdnative::Area2D) {
        if area.get_collision_layer_bit(2) {
            owner.queue_free();
        }
    }

    #[export]
    unsafe fn _process(&self, mut owner: gdnative::Node2D, delta: f64) {
        owner.move_local_x(delta * 400.0, false)
    }
}

#[derive(NativeClass)]
#[inherit(Node)]
struct TitleScreen;

#[methods]
impl TitleScreen {
    fn _init(_owner: gdnative::Node) -> Self {
        TitleScreen
    }

    #[export]
    unsafe fn _ready(&self, owner: gdnative::Node) {
        let rust_game_state: Instance<GameState> = owner
            .get_tree()
            .and_then(|tree| tree.get_root())
            .and_then(|root| root.get_node(NodePath::from_str("./rustGameState")))
            .and_then(|node| Instance::try_from_base(node))
            .expect("Failed to get game state instance");

        rust_game_state
            .map_mut(|gs, o| gs.reset(o))
            .expect("Could not reset game state");

        if let Some(node) = &mut owner.get_node(NodePath::from_str("./NewGame")) {
            let godot_object = &owner.to_object();
            node.connect(
                "pressed".into(),
                Some(*godot_object),
                "_on_newgame_pressed".into(),
                VariantArray::new(),
                0,
            )
            .expect("Couldn't connect on_newgame_pressed");
        }

        if let Some(node) = &mut owner.get_node(NodePath::from_str("./QuitGame")) {
            let godot_object = &owner.to_object();
            node.connect(
                "pressed".into(),
                Some(*godot_object),
                "_on_quitgame_pressed".into(),
                VariantArray::new(),
                0,
            )
            .expect("Couldn't connect on_newgame_pressed");
        }
    }

    #[export]
    unsafe fn _on_newgame_pressed(&self, owner: gdnative::Node) {
        if let Some(tree) = &mut owner.get_tree() {
            tree.change_scene("res://GameScene.tscn".into());
        }
    }

    #[export]
    unsafe fn _on_quitgame_pressed(&self, owner: gdnative::Node) {
        let tree = &mut owner.get_tree().expect("Couldn't find scene tree!");
        tree.quit(-1);
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<GameState>();
    handle.add_class::<Bullet>();
    handle.add_class::<TitleScreen>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
