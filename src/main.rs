use bevy::prelude::*;
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_survivors::game::game_plugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, EnhancedInputPlugin));
    app.add_plugins(game_plugin);

    app.run()
}
