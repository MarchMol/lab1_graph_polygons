mod color;
use color::Color;
use framebuffer::Framebuffer;
use nalgebra_glm::Vec3;
use poly::Polygon;
mod bmp;
mod line;
mod poly;

mod framebuffer;

fn main() {
    //Init
    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_bgcolor(0x000000);
    framebuffer.clear();

    // Polygon 1
    let poly1 = [
        (165, 380),
        (185, 360),
        (180, 330),
        (207, 345),
        (233, 330),
        (230, 360),
        (250, 380),
        (220, 385),
        (205, 410),
        (193, 383),
    ];
    let poly1_v: Vec<Vec3> = poly1
        .iter()
        .map(|&(x, y)| Vec3::new(x as f32, y as f32, 0.0))
        .collect();

    // Polygon 2
    let poly2 = [(321, 335), (288, 286), (339, 251), (374, 302)];
    let poly2_v: Vec<Vec3> = poly2
        .iter()
        .map(|&(x, y)| Vec3::new(x as f32, y as f32, 0.0))
        .collect();

    // colors
    let white = Color::from_hex(0xFFFFFF);
    let yellow = Color::from_hex(0xffe100);
    let blue = Color::from_hex(0x001eff);

    //drawing
    framebuffer.draw_polygon(&poly1_v, white, yellow);
    framebuffer.draw_polygon(&poly2_v, white, blue);

    //Render
    framebuffer.render("poligon2.bmp");
}
