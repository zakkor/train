use sfml::graphics::View;
use sfml::system::Vector2f;

pub struct Camera {
    pub game: View,
    pub ui: View,
    zoom_step: f32,
    pub mouse_pos_old: Vector2f,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            game: View::new().unwrap(),
            ui: View::new_init(&Vector2f::new(1600. / 2., 1200. / 2.), &Vector2f::new(1600., 1200.)).unwrap(),
            zoom_step: 0.1,
            mouse_pos_old: Vector2f::new(0., 0.),
        }
    }

    pub fn move_by_mouse(&mut self, mouse_pos: &Vector2f) {
        let move_factor = Vector2f::new(self.mouse_pos_old.x - mouse_pos.x,
                                        self.mouse_pos_old.y - mouse_pos.y);

        self.game.move_(&move_factor);
    }

    pub fn zoom(&mut self, delta: i32) {
        let zoom_factor = match delta < 0 {
            true => 1.0 + self.zoom_step,
            false => 1.0 - self.zoom_step,
        };

        self.game.zoom(zoom_factor);
    }
}
