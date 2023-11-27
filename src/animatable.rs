// src/animatable.rs

pub trait Animatable: Clone {
    fn interpolate(start: &Self, end: &Self, progress: f64) -> Self;

    // Returns the difference between two animatable values
    //fn subtract(start: &Self, end: &Self) -> Self;

    // Scales an animatable value by a scalar
    //fn multiply_by_scalar(value: &Self, scalar: f64) -> Self;
}

pub struct AnimatableProperties {
    // Fields representing animatable properties
}


// BUILT-IN TYPES
impl Animatable for f32 {
    fn interpolate(start: &f32, end: &f32, progress: f64) -> Self {
        start + ((end - start) as f64 * progress) as f32
    }
}

impl Animatable for f64 {
    fn interpolate(start: &f64, end: &f64, progress: f64) -> Self {
        start + (end - start) * progress
    }
}