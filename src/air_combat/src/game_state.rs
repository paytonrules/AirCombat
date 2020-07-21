use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GameState {
    pub kills: u16,
    pub current_stage: u16,
}

#[methods]
impl GameState {
    fn new(_owner: &Node) -> Self {
        GameState {
            kills: 0,
            current_stage: 1,
        }
    }

    #[export]
    pub fn reset(&mut self, _owner: &Node) {
        self.kills = 0;
        self.current_stage = 1;
    }

    #[export]
    fn current_stage(&self, _owner: &Node) -> u16 {
        self.current_stage
    }

    #[export]
    fn kills(&self, _owner: &Node) -> u16 {
        self.kills
    }

    pub fn advance_to_next_stage(&mut self) {
        self.current_stage += 1;
    }

    pub fn increment_kills(&mut self) {
        self.kills += 1;
    }
}
