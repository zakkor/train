use sfml::graphics::*;
use sfml::system::*;
use std::collections::VecDeque;
use std::thread::JoinHandle;
use std::thread;
use actor::Actor;
use std::sync::mpsc::*;
use pathfinding::*;

pub struct ActorManager<'a> {
    pub actors: Vec<Actor<'a>>,
    pub selected: Vec<usize>,
    selection_rect: RectangleShape<'a>,
    is_selecting: bool,
    handles: Vec<(usize, JoinHandle<Option<VecDeque<(i32, i32)>>>)>,
    channel: (Sender<usize>, Receiver<usize>),
}

impl<'a> ActorManager<'a> {
    pub fn new() -> Self {
        let mut selection_rect = RectangleShape::new().unwrap();
        selection_rect.set_size2f(0., 0.);
        selection_rect.set_fill_color(&Color::new_rgba(0, 255, 0, 150));

        ActorManager {
            actors: vec![],
            selected: vec![],
            selection_rect: selection_rect,
            is_selecting: false,
            handles: vec![],
            channel: channel(),
        }
    }

    pub fn update_threads(&mut self, train_origin: Vector2f) {
        for recv in self.channel.1.try_iter() {
            let true_index = self.handles.iter().position(|ref x| x.0 == recv).unwrap();
            self.actors[self.selected[recv]].set_path(&mut self.handles.remove(true_index).1.join().unwrap().unwrap(),
                                                            train_origin);
        }
    }

    pub fn update_selection_rect(&mut self, mouse_x: i32, mouse_y: i32) {
        if self.is_selecting {
            let rect_pos = self.selection_rect.get_position();
            self.selection_rect.set_size2f(-(rect_pos.x - mouse_x as f32), -(rect_pos.y - mouse_y as f32));
        }
    }

    pub fn start_selection(&mut self, coords: &Vector2f) {
        self.is_selecting = true;
        self.selection_rect.set_size2f(0., 0.);
        self.selection_rect.set_position(coords);
    }

    pub fn launch_movement_orders(&mut self,
                                  pfgrid_in: PathfindingGrid,
                                  pfgrid_out: PathfindingGrid,
                                  train_pos: Vector2f,
                                  click_pos: Vector2f) {
        if !self.selected.is_empty() {
            for (idx, sa) in self.selected.iter().enumerate() {
                let actor = &mut self.actors[*sa];

                let pfgrid_to_use = if actor.inside_wagon {
                    pfgrid_in.clone()
                } else {
                    pfgrid_out.clone()
                };

                let start = actor.sprite.get_position();
                let send = self.channel.0.clone();
                self.handles.push((idx, thread::spawn(move || {
                    let path = compute_path(start,
                                            pfgrid_to_use,
                                            train_pos,
                                            click_pos);
                    send.send(idx);
                    path
                })));
            }
        }
    }

    pub fn apply_selection(&mut self) {
        if self.is_selecting {
            self.is_selecting = false;

            self.selected.clear();
            for (idx, a) in self.actors.iter_mut().enumerate() {
                if a.sprite
                    .get_global_bounds()
                    .intersects(&self.selection_rect.get_global_bounds()) != None {
                        if !self.selected.contains(&idx) {
                            a.sprite.set_color(&Color::green());
                            self.selected.push(idx);
                        }
                    }
                else if self.selected.get(idx) == None {
                    a.sprite.set_color(&Color::white());
                }
            }
        }
    }

    pub fn draw(&self, window: &mut RenderWindow) {
        for a in self.actors.iter() {
            // draw path
            let steps = Vec::from(a.move_seq.clone());
            for step in steps.windows(2) {
                let mut va = VertexArray::new().unwrap();
                va.set_primitive_type(PrimitiveType::sfLines);
                va.append(&Vertex::new_with_pos_color(&step[0], &Color::green()));
                va.append(&Vertex::new_with_pos_color(&step[1], &Color::green()));
                window.draw(&va);
            }

            // draw actor
            window.draw(&a.sprite);
        }

        if self.is_selecting {
            window.draw(&self.selection_rect);
        }
    }
}
