use crate::shape::*;

const RATIO_X: f64 = 0.5;
const RATIO_Y: f64 = 0.5;
const VIEWPORT_X: f64 = (1.0 - RATIO_X) / 2.0;
const VIEWPORT_Y: f64 = (1.0 - RATIO_Y) / 2.0;

#[derive(Clone,Debug)]
pub struct Camera {
    offset: Point,
    screen: Rectangle,
    viewport: Rectangle,
    scale: f64,
    follow: (bool, bool),
}

impl Camera {
    pub fn new(width: f64, height: f64, scale: f64, follow: (bool, bool)) -> Camera {
        let mut camera = Camera {
            offset: Point::new(0.0, 0.0),
            screen: Rectangle::new(0.0, 0.0, width, height),
            viewport: Rectangle::new(0.0, 0.0, 0.0, 0.0),
            scale: 1.0,
            follow: follow,
        };

        camera.adjust_viewport();

        camera
    }

    pub fn get_x(&self) -> f64 {
        self.offset.x
    }

    pub fn get_y(&self) -> f64 {
        self.offset.y
    }
    
    pub fn get_offset(&self) -> Point {
        self.offset
    }
    
    pub fn get_status(&self) -> (bool, bool) {
        self.follow
    }

    pub fn follow(&mut self, x: f64, y: f64) {
        if self.viewport.check_collision_point(&Point::new(x, y)) {
            return ();
        }

        if self.follow.0 {
            self.follow_x(x);
        }

        if self.follow.1 {
            self.follow_y(y);
        }

        self.adjust_viewport();
    }

    pub fn follow_x(&mut self, x: f64) {
        if self.viewport.left() > x || self.viewport.right() < x {
            let dx = f64::min((self.viewport.left() - x).abs(), (x - self.viewport.right()).abs());
            let vx = f64::powf(dx / (VIEWPORT_X * self.screen.width), 2.0) * VIEWPORT_X * self.screen.width;

            self.offset.x += vx * f64::signum(x - self.viewport.left());
        }
    }

    pub fn follow_y(&mut self, y: f64) {
        if self.viewport.top() > y || self.viewport.bottom() < y {
            let dy = f64::min((self.viewport.top() - y).abs(), (y - self.viewport.bottom()).abs());
            let vy = f64::powf(dy / (VIEWPORT_Y * self.screen.height), 2.0) * VIEWPORT_Y * self.screen.height;
    
            self.offset.y += vy * f64::signum(y - self.viewport.top());
        }
    }

    pub fn update_screen(&mut self, width: f64, height: f64, scale: f64) {
        self.screen = Rectangle::new(self.offset.x, self.offset.y, width, height);
        self.scale = scale;

        self.adjust_viewport();
    }

    pub fn adjust_viewport(&mut self) {
        self.viewport = Rectangle::new(
            self.offset.x + (1.0 - RATIO_X) / 2.0 * self.screen.width,
            self.offset.y + (1.0 - RATIO_Y) / 2.0 * self.screen.height,
            self.screen.width * RATIO_X,
            self.screen.height * RATIO_Y
        );
    }
}