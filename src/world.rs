use sfml::graphics::*;
use sfml::system::Vector2f;
use resource_manager::*;
use lyon_bezier::*;

use game_consts::*;

pub struct World<'a> {
    pub bgs: Vec<Sprite<'a>>,
    pub rails: Vec<RectangleShape<'a>>,
    pub connectors: Vec<RectangleShape<'a>>,
    pub curves: Vec<(QuadraticBezierSegment, QuadraticBezierSegment)>,
}

fn new_curve(from: Vec2, ctrl: Vec2, to: Vec2) -> QuadraticBezierSegment {
    QuadraticBezierSegment {
        from: from,
        ctrl: ctrl,
        to: to,
    }
}

fn populate_vec_with_shapes_from_curve(curve: &mut QuadraticBezierSegment,
                                       vec: &mut Vec<RectangleShape>)
{
    let mut previous_seg = curve.from;
    for (idx, seg) in curve.flattening_iter(0.01).enumerate() {
        let mut rect_seg = RectangleShape::new().unwrap();
        rect_seg.set_position2f(previous_seg.x, previous_seg.y);
        rect_seg.set_fill_color(&Color::new_rgb(192, 192, 192));

        let distance = ( (seg.x - previous_seg.x).powi(2) + (seg.y - previous_seg.y).powi(2) ).sqrt();
        let angle = ( seg.y - previous_seg.y ).atan2( seg.x - previous_seg.x ).to_degrees();
        rect_seg.set_size2f(distance, 20.);
        rect_seg.set_rotation(angle);

        vec.push(rect_seg);

        previous_seg = seg;
    }
}

impl<'a> World<'a> {
    pub fn init(&mut self, tm: &'a TextureManager) {
        self.curves.push(
            (new_curve(Vec2::new(0., 300.),
                       Vec2::new(800., 600.),
                       Vec2::new(1600., 300.)),
             new_curve(Vec2::new(0., 500.),
                       Vec2::new(800., 800.),
                       Vec2::new(1600., 500.))));

        self.curves.push(
            (new_curve(Vec2::new(1600., 300.),
                       Vec2::new(2400., 0.),
                       Vec2::new(3200., 300.)),
             new_curve(Vec2::new(1600., 500.),
                       Vec2::new(2400., 200.),
                       Vec2::new(3200., 500.))));




        for pair in self.curves.iter_mut() {
            populate_vec_with_shapes_from_curve(&mut pair.0, &mut self.rails);
            populate_vec_with_shapes_from_curve(&mut pair.1, &mut self.rails);

            let mut previous_seg = pair.0.from;
            let mut previous_seg_bot = pair.1.from;
            for (seg, seg_bot) in pair.0.flattening_iter(0.1).zip( // TODO: fix this
                pair.1.flattening_iter(0.1)) {
                let p1 = Vector2f::new((previous_seg.x + seg.x) / 2., (previous_seg.y + seg.y) / 2.);
                let p2 = Vector2f::new((previous_seg_bot.x + seg_bot.x) / 2., (previous_seg_bot.y + seg_bot.y) / 2.);

                let perp_angle = (previous_seg.y - seg.y).atan2(previous_seg.x - seg.x).to_degrees() - 90.;
                let perp_distance = ( (p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2) ).sqrt();

                let mut rect_perp = RectangleShape::new().unwrap();
                rect_perp.set_position(&p1);
                rect_perp.set_size2f(perp_distance + 20., 20.);
                rect_perp.set_rotation(perp_angle);
                rect_perp.set_fill_color(&Color::new_rgb(130, 82, 1));

                self.connectors.push(rect_perp);

                previous_seg = seg;
                previous_seg_bot = seg_bot;
            }
        }

        for x in 0..3 {
            for y in 0..3 {
                let mut new_bg = Sprite::new_with_texture(&tm.get(TextureId::Background)).unwrap();
                new_bg.set_position2f((x * WINDOW_SIZE_X as i32) as f32,
                                      (y * WINDOW_SIZE_Y as i32) as f32);
                self.bgs.push(new_bg);
            }
        }
    }

    pub fn update(&mut self, speed: f32) {
        for bg in self.bgs.iter_mut() {
            if bg.get_position().x <= -(WINDOW_SIZE_X as f32) {
                bg.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
            }
        }

        for bg in self.bgs.iter_mut() {
            bg.move2f(speed, 0.);
        }

        for rail in self.rails.iter_mut() {
            if rail.get_position().x <= -(WINDOW_SIZE_X as f32) {
                rail.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
            }
            rail.move2f(speed, 0.);
        }

        for ctr in self.connectors.iter_mut() {
            if ctr.get_position().x <= -(WINDOW_SIZE_X as f32) {
                ctr.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
            }
            ctr.move2f(speed, 0.);
        }
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
