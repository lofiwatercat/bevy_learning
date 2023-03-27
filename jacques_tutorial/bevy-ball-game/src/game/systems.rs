use super::SimulationState;
use bevy::prelude::*;

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.0 {
            SimulationState::Running => {
                next_simulation_state.set(SimulationState::Paused);
                println!("Simulation Paused");
            }
            SimulationState::Paused => {
                next_simulation_state.set(SimulationState::Running);
                println!("Simulation Resumed");
            }
        }
    }
}
