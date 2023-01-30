use bevy::{
    app::{AppExit, ScheduleRunnerPlugin, ScheduleRunnerSettings},
    ecs::schedule::ReportExecutionOrderAmbiguities,
    log::LogPlugin,
    prelude::*,
    utils::Duration,
};
use rand::random;

// Going to be simple Armored Core simulator
// 2 Pilots will customize their AC and will enter a simulated battle

#[derive(Component)]
struct Pilot {
    name: String,
}

// Affects the movement of the AC, in kilos
#[derive(Component)]
struct Weight {
    value: i32,
}

#[derive(Component)]
struct Armor {
    value: i32,
}

#[derive(Component)]
struct Damage {
    value: i32,
}

#[derive(Component)]
struct Firerate {
    value: i32,
}

#[derive(Component)]
struct Accuracy {
    value: i32,
}

#[derive(Component)]
struct Speed {
    value: i32,
}

// Weapon bundle
#[derive(Bundle)]
struct Weapon {
    damage: Damage,
    firerate: Firerate,
    accuracy: Accuracy,
}

// This resource holds info about the game
#[derive(Resource, Default)]
struct GameState {
    current_round: i32,
    winning_pilot: Option<String>,
}

#[derive(Resource)]
struct GameRules {
    max_rounds: i32,
}

fn new_round_system(game_rules: Res<GameRules>, mut game_state: ResMut<GameState>) {
    game_state.current_round += 1;
    println!("Begin round {}", game_state.current_round)
}

// Need to calculate the stats of an AC

fn main() {
    App::new().run();
}
