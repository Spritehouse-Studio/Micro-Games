//! The main logic of the game.

/// The main game module.
pub mod game {
    use bevy::app::{App, Plugin};
    use spritehouse_engine::engine::EnginePlugin;

    const GAME_NAME: &str = "Micro Games";

    /// The main game plugin.
    #[derive(Default, Debug)]
    pub struct GamePlugin;

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins(EnginePlugin {
                game_name: GAME_NAME.to_string(),
            });
        }
    }
}
