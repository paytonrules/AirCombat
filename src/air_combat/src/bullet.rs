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
