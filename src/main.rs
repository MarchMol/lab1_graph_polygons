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

    // Polygon 3
    let poly3 = [
        (377, 249), (411, 197), (436, 249),
    ];
    let poly3_v: Vec<Vec3> = poly3.iter().map(|&(x, y)| Vec3::new(x as f32, y as f32, 0.0)).collect();
    
    // Polygon 4
    let poly4 = [
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ];
    let poly4_v: Vec<Vec3> = poly4.iter().map(|&(x, y)| Vec3::new(x as f32, y as f32, 0.0)).collect();


    // Polygon 5
    let poly5 = [
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];
    let poly5_v: Vec<Vec3> = poly5.iter().map(|&(x, y)| Vec3::new(x as f32, y as f32, 0.0)).collect();

    // colors
    let white = Color::from_hex(0xFFFFFF);
    let yellow = Color::from_hex(0xffe100);
    let blue = Color::from_hex(0x001eff);
    let red = Color::from_hex(0xff0000);
    let green = Color::from_hex(0x00ff0d);
    let black = Color::from_hex(0x000000);

    //drawing
    framebuffer.draw_polygon(&poly1_v, white, yellow);
    framebuffer.draw_polygon(&poly2_v, white, blue);
    framebuffer.draw_polygon(&poly3_v, white, red);

    framebuffer.draw_polygon(&poly4_v, white,green);
    framebuffer.draw_polygon(&poly5_v, white,black);
    //Render
    framebuffer.render("polygon4.bmp");
}
