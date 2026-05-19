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
