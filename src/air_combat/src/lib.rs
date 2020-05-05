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

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<GameState>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
