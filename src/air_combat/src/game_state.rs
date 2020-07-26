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

pub fn load_game_state(node: &Node) -> Option<Instance<GameState, Unique>> {
    let tree = node.get_tree()?;
    let tree = unsafe { tree.assume_safe() };

    let root = tree.root()?;
    let root = unsafe { root.assume_safe() };

    let game_state_node = root.get_node("./rustGameState")?;
    let game_state_node = unsafe { game_state_node.assume_unique() };

    Instance::<GameState, _>::from_base(game_state_node)
}
