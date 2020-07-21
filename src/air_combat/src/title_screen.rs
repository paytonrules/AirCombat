use crate::game_state;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct TitleScreen;

#[methods]
impl TitleScreen {
    fn new(_owner: &Node) -> Self {
        TitleScreen
    }

    #[export]
    fn _ready(&self, owner: &Node) {
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
                Instance::<game_state::GameState, _>::from_base(node)
            })
            .expect("Failed to get game state instance");

        rust_game_state
            .map_mut(|gs, o| gs.reset(&o))
            .expect("Could not reset game state");
    }

    #[export]
    fn _on_newgame_pressed(&self, owner: &Node) {
        if let Some(tree) = &owner.get_tree() {
            let tree = unsafe { tree.assume_safe() };
            tree.change_scene("res://GameScene.tscn")
                .expect("Game Scene could not be loaded");
        }
    }

    #[export]
    fn _on_quitgame_pressed(&self, owner: &Node) {
        let tree = owner.get_tree().expect("Couldn't find scene tree!");
        let tree = unsafe { tree.assume_safe() };
        tree.quit(-1);
    }
}
