// animation_track.rs

use crate::{Animatable, Keyframe, EndBehavior};

pub struct AnimationTrack<T> {
    keyframes: Vec<Keyframe<T>>,
    pub end_behavior: Option<EndBehavior>, // End behavior is optional
}

impl<T> AnimationTrack<T>
where
    T: Animatable + Clone,
{
    pub fn new() -> Self {
        Self {
            keyframes: Vec::new(),
            end_behavior: None,
        }
    }

    pub fn set_end_behavior(&mut self, behavior: EndBehavior) {
        self.end_behavior = Some(behavior);
    }

    pub fn add_keyframe(&mut self, timestamp: f64, value: T) {
        // Might want to keep the keyframes sorted or insert them in time order
        self.keyframes.push(Keyframe { timestamp, value });
        self.keyframes
            .sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());
    }

    // Method to get interpolated value with consideration for end behavior
    pub fn interpolate_with_behavior(
        &self,
        current_time: f64,
        end_behavior: &EndBehavior,
    ) -> Option<T> {
        // If no keyframes are present, return None
        if self.keyframes.is_empty() {
            return None;
        }

        // Check if we're beyond the last keyframe and apply end behavior if so
        if let Some(last_keyframe) = self.keyframes.last() {
            if current_time >= last_keyframe.timestamp {
                // Calculate the total duration of the animation track
                let first_timestamp = self.keyframes.first().map(|kf| kf.timestamp).unwrap_or(0.0);
                let duration = last_keyframe.timestamp - first_timestamp;

                match self.end_behavior.as_ref().unwrap_or(&EndBehavior::Stop) {
                    EndBehavior::Stop => {
                        // Simply return the last keyframe's value
                        return Some(last_keyframe.value.clone());
                    }
                    EndBehavior::Loop => {
                        // Loop the animation from the start
                        if duration > 0.0 {
                            let looped_time = current_time % duration;
                            return self.interpolate(looped_time);
                        }
                        return Some(last_keyframe.value.clone());
                    }
                    EndBehavior::Reverse => {
                        // Reverse the animation
                        if duration > 0.0 {
                            let mut reverse_time = current_time % (duration * 2.0);
                            if reverse_time > duration {
                                reverse_time = 2.0 * duration - reverse_time;
                            }
                            return self.interpolate(reverse_time);
                        }
                        return Some(last_keyframe.value.clone());
                    }
                    // TODO: "Extrapolate" end behavior
                    /*
                    EndBehavior::Extrapolate => {
                        if self.keyframes.len() >= 2 {
                            let last_keyframe = self.keyframes.last().unwrap();
                            let second_last_keyframe = self.keyframes.get(self.keyframes.len() - 2).unwrap();
                            let extrapolation_time = current_time - last_keyframe.timestamp;
                            let difference_in_values = T::subtract(&last_keyframe.value, &second_last_keyframe.value);
                            let extrapolated_value = T::multiply_by_scalar(&difference_in_values, extrapolation_time);
                            Some(last_keyframe.value.clone() + extrapolated_value)
                        } else {
                            Some(last_keyframe.value.clone())
                        }
                    },
                    */
                    EndBehavior::Rebound => {
                        // Return the value of the first keyframe. Assumes keyframes is non-empty.
                        return Some(self.keyframes.first().unwrap().value.clone())
                    },
                    _ => {
                        // Other end behaviors not yet implemented?
                    }
                }
            }
        }

        // If we're before the last keyframe or implementing other behaviors,
        // use the standard interpolation method
        self.interpolate(current_time)
    }

    // Get interpolated value
    pub fn interpolate(&self, current_time: f64) -> Option<T> {
        // If no keyframes, return None
        if self.keyframes.is_empty() {
            return None;
        }

        // Get two keyframes to interpolate between
        let (start_frame, end_frame) = self.get_interpolation_frames(current_time)?;

        let progress =
            (current_time - start_frame.timestamp) / (end_frame.timestamp - start_frame.timestamp);

        Some(T::interpolate(
            &start_frame.value,
            &end_frame.value,
            progress,
        ))
    }

    // Helper method to find the keyframes we're interpolating between
    fn get_interpolation_frames(&self, current_time: f64) -> Option<(&Keyframe<T>, &Keyframe<T>)> {
        let mut start_frame = &self.keyframes[0];
        let mut end_frame = &self.keyframes[0];

        for frame in &self.keyframes {
            if frame.timestamp <= current_time {
                start_frame = frame;
            } else {
                end_frame = frame;
                break;
            }
        }

        // Return the two keyframes if they are different, otherwise None
        if start_frame.timestamp != end_frame.timestamp {
            Some((start_frame, end_frame))
        } else {
            None
        }
    }
}
