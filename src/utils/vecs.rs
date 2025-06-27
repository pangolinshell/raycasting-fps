#![allow(unused)]
use sdl2::rect::FPoint;

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

pub fn direct_to(from: FPoint, to: FPoint) -> f32 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    dy.atan2(dx)
}

pub fn delta(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (dx * dx + dy * dy).sqrt()
}

pub fn go_toward(from: FPoint, to: FPoint, distance: f32) -> FPoint {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let length = (dx * dx + dy * dy).sqrt();

    if length == 0.0 {
        return from; // Pas de direction, on reste au même point
    }

    let unit_x = dx / length;
    let unit_y = dy / length;

    FPoint::new(from.x + unit_x * distance, from.y + unit_y * distance)
}
