use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use crate::events::GameOver;
use crate::AppState;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
