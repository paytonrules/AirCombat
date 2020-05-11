use gdnative::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Enemy;

#[methods]
impl Enemy {
    fn _init(_owner: gdnative::Node2D) -> Self {
        Enemy
    }
}
