use gdnative::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Bullet;

#[methods]
impl Bullet {
    fn _init(_owner: gdnative::Node2D) -> Self {
        Bullet
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
