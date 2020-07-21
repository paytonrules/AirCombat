use gdnative::api::Area2D;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Bullet;

#[methods]
impl Bullet {
    fn new(_owner: &Node2D) -> Self {
        Bullet
    }

    #[export]
    fn _on_area2d_area_entered(&self, owner: &Node2D, area: Ref<Area2D>) {
        let area = unsafe { area.assume_safe() };
        if area.get_collision_layer_bit(2) {
            owner.queue_free();
        }
    }

    #[export]
    fn _process(&self, owner: &Node2D, delta: f64) {
        owner.move_local_x(delta * 400.0, false)
    }
}
