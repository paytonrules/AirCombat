use crate::game_state;
use gdnative::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct TitleScreen;

#[methods]
impl TitleScreen {
    fn _init(_owner: gdnative::Node) -> Self {
        TitleScreen
    }

    #[export]
    unsafe fn _ready(&self, owner: gdnative::Node) {
        let rust_game_state: Instance<game_state::GameState> = owner
            .get_tree()
            .and_then(|tree| tree.get_root())
            .and_then(|root| root.get_node(NodePath::from_str("./rustGameState")))
            .and_then(|node| Instance::try_from_base(node))
            .expect("Failed to get game state instance");

        rust_game_state
            .map_mut(|gs, o| gs.reset(o))
            .expect("Could not reset game state");
    }

    #[export]
    unsafe fn _on_newgame_pressed(&self, owner: gdnative::Node) {
        if let Some(tree) = &mut owner.get_tree() {
            tree.change_scene("res://GameScene.tscn".into())
                .expect("Game Scene could not be loaded");
        }
    }

    #[export]
    unsafe fn _on_quitgame_pressed(&self, owner: gdnative::Node) {
        let tree = &mut owner.get_tree().expect("Couldn't find scene tree!");
        tree.quit(-1);
    }
}
