// src/animation_timeline.rs

use crate::{AnimationTrack, AnimatableProperties};

pub struct AnimationTimeline {
    //pub color_track: AnimationTrack<Color>,
    //pub position_track: AnimationTrack<Vec3>,
    // Add more tracks as needed
}

impl AnimationTimeline {
    pub fn new() -> Self {
        // Initialize the timeline with tracks
        unimplemented!();
    }

    pub fn update(&self, current_time: f64) -> AnimatableProperties {
        // Update logic to interpolate values from tracks
        unimplemented!();
    }
}