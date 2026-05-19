use super::interaction::InteractionPalette;
use crate::theme::palette::*;
use bevy::{
    ecs::{
        spawn::SpawnWith,
        system::{IntoObserverSystem, entity_command::observe},
    },
    prelude::*,
    ui_widgets::{Slider, SliderRange, SliderStep, SliderThumb, SliderValue, ValueChange},
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

pub fn header(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font_size(48.0),
        TextColor(HEADER_TEXT),
    )
}

pub fn label(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont::from_font_size(20.0),
        TextColor(LABEL_TEXT),
    )
}

/// Standard menu button. `action` is any observer system: a closure or a fn.
///
/// Examples:
///   button("Play",     |_: On<Pointer<Click>>, mut next: ResMut<NextState<AppState>>| {
///       next.set(AppState::InGame);
///   })
///   button("Quit",     on_quit_pressed)   // where fn on_quit_pressed(_: On<Pointer<Click>>, ...) {}
pub fn button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
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
                    BackgroundColor(BUTTON_BACKGROUND),
                    InteractionPalette {
                        none: BUTTON_BACKGROUND,
                        hovered: BUTTON_HOVERED,
                        pressed: BUTTON_PRESSED,
                    },
                    children![(
                        Name::new("Button Text"),
                        Text(text),
                        TextFont::from_font_size(28.0),
                        TextColor(BUTTON_TEXT),
                        // Clicks on the text should bubble to the button, not be intercepted.
                        Pickable::IGNORE,
                    )],
                ))
                .observe(action);
        })),
    )
}

/// A labeled, 10-step slider row. `on_change` is called whenever the value changes.
///
/// Example:
///   slider_row("Music", current_music_volume, |v: On<ValueChange<f32>>, mut s: ResMut<AudioSettings>| {
///       s.music_volume = v.value;
///   })
pub fn slider_row<M, I>(label_text: impl Into<String>, initial: f32, on_change: I) -> impl Bundle
where
    I: IntoObserverSystem<ValueChange<f32>, (), M>,
{
    // FIXME
    unimplemented!();
    let label_text = label_text.into();
    (
        Name::new("SliderRow"),
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn((
                Name::new("SliderRow"),
                Node::default(),
                children![
                    // Label
                    (
                        Name::new("SliderLabel"),
                        Node {
                            width: px(120),
                            ..default()
                        },
                        Text(label_text),
                        TextFont::from_font_size(20.0),
                        TextColor(LABEL_TEXT),
                    ),
                    // Slider
                    (
                        Name::new("Slider"),
                        Slider::default(),
                        SliderValue(0.1),
                        // SliderValue(initial),
                        SliderRange::new(0.0, 1.0),
                        SliderStep(0.1), // 10 discrete steps
                        Node {
                            width: px(260),
                            height: px(20),
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(SLIDER_TRACK),
                        // BorderRadius::all(px(10)),
                        // observe(on_change),
                        children![(
                            Name::new("SliderThumb"),
                            SliderThumb,
                            Node {
                                position_type: PositionType::Absolute,
                                width: px(24),
                                height: px(24),
                                left: percent(0), // updated by the widget plugin
                                ..default()
                            },
                            BackgroundColor(SLIDER_THUMB),
                            // BorderRadius::MAX,
                        )],
                    ),
                ],
            ));
        })),
        // children![
        //     // Label
        //     (
        //         Name::new("SliderLabel"),
        //         Node {
        //             width: px(120),
        //             ..default()
        //         },
        //         // Text(label_text),
        //         TextFont::from_font_size(20.0),
        //         TextColor(LABEL_TEXT),
        //     ),
        //     // Slider
        //     (
        //         Name::new("Slider"),
        //         Slider::default(),
        //         SliderValue(initial),
        //         SliderRange::new(0.0, 1.0),
        //         SliderStep(0.1), // 10 discrete steps
        //         Node {
        //             width: px(260),
        //             height: px(20),
        //             justify_content: JustifyContent::Start,
        //             align_items: AlignItems::Center,
        //             ..default()
        //         },
        //         BackgroundColor(SLIDER_TRACK),
        //         // BorderRadius::all(px(10)),
        //         // observe(on_change),
        //         children![(
        //             Name::new("SliderThumb"),
        //             SliderThumb,
        //             Node {
        //                 position_type: PositionType::Absolute,
        //                 width: px(24),
        //                 height: px(24),
        //                 left: percent(0), // updated by the widget plugin
        //                 ..default()
        //             },
        //             BackgroundColor(SLIDER_THUMB),
        //             // BorderRadius::MAX,
        //         )],
        //     ),
        // ],
    )
}
