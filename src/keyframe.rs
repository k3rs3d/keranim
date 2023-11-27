// src/keyframe.rs

pub struct Keyframe<T> {
    pub timestamp: f64,
    pub value: T,
}