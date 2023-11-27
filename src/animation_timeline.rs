// src/animation_timeline.rs

use crate::{Animatable, AnimationTrack, EndBehavior};

pub struct AnimationTimeline<T> where T: Animatable {
    // Hold multiple tracks
    pub tracks: Vec<AnimationTrack<T>>,
    default_end_behavior: EndBehavior, // Timeline-wide default end behavior
}

impl<T> AnimationTimeline<T> where T: Animatable {
    pub fn new() -> Self {
        Self { tracks: Vec::new(), default_end_behavior: EndBehavior::Stop }
    }

    pub fn set_default_end_behavior(&mut self, behavior: EndBehavior) {
        self.default_end_behavior = behavior;
    }

    // Adds an AnimationTrack to this AnimationTimeline
    pub fn add_track(&mut self, track: AnimationTrack<T>) {
        self.tracks.push(track);
    }

    // Update will call interpolate on all tracks and then perform some logic
    // to apply the updated values back to the target properties
    pub fn update(&self, current_time: f64) -> Vec<Option<T>> {
        self.tracks.iter().map(|track| {
            // Determine which end behavior to use
            let behavior = track.end_behavior.as_ref().unwrap_or(&self.default_end_behavior);
            // Call a method on the track that factors in the end_behavior
            track.interpolate_with_behavior(current_time, behavior)
        }).collect()
    }
}