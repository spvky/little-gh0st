use bevy::prelude::*;

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<GameMode>()
            .add_sub_state::<Vessel>();
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    MainMenu,
    #[default]
    InGame,
    Paused,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::InGame)]
pub enum GameMode {
    #[default]
    ThickWorld,
    ThinWorld,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Reflect, Default)]
#[source(GameMode = GameMode::ThickWorld)]
pub enum Vessel {
    #[default]
    Bipedal,
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameState::MainMenu => write!(f, "MainMenu"),
            GameState::Paused => write!(f, "Paused"),
            GameState::InGame => write!(f, "InGame"),
        }
    }
}

impl std::fmt::Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameMode::ThinWorld => write!(f, "Thin W0rld"),
            GameMode::ThickWorld => write!(f, "Thick W0rld"),
        }
    }
}

impl std::fmt::Display for Vessel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vessel::Bipedal => write!(f, "Bipedal"),
        }
    }
}
