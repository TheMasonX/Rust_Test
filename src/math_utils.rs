use num_traits::Num;

/// Clamps `val` between `min_val` and `max_val`.
///
/// # Examples
///
/// ```
/// # use tmx_utils::math_utils::clamp;
/// assert_eq!(clamp(3,  0, 2), 2);
/// assert_eq!(clamp(1,  0, 2), 1);
/// assert_eq!(clamp(-1.0, 0.0, 1.0), 0.0);
/// ```
pub fn clamp<T: PartialOrd + Num>(val: T, min_val: T, max_val: T) -> T {
    match val {
        v if v < min_val => min_val,
        v if v > max_val => max_val,
        _ => val,
    }
}

/// Clamps `val` between 0 and 1. Had intended to use `clamp(val, 0, 1)`, but float doesn't implement the Ord trait.
///
/// # Examples
///
/// ```
/// # use tmx_utils::math_utils::clamp01;
/// assert_eq!(clamp01(0.5), 0.5);
/// assert_eq!(clamp01(-1), 0);
/// assert_eq!(clamp01(3), 1);
/// assert_eq!(clamp01(3.0), 1.0);
/// ```
pub fn clamp01<T: PartialOrd + Num>(val: T) -> T {
    let zero = T::zero();
    let one = T::one();

    match val {
        v if v < zero => zero,
        v if v > one => one,
        _ => val,
    }
}

/// Linearly interpolates between `a` and `b` by `t`. Does NOT clamp 't' between 0 and 1, allowing for extrapolation.
///
/// # Examples
///
/// ```
/// # use tmx_utils::math_utils::lerp;
/// assert_eq!(lerp(0, 1, 1), 1);
/// assert_eq!(lerp(0.0, 1.0, 0.5), 0.5);
/// assert_eq!(lerp(0.0, 1.0, 2.0), 2.0);
/// assert_eq!(lerp::<f32>(-1.0, 1.0, 0.5), 0.0); // Can force casting
/// ```
pub fn lerp<T: Num + Copy>(a: T, b: T, t: T) -> T {
    a + (b - a) * t
}

/// Linearly interpolates between `a` and `b` by `t`. Clamps 't' between 0 and 1.
///
/// # Examples
///
/// ```
/// # use tmx_utils::math_utils::lerp_clamped;
/// assert_eq!(lerp_clamped(0, 1, 2), 1);
/// assert_eq!(lerp_clamped(0.0, 1.0, -0.01), 0.0);
/// ```
pub fn lerp_clamped<T: PartialOrd + Num + Copy>(a: T, b: T, t: T) -> T {
    a + (b - a) * clamp01(t)
}
