use gdnative::*;
mod bullet;
mod enemy;
mod game_scene;
mod game_state;
mod player;
mod title_screen;

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<bullet::Bullet>();
    handle.add_class::<title_screen::TitleScreen>();
    handle.add_class::<game_state::GameState>();
    handle.add_class::<game_scene::GameScene>();
    handle.add_class::<player::Player>();
    handle.add_class::<enemy::Enemy>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
