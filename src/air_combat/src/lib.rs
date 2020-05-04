use gdnative;
use gdnative::{godot_gdnative_init, godot_gdnative_terminate, godot_nativescript_init};

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Node)]
struct Globals;

#[gdnative::methods]
impl Globals {
    fn _init(_owner: gdnative::Node) -> Self {
        Globals
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<Globals>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
