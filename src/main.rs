mod color;
use color::Color;
use framebuffer::Framebuffer;
use nalgebra_glm::Vec3;
use poly::Polygon;
mod bmp;
mod poly;
mod line;


mod framebuffer;

fn main(){
    //Init
    let mut framebuffer = Framebuffer::new(800,600);
    framebuffer.set_bgcolor(0x000000);
    framebuffer.clear();

    // Polygon 1
    let poly1 = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];
    let poly1_v: Vec<Vec3> = poly1.iter().map(|&(x, y)| Vec3::new(x as f32, y as f32, 0.0)).collect();

    // colors
    let white = Color::from_hex(0xFFFFFF);
    let yellow = Color::from_hex(0xffe100);
    
    //drawing
    framebuffer.draw_polygon(&poly1_v, white, yellow);

    //Render
    framebuffer.render("poligon1.bmp");

}