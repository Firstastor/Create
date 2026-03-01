use bevy::prelude::*;

pub mod assets;
pub mod configs;
pub mod ui;

fn main() {
    App::new()
        .add_plugins(configs::ConfigsPlugin)
        .add_plugins(ui::UIPlugin)
        .run();
}
