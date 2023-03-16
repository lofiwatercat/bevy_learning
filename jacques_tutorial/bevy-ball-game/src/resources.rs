use bevy::prelude::*;

const STAR_SPAWN_TIME: f32 = 1.0;
const ENEMY_SPAWN_TIME: f32 = 5.0;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> HighScores {
        HighScores { scores: Vec::new() }
    }
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}
