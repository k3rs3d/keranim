// src/animatable.rs

pub trait Animatable<T> {
    fn interpolate(&self, end: &Self, progress: f64) -> Self;
}

pub struct AnimatableProperties {
    // Fields representing animatable properties
}