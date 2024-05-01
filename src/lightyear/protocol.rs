use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use lightyear::client::components::{ComponentSyncMode, Confirmed};
use lightyear::prelude::*;

use crate::shared::{PlayerColor, Position};

// Components

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PlayerId(pub ClientId);

#[derive(Component, Deserialize, Serialize, Clone, Debug, PartialEq)]
// Marker component
pub struct CircleMarker;

// Channels

#[derive(Channel)]
pub struct Channel1;

// Messages

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message1(pub usize);

// Protocol
pub(crate) struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // messages
        app.add_message::<Message1>(ChannelDirection::Bidirectional);

        // components
        app.register_component::<PlayerId>(ChannelDirection::ServerToClient);
        app.add_prediction::<PlayerId>(ComponentSyncMode::Once);
        app.add_interpolation::<PlayerId>(ComponentSyncMode::Once);

        app.register_component::<Position>(ChannelDirection::Bidirectional);
        app.add_prediction::<Position>(ComponentSyncMode::Full);
        app.add_interpolation::<Position>(ComponentSyncMode::Full);
        app.add_linear_interpolation_fn::<Position>();

        app.register_component::<PlayerColor>(ChannelDirection::ServerToClient);
        app.add_prediction::<PlayerColor>(ComponentSyncMode::Once);
        app.add_interpolation::<PlayerColor>(ComponentSyncMode::Once);

        app.register_component::<CircleMarker>(ChannelDirection::ServerToClient);
        app.add_prediction::<CircleMarker>(ComponentSyncMode::Once);
        app.add_interpolation::<CircleMarker>(ComponentSyncMode::Once);
        // channels
        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });
    }
}

/// Generate a color from the `ClientId`
pub(crate) fn color_from_id(client_id: ClientId) -> Color {
    let h = (((client_id.to_bits().wrapping_mul(30)) % 360) as f32) / 360.0;
    let s = 1.0;
    let l = 0.5;
    Color::hsl(h, s, l)
}

/// System that draws the boxed of the player positions.
/// The components should be replicated from the server to the client
/// This time we will only draw the predicted/interpolated entities
pub(crate) fn draw_boxes(
    mut gizmos: Gizmos,
    players: Query<(&Position, &PlayerColor), Without<Confirmed>>,
) {
    for (position, color) in &players {
        gizmos.rect(
            Vec3::new(position.x, position.y, 0.0),
            Quat::IDENTITY,
            Vec2::ONE * 50.0,
            color.0,
        );
    }
}
