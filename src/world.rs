use sfml::graphics::*;
use resource_manager::*;

use game_consts::*;

pub struct World<'a> {
    pub bgs: Vec<Sprite<'a>>,
}

impl<'a> World<'a> {
    pub fn init(&mut self, tm: &'a TextureManager) {
        let bg1 = Sprite::new_with_texture(&tm.get(TextureId::Background)).unwrap();

        let mut bg2 = bg1.clone();
        bg2.set_position2f( -(WINDOW_SIZE_X as f32), 0.);

        self.bgs.push(bg1);
        self.bgs.push(bg2);
    }

    pub fn update(&mut self) {
        for bg in self.bgs.iter_mut() {
            if bg.get_position().x <= -(WINDOW_SIZE_X as f32) {
                bg.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
            }
        }
    }
}
