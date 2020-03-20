extern crate opengl_graphics;

use std::collections::HashMap;

use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};

use crate::ship::*;

fn get_pallette(category: ShipCategory) -> [[f32; 4]; 2]{
    match category {
        ShipCategory::Bell => [[0.8, 0.4, 0.4, 1.0], [0.6, 0.2, 0.2, 1.0]],
        ShipCategory::Jalapeno => [[0.38, 0.49, 0.2, 1.0], [0.23, 0.39, 0.03, 1.0]],
        ShipCategory::Cayenne => [[0.65, 0.1, 0.1, 1.0], [0.6, 0.2, 0.2, 1.0]],
    }
}

pub struct ShipSprite {}

impl ShipSprite {
    pub fn draw(gl: &mut GlGraphics, args: &RenderArgs, ship: &ShipCache) {
        use crate::graphics::Transformed;

        let [_x, _y, _r, _d] = ship.render_piston();
        let [ship_color, wing_color] = get_pallette(ship.category);

        gl.draw(args.viewport(), |c, gl| {
            let body = [_x - _r, _y - _r, _r * 2.0, _r * 2.0];
            let wing = [[0.0, 1.0], [0.0, -0.4], [-1.5, -0.4]];
            let nozzle = [[0.0, 1.0], [0.6, -1.2], [-0.6, -1.2]];

            let transform = c
                .transform
                .trans(_x, _y)
                .rot_rad(-_d)
                .scale(_r, _r);

            graphics::polygon(wing_color, &nozzle, transform, gl);
            graphics::polygon(wing_color, &wing, transform, gl);
            graphics::polygon(wing_color, &wing, transform.flip_h(), gl);
            graphics::ellipse(ship_color, body, c.transform, gl);

            //graphics::rectangle([1.0, 1.0, 1.0, 0.4], ship.trajectory.render_piston(), c.transform, gl);
        });
    }
}

            /*
            println!("New loop, rotating {:.2} radians. \n{:?}", rotation, c.transform);
            println!("{:?}", c.transform.trans(x, y));
            println!("{:?}", c.transform.trans(x, y).rot_rad(rotation));
            println!("{:?}", c.transform.trans(x, y).rot_rad(rotation).trans(-25.0, -25.0));
             */
