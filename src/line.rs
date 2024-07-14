use crate::framebuffer::Framebuffer;

use nalgebra_glm::Vec3;

pub trait Line {
    fn line(&mut self, p1: Vec3, p2: Vec3);
}

impl Line for Framebuffer {
    fn line(&mut self, p1: Vec3, p2: Vec3){
        let x1 = p1.x as usize;
        let x2  = p2.x as usize;
        let y1  = p1.y as usize;
        let y2  = p2.y as usize;
        
        let mut x = (x1 as usize) as f32;
        let mut y = (y1 as usize) as f32;

        let dy =  y2 as f32 - y1 as f32;
        let dx = x2 as f32-x1 as f32;

        let mut valy: f32 = 1.0;
        let mut valx: f32 = 1.0;
        if x1 == x2{
            if dy<0.0{
                valy = -1.0
            }
            while (y!=y2 as f32) {
                self.point(x as usize, y as usize);
                y+=valy;
            }
        } else if y1 == y2{
            if dx<0.0{
                valx = -1.0
            }
            while x!=x2 as f32 {
                self.point(x as usize, y as usize);
                x+=valx;
            }
        } else{

            if dy <= 0.0{
                valy = -1.0;
            }
            if dx <= 0.0{
                valx = -1.0
            }
            let m = (y2 as f32 - y1 as f32)/(x2 as f32-x1 as f32);
            if m<=1.0 && -1.0< m{
                while x!=x2 as f32{
                    self.point(x as usize, y as usize);
                    x+=valx;
                    if (((x-x1 as f32)*m +y1 as f32-y -0.5)>= 0.0 && valy==1.0) || (((x-x1 as f32)*m +y1 as f32-y -0.5)<= 0.0 && valy==-1.0) {
                        y+=valy;
                    }
                }
            } else{
                while y!=y2 as f32{
                    self.point(x as usize, y as usize);
                    y+=valy;
                    if (((y-y1 as f32)/m +x1 as f32-x -0.5)>= 0.0 && valx==1.0) ||(((y-y1 as f32)/m +x1 as f32-x -0.5)<= 0.0 && valx==-1.0){
                        x+=valx;
                    }
                }
            }
        }
    }
}