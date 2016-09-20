use sfml::graphics::*;
use sfml::system::Vector2f;
use resource_manager::*;

use game_consts::*;

pub struct World<'a> {
    pub bgs: Vec<Sprite<'a>>,
    pub rails: Vec<RectangleShape<'a>>,
}

impl<'a> World<'a> {
    pub fn init(&mut self, tm: &'a TextureManager) {
        for x in 0..3 {
            for y in 0..3 {
                let mut new_bg = Sprite::new_with_texture(&tm.get(TextureId::Background)).unwrap();
                new_bg.set_position2f((x * WINDOW_SIZE_X as i32) as f32,
                                      (y * WINDOW_SIZE_Y as i32) as f32);
                self.bgs.push(new_bg);
            }
        }

        let spacing = 44.; // magic

        for x in 0..40 {
            let mut new_rail = RectangleShape::new().unwrap();
            new_rail.set_fill_color(&Color::new_rgb(153, 76, 0));
            new_rail.set_position2f(x as f32 * spacing, 2. * 64.);
            new_rail.set_size2f(20., 3. * 64.); // magic
            self.rails.push(new_rail);
        }

        // 2 metal bars for the actual rails
        let mut new_bar = RectangleShape::new().unwrap();
        new_bar.set_fill_color(&Color::new_rgb(192, 192, 192));
        new_bar.set_position2f(0., 2.25 * 64.);
        new_bar.set_size2f(1600. * 2., 20.); // magic
        let mut new_bar2 = new_bar.clone();
        new_bar2.set_position2f(0., 4.40 * 64.);
        self.rails.push(new_bar);
        self.rails.push(new_bar2);
    }

    pub fn update(&mut self, speed: f32) {
        for bg in self.bgs.iter_mut() {
            if bg.get_position().x <= -(WINDOW_SIZE_X as f32) {
                bg.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
            }
        }

        for rail in self.rails.iter_mut() {
            if rail.get_position().x <= -(WINDOW_SIZE_X as f32) {
                rail.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
            }
        }

        for bg in self.bgs.iter_mut() {
            bg.move2f(speed, 0.);
        }

        // for rail in self.rails.iter_mut() {
        //     rail.move2f(speed, 0.);
        // }
    }

    // pub fn recalculate_drawables(&mut self, view: &View, origin_pixels: &Vector2f, tm: &'a TextureManager) {
    //     println!("{:?}", origin_pixels);
    //     self.bgs.clear();
    //     for x in 0..(view.get_size().x / WINDOW_SIZE_X as f32) as i32 {
    //         for y in 0..(view.get_size().y / WINDOW_SIZE_Y as f32) as i32 {
    //             let mut new_bg = Sprite::new().unwrap();
    //             new_bg.set_texture(tm.get(TextureId::Background), true);
    //             new_bg.set_position2f(origin_pixels.x + (x * WINDOW_SIZE_X as i32) as f32,
    //                                   origin_pixels.y + (y * WINDOW_SIZE_Y as i32) as f32);
    //             self.bgs.push(new_bg);
    //             print!("[]");
    //         }
    //         println!("");
    //     }
    // }
}
