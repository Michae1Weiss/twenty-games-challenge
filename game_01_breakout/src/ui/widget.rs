use super::interaction::InteractionPalette;
use crate::{audio::PlaySfx, theme::palette::*};
use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
};
use std::borrow::Cow;

/// Full-screen centered column. Use as the root of any menu screen.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(16),
            ..default()
        },
        BackgroundColor(SCREEN_BACKGROUND),
        // Don't block underlying picking when used as an overlay.
        Pickable::IGNORE,
    )
}

pub fn header(text: impl Into<String>, font: Handle<Font>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont {
            font,
            font_size: 48.0,
            ..default()
        },
        TextColor(HEADER_TEXT),
    )
}

/// Standard menu button. `action` is any observer system: a closure or a fn.
///
/// Examples:
///   button("Play",     |_: On<Pointer<Click>>, mut next: ResMut<NextState<AppState>>| {
///       next.set(AppState::InGame);
///   })
///   button("Quit",     on_quit_pressed)   // where fn on_quit_pressed(_: On<Pointer<Click>>, ...) {}
pub fn button<E, B, M, I>(
    text: impl Into<String>,
    image: Handle<Image>,
    font: Handle<Font>,
    action: I,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    Node {
                        width: px(320),
                        height: px(64),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    // BackgroundColor(BUTTON_BACKGROUND),
                    ImageNode::new(image),
                    InteractionPalette {
                        none: BUTTON_BACKGROUND,
                        hovered: BUTTON_HOVERED,
                        pressed: BUTTON_PRESSED,
                    },
                    children![(
                        Name::new("Button Text"),
                        Text(text),
                        TextFont {
                            font,
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(BUTTON_TEXT),
                        // Clicks on the text should bubble to the button, not be intercepted.
                        Pickable::IGNORE,
                    )],
                ))
                .observe(action)
                .observe(|_: On<Pointer<Over>>, mut sfx: MessageWriter<PlaySfx>| {
                    info!("Over");
                    sfx.write(PlaySfx::ButtonHover);
                })
                .observe(|_: On<Pointer<Press>>, mut sfx: MessageWriter<PlaySfx>| {
                    info!("Press");
                    sfx.write(PlaySfx::ButtonPress);
                });
        })),
    )
}
