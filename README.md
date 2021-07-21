# bevy_swarm
Swarm stands for Static Utility AI Reasoner Macro(if you replace the U with W :D).
Like the name says, this crate ships a macro to create action-components that hold the score for themselves (and bundles for them), and reasoner systems to decide what action was best scored. 
The name has static in it because it relies on Rust's type system and bevy's static queries, this means you need to define all actions at compile time.

## How to use
Well, the main feature of this crate is the reasoner macro, so let's use it and see what it does: 
For the following macro call, for example:
```rs
create_reasoner! {
    Dwarf{
        Eat,
        Sleep
    }
}
```
the macro "create_reasoner" would generate the following things:
```rs
// A component that holds the currently highest scored action, this component is mutated in
// the determine_next_action_dwarf_actions reasoner system
#[derive(Debug,Clone,EnumDefault)]
pub enum DwarfActions {
    Eat,
    Sleep
}
// These are Action components, they're newtypes wrapping an f32
// Your scoring systems would need to mutate their value to affect the chosen DwarfActions // // variant
#[derive(Debug,Default)]
pub struct Eat(f32);
#[derive(Debug,Default)]
pub struct Sleep(f32);

// This is a regular bevy bundle that you add to your dwarf entities
#[derive(bevy::prelude::Bundle,Default)]
pub struct DwarfActionsBundle{
    pub eat:Eat,
    pub sleep:Sleep,
    pub dwarf_actions:DwarfActions
}
/// This is the reasoner systems that chooses our next action
fn determine_next_action_dwarf_actions(
    mut query: Query<(
        &Eat,
        &Sleep,
        &mut DwarfActions,
    )>,
) {
    for (eat, sleep, mut dwarf_actions) in
        query.iter_mut()
    {
        let array = vec![
            (eat.0, DwarfActions::Eat),
            (sleep.0, DwarfActions::Sleep),

        ];
        let mut index_max: usize = 0;
        let mut temp = f32::MIN;
        for (index, (item, _action)) in array.iter().enumerate() {
            if *item > temp {
                temp = *item;
                index_max = index;
            }
        }
        *dwarf_actions = array[index_max].1.clone();
    }
}
```

This means you can just insert the generated bundle on any entity, and if you run the generated reasoner system, it will decide what action is needed to be taken next.

Scoring systems are thus also regular bevy systems that could be ran in parralel to each other since they have nothing to do with each other, you just have to make sure the reasoner is ran after all scores are updated.

This crate only ships the reasoner macro, it doesn't have any opinion on how to score actions or what do to with them after the reasoner has chosen an action!
You should perhaps use a tweening library, easer was quite nice :D.

## License

bevy_swarm is primarily distributed under the terms of either the MIT license or the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
