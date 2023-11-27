// src/end_behavior.rs

pub enum EndBehavior {
    Stop,
    Extrapolate,
    Reverse,
    Loop,
    Rebound,

    // A special variant to indicate that the track should use the timeline's default behavior
    UseTimelineDefault,
}