// ui/stepper.rs
use super::interaction::InteractionPalette;
use crate::theme::palette::*;
use bevy::{
    ecs::spawn::{Spawn, SpawnIter},
    prelude::*,
};

/// A discrete value control:  label above,  [−] ▮▮▯▯… [+]  below.
/// Generic and self-owned — bind it to game state with a marker + a
/// `Changed<Stepper>` system (see settings_menu).
#[derive(Component)]
pub struct Stepper {
    pub value: u8,
    pub max: u8,
}

#[derive(Component)]
struct StepperButton {
    delta: i8,
}

#[derive(Component)]
struct Segment(u8);

// Portrait segments read as a level meter. For literal brick consistency
// (2:1 landscape) swap to (36, 18) — but portrait is the right UX, see below.
const SEG_W: f32 = 18.0;
const SEG_H: f32 = 36.0;
const STEP_BTN: f32 = 48.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (handle_stepper_buttons, recolor_segments).chain());
}

/// The whole widget as one declarative bundle. Composes inside `children![]`.
pub fn stepper(label: impl Into<String>, initial: u8, max: u8, marker: impl Bundle) -> impl Bundle {
    (
        Name::new("Stepper"),
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: px(8),
            ..default()
        },
        children![
            (
                Name::new("StepperLabel"),
                Text(label.into()),
                TextFont::from_font_size(20.0),
                TextColor(LABEL_TEXT),
            ),
            stepper_row(initial, max, marker),
        ],
    )
}

/// The [−] segments… [+] row. `Stepper` lives here so buttons/segments are
/// its DIRECT children — one `ChildOf` hop, no hierarchy walking.
fn stepper_row(initial: u8, max: u8, marker: impl Bundle) -> impl Bundle {
    (
        Name::new("StepperRow"),
        Stepper {
            value: initial,
            max,
        },
        marker,
        Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: px(4),
            ..default()
        },
        // Static button + dynamic segments + static button: the declarative
        // composition the `children!` macro can't express on its own.
        Children::spawn((
            Spawn(step_button("-", -1)),
            SpawnIter((0..max).map(Segment).map(segment)),
            Spawn(step_button("+", 1)),
        )),
    )
}

fn segment(seg: Segment) -> impl Bundle {
    (
        seg,
        Node {
            width: px(SEG_W),
            height: px(SEG_H),
            border_radius: BorderRadius::all(px(3)),
            ..default()
        },
        BackgroundColor(SEGMENT_EMPTY), // recolored on spawn (counts as Changed)
                                        // BorderRadius::all(px(3)),
    )
}

fn step_button(glyph: &str, delta: i8) -> impl Bundle {
    (
        Button,
        StepperButton { delta }, // carries the action data → no observer needed
        Node {
            width: px(STEP_BTN),
            height: px(STEP_BTN),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border_radius: BorderRadius::all(px(8)),
            ..default()
        },
        BackgroundColor(BUTTON_BACKGROUND),
        // BorderRadius::all(px(8)),
        InteractionPalette {
            none: BUTTON_BACKGROUND,
            hovered: BUTTON_HOVERED,
            pressed: BUTTON_PRESSED,
        },
        children![(
            Text(glyph.to_string()),
            TextFont::from_font_size(28.0),
            TextColor(BUTTON_TEXT),
            Pickable::IGNORE,
        )],
    )
}

/// One system for every −/+ button in the app. The button's parent IS the
/// Stepper (direct child), so `child_of.parent()` resolves it in one hop.
fn handle_stepper_buttons(
    buttons: Query<(&Interaction, &StepperButton, &ChildOf), Changed<Interaction>>,
    mut steppers: Query<&mut Stepper>,
) {
    for (interaction, button, child_of) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok(mut stepper) = steppers.get_mut(child_of.parent()) {
            let next = stepper.value as i16 + button.delta as i16;
            stepper.value = next.clamp(0, stepper.max as i16) as u8;
        }
    }
}

/// Fills segments below `value`, empties the rest. Reacts to `Changed<Stepper>`
/// (incl. spawn). The InteractionPalette equivalent for steppers — one system,
/// any number of steppers.
fn recolor_segments(
    steppers: Query<(Entity, &Stepper), Changed<Stepper>>,
    children: Query<&Children>,
    mut segments: Query<(&Segment, &mut BackgroundColor)>,
) {
    for (entity, stepper) in &steppers {
        for descendant in children.iter_descendants(entity) {
            if let Ok((seg, mut bg)) = segments.get_mut(descendant) {
                bg.0 = if seg.0 < stepper.value {
                    SEGMENT_FILLED
                } else {
                    SEGMENT_EMPTY
                };
            }
        }
    }
}
