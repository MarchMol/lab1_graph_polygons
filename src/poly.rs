use crate::{color::Color, framebuffer::Framebuffer, line::Line};
use nalgebra_glm::Vec3;

pub trait Polygon {
    fn draw_polygon(&mut self, arr: &[Vec3], line: Color, fill: Color);
}

impl Polygon for Framebuffer {
    fn draw_polygon(&mut self, arr: &[Vec3], line: Color, fill: Color) {
        if arr.len() >= 3 {
            // se pone el color de la linea
            self.set_current_color(Color::to_hex(&line));

            // se dibujan las lineas del polígono
            for (i, &v) in arr.iter().enumerate() {
                let next = if i == arr.len() - 1 {
                    arr[0]
                } else {
                    arr[i + 1]
                };
                self.line(v, next);
            }
            
            let mut min = (arr[0].x, arr[0].y);
            let mut max = (arr[0].x, arr[0].y);

            for &v in arr.iter() {
                if v.x < min.0 {
                    min.0 = v.x;
                }
                if v.x > max.0 {
                    max.0 = v.x;
                }
                if v.y < min.1 {
                    min.1 = v.y;
                }
                if v.y > max.1 {
                    max.1 = v.y;
                }
            }

            // Algoritmo de llenado
            
                self.set_current_color(Color::to_hex(&fill));
                let mut y = min.1.ceil() as i32;
                while y <= max.1.ceil() as i32 {
                    let mut nodes = vec![];

                    for (i, &v) in arr.iter().enumerate() {
                        let next = if i == arr.len() - 1 {
                            arr[0]
                        } else {
                            arr[i + 1]
                        };
                        if (v.y <= y as f32 && next.y > y as f32)
                            || (v.y > y as f32 && next.y <= y as f32)
                        {
                            let node_x = v.x + (y as f32 - v.y) * (next.x - v.x) / (next.y - v.y);
                            nodes.push(node_x);
                        }
                    }

                    nodes.sort_by(|a, b| a.partial_cmp(b).unwrap());

                    for n in (0..nodes.len()).step_by(2) {
                        if n + 1 < nodes.len() {
                            let start_x = (nodes[n].ceil()-1.0) as i32;
                            let end_x = (nodes[n + 1].ceil()) as i32;
                            for x in start_x..end_x {
                                let current_point = self.get_point(x, y);
                                
                                if current_point!= line.to_array(){
                                    self.point(x as usize, y as usize);
                                }
                            }
                        }
                    }
                    

                    y += 1;
                }
            
        }
    }
}
