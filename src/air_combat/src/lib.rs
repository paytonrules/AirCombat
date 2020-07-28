use gdnative::prelude::*;
mod bullet;
mod enemy;
mod game_scene;
mod game_state;
mod player;
mod title_screen;
mod util;

fn init(handle: InitHandle) {
    handle.add_class::<bullet::Bullet>();
    handle.add_class::<title_screen::TitleScreen>();
    handle.add_class::<game_state::GameState>();
    handle.add_class::<game_scene::GameScene>();
    handle.add_class::<player::Player>();
    handle.add_class::<enemy::Enemy>();
}

godot_init!(init);
