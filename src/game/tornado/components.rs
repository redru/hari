use bevy::prelude::*;

pub const TORNADO_ENTERING_DURATION: f32 = 3.;

pub enum TornadoState {
    Entering,
    Executing,
    Exiting,
}

#[derive(Component)]
pub struct Tornado {
    pub timer: Timer,
    pub thrown_shots: i32,
    pub state: TornadoState,
}

impl Tornado {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(TORNADO_ENTERING_DURATION, TimerMode::Once),
            thrown_shots: 0,
            state: TornadoState::Entering,
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct TornadoTimer(pub Timer);
