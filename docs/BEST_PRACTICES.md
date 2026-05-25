## If we need to make components to depend on each other
### Use-Cases
1. Each entity with a component `Enemy` MUST have `Health` component 
### How-to?
Use `#[require(...)]` macros. The `Default` constructor will be used to initialize the component, by default.
If required component was manually added, it will _not_ implicitly inserted, because it was already provided! 
Components can have more than one required component `#[require(B, C)]`
You can even define inline component values!
You can also define arbitrary expressions by using `=`
PLEASE read second doc link, it is REALLY GOOD DOCUMENTED!
```rust
#[derive(Component)]
struct Health;

#[derive(Component)]
#[require(Health)]
struct Enemy;
```

### Links:
- [**tainedcoders.com** - Bevy Commands](https://taintedcoders.com/bevy/commands)
- [**docs.rs/bevy** - Required Components](https://docs.rs/bevy/latest/bevy/prelude/trait.Component.html#required-components)

## When to write custom commands?
### Links:
- [**tainedcoders.com** - Custom Commands](https://taintedcoders.com/bevy/patterns/custom-commands)

## When to write custom queries?
### Links:
- [**tainedcoders.com** - Custom Queries](https://taintedcoders.com/bevy/patterns/custom-queries)

## Pause the game (clean way)?
Shows how `Time<Virtual>` can be used to pause, resume, slow down and speed up a game.
### Links:
- [**github/bevy/examples** - time/virtual_time.rs](https://github.com/bevyengine/bevy/blob/main/examples/time/virtual_time.rs)
- [**docs.rs/bevy** - bevy::time::Virtual](https://docs.rs/bevy/latest/bevy/time/struct.Virtual.html)

## React on deletion
### Links:
- [**github/bevy/examples** - ecs/removal_detection.rs](https://github.com/bevyengine/bevy/blob/main/examples/ecs/removal_detection.rs)
- [**docs.rs/bevy** - bevy::prelude::RemovedComponents](https://docs.rs/bevy/latest/bevy/prelude/struct.RemovedComponents.html)

## How to slow gameplay down?
### Links
- [**github/bevy/examples** - time/virtual_time.rs](https://github.com/bevyengine/bevy/blob/main/examples/time/virtual_time.rs)

## How to group systems? For examle systems that should be pausable?
Use `SystemSet`
### Links
- [**docs.rs/bevy** - bevy::prelude::SystemSet](https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.SystemSet.html)
- [**github/TheBevyFlock/bevy_new_2d** - main/src/main.rs](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/src/main.rs#L97)

## Input handling, how to make it?
### Links
- [**docs.rs/bevy_enhanced_input** - Crate bevy_enhanced_input](https://docs.rs/bevy_enhanced_input/latest/bevy_enhanced_input/)
- [**github** - leafwing-input-manager](https://github.com/Leafwing-Studios/leafwing-input-manager)

## Fullscreen shader?
