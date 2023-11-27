# Rust Animation Library

A very early work-in-progress. This will become a flexible and powerful Rust library for animating arbitrary values/data types using keyframes and timelines.

## Features

- **Type-Agnostic Animations**: Animate any type that implements the `Animatable` trait.
- **Keyframe-Based**: Define animation steps with keyframes for precise control.
- **Flexible Timelines**: Compose multiple animation tracks in parallel with independent timelines.
- **Easing & Interpolation**: Support for linear and more advanced easing functions.
- **End Behaviors**: Customize the behavior of animations after they finish (including loop, reverse, stop, and more).
- **Macro Support**: Simplify animation setup with macros to reduce verbosity.

## Getting Started

Add the animation library to your project's `Cargo.toml`

## Usage 

```
use keranim::{Animatable, AnimationTrack, AnimationTimeline, EndBehavior};

// Implement Animatable for your custom type
#[derive(Clone, Debug)]
pub struct MyValueType {
    // Your fields
}

impl Animatable for MyValueType {
    // Your interpolation logic
}

// Set up your animation
let mut timeline = AnimationTimeline::new();
let mut track = AnimationTrack::new();

track.add_keyframe(0.0, MyValueType { /* initial state */ });
track.add_keyframe(1.0, MyValueType { /* final state */ });

timeline.add_track(track);
```

Update your animation somewhere in the application logic, such as a game update loop:

```
// Inside your update loop
let elapsed_time = get_elapsed_time();
let animated_properties = timeline.update(elapsed_time);

// Apply the animated properties to your objects
```

Includes a macro for less boilerplate:

```
create_animation_track!(MyModel {
    field: MyValueType {
        from: (0.0, initial_state),
        to: (1.0, final_state),
        end_behavior: EndBehavior::Loop,
    },
});
```
