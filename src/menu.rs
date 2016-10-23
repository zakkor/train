extern crate sfml;
use sfml::graphics::*;
use sfml::system::Vector2f;
use resource_manager::{FontManager, FontId};

pub enum ButtonType {
    Quit,
    Resume,
}

pub struct Button<'a> {
    pub button_type: ButtonType,
    pub text: Text<'a>,
    pub is_highlighted: bool,
}

impl<'a> Button<'a> {
    pub fn new(font: &'a Font, button_type: ButtonType, pos: &Vector2f) -> Self {
        let mut text = Text::new().unwrap();
        text.set_font(font);
        text.set_color(&Color::white());
        text.set_character_size(50);

        match button_type {
            ButtonType::Quit => {
                text.set_string("QUIT");
            }
            ButtonType::Resume => {
                text.set_string("RESUME");
            }
        }

        text.set_position(pos);

        Button {
            button_type: button_type,
            text: text,
            is_highlighted: false,
        }
    }
}

pub struct Menu<'a> {
    pub buttons: Vec<Button<'a>>,
}

impl<'a> Menu<'a> {
    pub fn init(&mut self, fm: &'a FontManager) {
        self.buttons.push(Button::new(fm.get(FontId::Arial),
                                           ButtonType::Resume,
                                           &Vector2f::new(150., 180.)));
        self.buttons.push(Button::new(fm.get(FontId::Arial),
                                           ButtonType::Quit,
                                           &Vector2f::new(150., 180. + 80.)));
    }
}
