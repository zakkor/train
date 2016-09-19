use sfml::graphics::*;
use resource_manager::*;

use game_consts::*;

pub struct World<'a> {
    pub bgs: Vec<Sprite<'a>>,
    pub rails: Vec<RectangleShape<'a>>,
}

impl<'a> World<'a> {
    pub fn init(&mut self, tm: &'a TextureManager) {
        let bg1 = Sprite::new_with_texture(&tm.get(TextureId::Background)).unwrap();

        let mut bg2 = bg1.clone();
        bg2.set_position2f(-(WINDOW_SIZE_X as f32), 0.);

        self.bgs.push(bg1);
        self.bgs.push(bg2);

        for x in 0..40 {
            let spacing = 60.;
            let mut new_rail = RectangleShape::new().unwrap();
            new_rail.set_fill_color(&Color::new_rgb(153, 76, 0));
            new_rail.set_position2f(x as f32 * spacing, 2. * 64.);
            new_rail.set_size2f(30., 3. * 64.);
            self.rails.push(new_rail);
        }

        // 2 metal bars for the actual rails
        let mut new_bar = RectangleShape::new().unwrap();
        new_bar.set_fill_color(&Color::new_rgb(192, 192, 192));
        new_bar.set_position2f(0., 2.25 * 64.);
        new_bar.set_size2f(1600. * 2., 20.);
        let mut new_bar2 = new_bar.clone();
        new_bar2.set_position2f(0., 4.40 * 64.);
        self.rails.push(new_bar);
        self.rails.push(new_bar2);

    }

    pub fn update(&mut self) {
        for bg in self.bgs.iter_mut() {
            if bg.get_position().x <= -(WINDOW_SIZE_X as f32) {
                bg.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
            }
        }

        // for rail in self.rails.iter_mut() {
        //     if rail.get_position().x <= -(WINDOW_SIZE_X as f32) {
        //         rail.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
        //     }
        // }
    }
}
