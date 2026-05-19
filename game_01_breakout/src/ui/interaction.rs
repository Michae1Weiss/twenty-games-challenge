use bevy::prelude::*;

/// Drives BackgroundColor from Interaction. Attach to any button-like entity.
#[derive(Component, Clone, Copy, Debug)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, apply_interaction_palette);
}

fn apply_interaction_palette(
    mut query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut bg) in &mut query {
        bg.0 = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        };
    }
}
