use bevy::prelude::*;

/// A normalized audio volume in 0.0..=1.0. Owns step <-> linear conversion
/// and the perceptual curve so no caller has to reinvent them.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct Volume(f32);

impl Volume {
    /// Single source of truth for "10 steps".
    pub const STEPS: u8 = 10;

    pub fn new(linear: f32) -> Self {
        Self(linear.clamp(0.0, 1.0))
    }

    pub fn from_steps(steps: u8) -> Self {
        Self::new(steps as f32 / Self::STEPS as f32)
    }

    pub fn as_steps(self) -> u8 {
        (self.0 * Self::STEPS as f32).round() as u8
    }

    /// Humans hear loudness logarithmically; square the linear value before
    /// sending it to an AudioSink so the control *feels* linear.
    pub fn perceptual(self) -> f32 {
        self.0 * self.0
    }
}
