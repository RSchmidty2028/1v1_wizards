//! Raylib helper functions
use raylib::prelude::*;
use rand::Rng;

pub fn check_collision_point_rect(point: &Vector2, rect: &Rectangle) -> bool {
    let in_x = point.x >= rect.x && point.x <= rect.x + rect.width;
    let in_y = point.y >= rect.y && point.y <= rect.y + rect.height;

    return in_x && in_y;
}

pub fn random_point(width: i32, height: i32) -> Vector2 {
    let mut rng = rand::rng();

    let x = rng.random_range(0..width);
    let y = rng.random_range(0..height);

    Vector2{x: x as f32, y: y as f32}
}
// checks if a circle (projectile) hits a rectangle (player)
pub fn check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
    let mut test_x = center.x;
    let mut test_y = center.y;

    // find the closest edge of the rectangle to the circle
    if center.x < rec.x { test_x = rec.x; }
    else if center.x > rec.x + rec.width { test_x = rec.x + rec.width; }

    if center.y < rec.y { test_y = rec.y; }
    else if center.y > rec.y + rec.height { test_y = rec.y + rec.height; }

    // calculate distance from closest edge
    let dist_x = center.x - test_x;
    let dist_y = center.y - test_y;
    let distance = (dist_x*dist_x + dist_y*dist_y).sqrt();

    // if distance is less than radius, they are touching
    return distance <= radius;
}