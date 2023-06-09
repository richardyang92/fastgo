use std::collections::{LinkedList, HashSet};

use iced::{widget::canvas::{self, Stroke, stroke, LineCap, Path, Cache}, Renderer, Theme, Point, Size, Color};

#[derive(Debug, Clone, Copy)]
pub enum Player {
    BLACK, WHITE,
}

pub struct GoBand<const D: usize> {
    window_width: u32,
    window_height: u32,
    margin_x: u32,
    margin_y: u32,
    dim: usize,
    stone_state: [[i8; D]; D],
    stone_block: Vec<(i32, i32, i8)>,
    stone_pos: (i32, i32),
    mouse_preview: (i32, i32),
    cur_player: Player,
    band_cache: Cache,
}

impl<const D: usize> GoBand<D> {
    pub fn new(
        window_width: u32,
        window_height: u32,
        margin_x: u32,
        margin_y: u32,
        dim: usize) -> Self {
        GoBand {
            window_width,
            window_height,
            margin_x,
            margin_y,
            dim,
            stone_state: [[0; D]; D],
            stone_block: vec![],
            mouse_preview: (D as i32 / 2, D as i32 / 2),
            stone_pos: (D as i32 / 2, D as i32 / 2),
            cur_player: Player::BLACK,
            band_cache: Cache::default(),
        }
    }

    pub fn get_preview_pos(&mut self, pos_x: f32, pos_y: f32) {
        let grid_size = self.window_width.min(self.window_height) as f32 / D as f32;
        let (align_x, align_y) = if self.window_width > self.window_height {
            let align_y = grid_size / 2.0;
            let align_x = self.window_width.abs_diff(self.window_height) as f32 / 2.0 + align_y;
            (align_x, align_y)
        } else {
            let align_x = grid_size / 2.0;
            let align_y = self.window_width.abs_diff(self.window_height) as f32 / 2.0 + align_x;
            (align_x, align_y)
        };
        let frame_position = (pos_x - align_x - self.margin_x as f32, pos_y - align_y - self.margin_y as f32);
        let mut x_grid = (frame_position.0 / grid_size) as i32;
        let mut y_grid = (frame_position.1 / grid_size) as i32;

        let x_res = frame_position.0 - grid_size * x_grid as f32;
        let y_res = frame_position.1 - grid_size * y_grid as f32;

        if x_res > align_y {
            x_grid += 1;
        }
        if y_res > align_y {
            y_grid += 1;
        }

        self.stone_pos = (x_grid, y_grid);

        if x_grid < 0 {
            x_grid = 0;
        }
        if x_grid > D as i32 - 1 {
            x_grid = D as i32 - 1;
        }
        if y_grid < 0 {
            y_grid = 0;
        }
        if y_grid > D as i32 - 1 {
            y_grid = D as i32 - 1;
        }

        self.mouse_preview = (x_grid, y_grid);
    }

    pub fn stone_pos(&self) -> (i32, i32) {
        self.stone_pos
    }

    pub fn mouse_preview(&self) -> (i32, i32) {
        self.mouse_preview
    }

    pub fn current_player(&self) -> Player {
        self.cur_player
    }

    pub fn set_current_player(&mut self, player: Player) {
        self.cur_player = player;
    }

    pub fn stone_state(&self, pos_x: usize, pos_y: usize) -> i8 {
        self.stone_state[pos_x][pos_y]
    }

    pub fn set_stone_state(&mut self, pos_x: usize, pos_y: usize, state: i8) {
        self.stone_state[pos_x][pos_y] = state;
    }

    pub fn set_window_width(&mut self, window_width: u32) {
        self.window_width = window_width;
    }

    pub fn window_width(&self) -> u32 {
        self.window_width
    }

    pub fn set_window_height(&mut self, window_height: u32) {
        self.window_height = window_height;
    }

    pub(crate) fn clear(&self) {
        self.band_cache.clear();
    }
}

impl<const D: usize> GoBand<D> {
    pub fn can_eat_stones(&mut self, stone_state: i8) -> Option<LinkedList<(HashSet<(usize, usize)>, i8)>> {
        let mut eaten_stones_list: LinkedList<(HashSet<(usize, usize)>, i8)> = LinkedList::new();
        for i in 0..D {
            for j in 0..D {
                if self.stone_state[i][j] == 0 {
                    continue;
                } else {
                    self.stone_block.clear();
                    self.stone_block.push((i as i32, j as i32, self.stone_state[i][j]));
                    self.check_stone_block(i as i32, j as i32, self.stone_state[i][j]);
                    if self.is_alive() {
                        continue;
                    } else {
                        let mut eaten_stones: HashSet<(usize, usize)> = HashSet::new();
                        let mut cur_state = stone_state;
                        for (i, j, stone_state) in &mut self.stone_block {
                            eaten_stones.insert((*i as usize, *j as usize));
                            cur_state = *stone_state;
                        }
                        if eaten_stones.len() > 0 {
                            let mut redundant = false;
                            for set in eaten_stones_list.clone() {
                                if set.0.eq(&eaten_stones) && set.1 == cur_state {
                                    redundant = true;
                                    break;
                                }
                            }
                            if !redundant {
                                eaten_stones_list.push_front((eaten_stones, cur_state));   
                            }
                            continue;
                        }
                    }
                }
            }
        }
        if eaten_stones_list.len() > 0 {
            Some(eaten_stones_list)
        } else {
            None
        }
    }

    fn check_stone_block(&mut self, pos_x: i32, pos_y: i32, stone_state: i8) {
        if self.can_connect(pos_x - 1, pos_y, stone_state) {
            self.stone_block.push((pos_x - 1, pos_y, stone_state));
            self.check_stone_block(pos_x - 1, pos_y, stone_state);
        }
        if self.can_connect(pos_x + 1, pos_y, stone_state) {
            self.stone_block.push((pos_x + 1, pos_y, stone_state));
            self.check_stone_block(pos_x + 1, pos_y, stone_state);
        }
        if self.can_connect(pos_x, pos_y - 1, stone_state) {
            self.stone_block.push((pos_x, pos_y - 1, stone_state));
            self.check_stone_block(pos_x, pos_y - 1, stone_state);
        }
        if self.can_connect(pos_x, pos_y + 1, stone_state) {
            self.stone_block.push((pos_x, pos_y + 1, stone_state));
            self.check_stone_block(pos_x, pos_y + 1, stone_state);
        }
    }

    fn can_connect(&mut self, pos_x: i32, pos_y: i32, stone_state: i8) -> bool {
        let ret = pos_x >= 0 && pos_x <= D as i32 - 1 && pos_y >= 0 && pos_y <= D as i32 - 1
            && self.stone_state[pos_x as usize][pos_y as usize] == stone_state
            && !self.is_in_block(pos_x, pos_y);
        ret
    }

    fn is_in_block(&mut self, pos_x: i32, pos_y: i32) -> bool {
        for (i, j, _) in &mut self.stone_block {
            if *i == pos_x && *j == pos_y {
                return true;
            }
        }
        false
    }

    fn is_alive(&mut self) -> bool {
        for (i, j, _) in &mut self.stone_block {
            let pos_x = *i;
            let pos_y = *j;

            if pos_x - 1 >= 0
                && self.stone_state[pos_x as usize - 1][pos_y as usize] == 0 { return true; }
            if pos_x + 1 < D as i32
                && self.stone_state[pos_x as usize + 1][pos_y as usize] == 0 { return true; }
            if pos_y - 1 >= 0
                && self.stone_state[pos_x as usize][pos_y as usize - 1] == 0 { return true; }
            if pos_y + 1 < D as i32
                && self.stone_state[pos_x as usize][pos_y as usize + 1] == 0 { return true; }
        }
        false
    }
}

impl<Message, const D: usize> canvas::Program<Message, Renderer> for GoBand<D> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        let go_band = self.band_cache.draw(renderer, bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;
            let top_left = Point::new(center.x - radius, center.y - radius);
            let background = Path::rectangle(top_left, Size::new(radius * 2.0, radius * 2.0));
            frame.fill(&background, Color::from_rgb8(250, 189, 132));

            let grid_size = (radius * 2.0) / D as f32;
            let thin_stroke = || -> Stroke {
                Stroke {
                    width: 1.0,
                    style: stroke::Style::Solid(Color::BLACK),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };
            let wide_stroke = || -> Stroke {
                Stroke {
                    width: 3.0,
                    style: stroke::Style::Solid(Color::BLACK),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };
            let top_left = Point::new(center.x - radius + grid_size / 2.0, center.y - radius + grid_size / 2.0);
            for _i in 0..D {
                let row = Path::line(Point::new(top_left.x, top_left.y + _i as f32 * grid_size), Point::new(top_left.x + radius * 2.0 - grid_size, top_left.y + _i as  f32 * grid_size));
                let col = Path::line(Point::new(top_left.x + _i as f32 * grid_size, top_left.y), Point::new(top_left.x + _i as f32 * grid_size, top_left.y + radius * 2.0 - grid_size));
                frame.with_save(|frame| {
                    let stroke = if _i == 0 || _i == D - 1 {
                        wide_stroke
                    } else {
                        thin_stroke
                    };
                    frame.stroke(&row, stroke());
                    frame.stroke(&col, stroke());
                })
            }
            let star_pos_arr = match self.dim {
                9 => vec![(2, 2), (6, 2), (2, 6), (6, 6)],
                13 => vec![(3, 3), (3, 9), (6, 6), (9, 3), (9, 9)],
                19 => vec![
                    (3, 3), (3, 9), (3, 15),
                    (9, 3), (9, 9), (9, 15),
                    (15, 3), (15, 9), (15, 15),
                ],
                _ => vec![],
            };

            for (x, y) in star_pos_arr {
                let star_pos = Path::circle(Point::new(top_left.x + x as f32 * grid_size, top_left.y + y as f32 * grid_size), 3.0);
                frame.fill(&star_pos, Color::BLACK);
            }

            let mouse_preview = self.mouse_preview();
            let mouse_preview = Path::rectangle(Point::new(top_left.x + mouse_preview.0 as f32 * grid_size - 10.0, top_left.y + mouse_preview.1 as f32 * grid_size - 10.0), Size::new(20.0, 20.0));
            let cur_player = self.current_player();
            let color = if let Player::BLACK = cur_player {
                Color::BLACK
            } else {
                Color::WHITE
            };
            frame.fill(&mouse_preview, color);

            for x in 0..D as usize {
                for y in 0..D as usize {
                    let band_state = self.stone_state(x, y);
                    if band_state != 0 {
                        let cur_pos = Path::circle(Point::new(top_left.x + x as f32 * grid_size, top_left.y + y as f32 * grid_size), grid_size / 2.0);
                        let color = if band_state == 1 { Color::BLACK } else { Color::WHITE };
                        frame.fill(&cur_pos, color);
                    }
                }
            }
        });

        vec![go_band]
    }
}