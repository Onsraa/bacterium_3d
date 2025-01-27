use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimulationState {
    #[default]
    Refreshing,
    Ready,
    Running,
    Updating,
    Paused,
}