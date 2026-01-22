//! Raylib helper functions
use raylib::prelude::*;
use rand::Rng;

/// Checks if a 2D point is within the bounds of a rectangle.
/// Used for menu button clicks.
pub fn check_collision_point_rect(point: &Vector2, rect: &Rectangle) -> bool {
    let in_x = point.x >= rect.x && point.x <= rect.x + rect.width;
    let in_y = point.y >= rect.y && point.y <= rect.y + rect.height;

    return in_x && in_y;
}

/// Generates a random Vector2 within the specified width and height.
pub fn random_point(width: i32, height: i32) -> Vector2 {
    let mut rng = rand::rng();

    let x = rng.random_range(0..width);
    let y = rng.random_range(0..height);

    Vector2{x: x as f32, y: y as f32}
}

/// Checks if a circle (projectile) hits a rectangle (player hitbox).
pub fn check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
    let mut test_x = center.x;
    let mut test_y = center.y;

    // Find the closest edge of the rectangle to the circle
    if center.x < rec.x { test_x = rec.x; }
    else if center.x > rec.x + rec.width { test_x = rec.x + rec.width; }

    if center.y < rec.y { test_y = rec.y; }
    else if center.y > rec.y + rec.height { test_y = rec.y + rec.height; }

    // Calculate distance from closest edge
    let dist_x = center.x - test_x;
    let dist_y = center.y - test_y;
    let distance = (dist_x*dist_x + dist_y*dist_y).sqrt();

    // If distance is less than radius, they are touching
    return distance <= radius;
}

pub fn draw_lava_world(
    d: &mut RaylibMode2D<RaylibDrawHandle>,
    lava_tex: &Texture2D,
    lava_y: f32,
    camera_y: f32,
) {
    let lava_height = 128.0;
    let tile_width = lava_tex.width() as f32;

    // Draw lava across the screen
    let start_x = (camera_y as i32 / tile_width as i32 - 5) * tile_width as i32;

    for i in -10..30 {
        d.draw_texture_pro(
            lava_tex,
            Rectangle::new(0.0, 0.0, lava_tex.width() as f32, lava_tex.height() as f32),
            Rectangle::new(
                start_x as f32 + i as f32 * tile_width,
                lava_y,
                tile_width,
                lava_height,
            ),
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
}
