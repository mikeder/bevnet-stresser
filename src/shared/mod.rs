use bevy::prelude::*;
use derive_more::{Add, Mul};
use serde::{Deserialize, Serialize};
use std::ops::Mul;

#[derive(Component, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PlayerColor(pub(crate) Color);

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Deref, DerefMut, Add, Mul)]
pub struct Position(pub(crate) Vec2);

impl Mul<f32> for &Position {
    type Output = Position;

    fn mul(self, rhs: f32) -> Self::Output {
        Position(self.0 * rhs)
    }
}

// Player
#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    position: Position,
    color: PlayerColor,
}

impl PlayerBundle {
    pub(crate) fn new(color: Color, position: Vec2) -> Self {
        Self {
            position: Position(position),
            color: PlayerColor(color),
        }
    }
}
