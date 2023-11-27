// lib.rs

mod animatable;
pub use animatable::{Animatable, AnimatableProperties};

mod keyframe;
pub use keyframe::Keyframe;

mod animation_track;
pub use animation_track::AnimationTrack;

mod animation_timeline;
pub use animation_timeline::AnimationTimeline;

mod end_behavior;
pub use end_behavior::EndBehavior;