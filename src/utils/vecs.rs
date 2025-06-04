#[allow(unused)]
/// Converts an angle in radians to a 2D unit vector (x, y).
///
/// # Arguments
///
/// * `angle` - The angle in radians, where 0 points along the positive x-axis.
///
/// # Returns
///
/// A tuple `(x, y)` representing the unit vector in the direction of the given angle.
///
/// # Example
///
/// ```
/// let (x, y) = from_direction(std::f32::consts::FRAC_PI_2);
/// assert!((x - 0.0).abs() < 1e-6);
/// assert!((y - 1.0).abs() < 1e-6);
/// ```
pub fn from_direction(angle: f32) -> (f32,f32) {
    (angle.cos(),angle.sin())
}