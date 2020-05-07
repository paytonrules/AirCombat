use gdnative::*;

enum GameState {
    Loading,
    Running,
    GameOver,
}
#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GameScene;

#[methods]
impl GameScene {
    fn _init(_owner: gdnative::Node2D) -> Self {
        GameScene
    }
}
