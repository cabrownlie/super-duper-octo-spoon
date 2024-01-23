use bevy::{app::App, DefaultPlugins};
use super_duper_octo_spoon::game::plugin::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
