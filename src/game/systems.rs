use bevy::prelude::*;

use super::components::MainCamera;

pub fn setup(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle::default()));
}
