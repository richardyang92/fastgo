use go_band::{GoBand, Player};
use go_move::GoMove;
use iced::mouse::Button;
use iced::{executor, Settings, Event, window, subscription, theme, application};
use iced::widget::{canvas, container, row, text};
use iced::{
    Application, Color, Command, Element, Length, Theme,
};

mod go_band;
mod go_move;
mod game_tree;

const DIM: usize = 19;
pub const WINDOW_WIDTH: u32 = 1024;
pub const WINDOW_HEIGHT: u32 = 768;
pub const WIDTH_SCALE: f32 = 3.0 / 4.0;
pub const GO_KM: f32 = 7.5;
pub const GO_SZ: i32 = 19;

fn main() -> iced::Result {
    GoBandView::<DIM>::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
}

struct GoBandView<const D: usize> {
    window_width: u32,
    window_height: u32,
    go_band: GoBand<D>,
    go_moves: Vec<GoMove>,
}

impl<const D: usize> GoBandView<D> {
    fn clear_band_view(&self) {
        self.go_band.clear();
    }
}

impl<const D: usize> Application for GoBandView<D> {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let go_band_width = (WINDOW_WIDTH as f32 * WIDTH_SCALE) as u32;
        (
            GoBandView {
                window_width: WINDOW_WIDTH,
                window_height: WINDOW_HEIGHT,
                go_band: GoBand::new(
                    go_band_width,
                    WINDOW_HEIGHT,
                    0,
                    0,
                    DIM,
                ),
                go_moves: vec![],
            },
            Command::none(),
        )
    }

    fn style(&self) -> theme::Application {
        fn dark_background(_theme: &Theme) -> application::Appearance {
            application::Appearance {
                background_color: Color::BLACK,
                text_color: Color::WHITE,
            }
        }

        theme::Application::from(dark_background as fn(&Theme) -> _)
    }

    fn title(&self) -> String {
        String::from("FunGo - Iced")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::EventOccurred(event) => {
                if let Event::Window(window::Event::CloseRequested) = event {
                    window::close()
                } else {
                    if let Event::Mouse(mouse) = event {
                        match mouse {
                            iced::mouse::Event::CursorMoved { position } => {
                                self.go_band.get_preview_pos(position.x, position.y);
                                self.clear_band_view();
                            },
                            iced::mouse::Event::ButtonPressed(button) => {
                                match button {
                                    Button::Left => {
                                        let stone_pos = self.go_band.stone_pos();
                                        let mouse_preview = self.go_band.mouse_preview();
                                        let cur_x = stone_pos.0 as usize;
                                        let cur_y = stone_pos.1 as usize;

                                        if stone_pos == mouse_preview
                                            && self.go_band.stone_state(cur_x, cur_y) == 0 {
                                            let mut can_record = true;
                                            let mut eaten_stones_vec: Vec<(usize, usize, i8)> = vec![];
                                            let cur_player = self.go_band.current_player();
                                            let cur_state = match cur_player {
                                                Player::BLACK => {
                                                    self.go_band.set_stone_state(cur_x, cur_y, 1);
                                                    self.go_band.set_current_player(Player::WHITE);
                                                    1
                                                },
                                                Player::WHITE => {
                                                    self.go_band.set_stone_state(cur_x, cur_y, -1);
                                                    self.go_band.set_current_player(Player::BLACK);
                                                    -1
                                                },
                                            };
                                            match self.go_band.can_eat_stones(cur_state) {
                                                Some(mut eaten_stones_list) => {
                                                    if eaten_stones_list.len() == 0 {
                                                        // nothing to do
                                                    }
                                                    else if eaten_stones_list.len() == 1 {
                                                        let eaten_stones = eaten_stones_list.pop_front().unwrap();
                                                        if eaten_stones.1 == cur_state {
                                                            can_record = false;
                                                            match cur_state {
                                                                1 => {
                                                                    self.go_band.set_stone_state(cur_x, cur_y, 0);
                                                                    self.go_band.set_current_player(Player::BLACK);
                                                                },
                                                                -1 => {
                                                                    self.go_band.set_stone_state(cur_x, cur_y, 0);
                                                                    self.go_band.set_current_player(Player::WHITE);
                                                                },
                                                                _ => {}
                                                            }
                                                        } else {
                                                            for (i, j) in eaten_stones.0 {
                                                                self.go_band.set_stone_state(i, j, 0);
                                                                eaten_stones_vec.push((i, j, eaten_stones.1));
                                                            }
                                                        }
                                                    } else {
                                                        for eaten_stones in eaten_stones_list {
                                                            // println!("eaten stones={:?}, cur_pos=({}, {})", eaten_stones, cur_x, cur_y);
                                                            let mut can_eat = true;
                                                            let eaten_state = eaten_stones.1;
                                                            if eaten_state == cur_state {
                                                                // println!("check for robbery issue");
                                                                let go_record_len = self.go_moves.len();
                                                                match self.go_moves.get(go_record_len - 1) {
                                                                    Some(go_move) => {
                                                                        // println!("checked move: {:?}, cur move: ({}, {})", go_move, cur_x, cur_y);
                                                                        let record_eaten_stones = go_move.eaten_stones();
                                                                        if record_eaten_stones.len() == 1 && record_eaten_stones.len() == 1 {
                                                                            let record_eaten_stone = record_eaten_stones.get(0);
                                                                            if let Some((i, j, state)) = record_eaten_stone {
                                                                                if eaten_stones.0.contains(&(*i, *j)) && eaten_stones.1 == *state {
                                                                                    // println!("meet robbery issue");
                                                                                    can_record = false;
                                                                                    let (record_move_x, record_move_y, record_move_state) = go_move.move_pos();
                                                                                    self.go_band.set_stone_state(record_move_x, record_move_y, record_move_state);
                                                                                    match cur_state {
                                                                                        1 => {
                                                                                            self.go_band.set_stone_state(cur_x, cur_y, 0);
                                                                                            self.go_band.set_current_player(Player::BLACK);
                                                                                        },
                                                                                        -1 => {
                                                                                            self.go_band.set_stone_state(cur_x, cur_y, 0);
                                                                                            self.go_band.set_current_player(Player::WHITE);
                                                                                        },
                                                                                        _ => {}
                                                                                    }
                                                                                    break;
                                                                                }
                                                                            }
                                                                        }
                                                                    },
                                                                    None => {},
                                                                }
                                                            } else {
                                                                for (i, j) in eaten_stones.0.clone() {
                                                                    // println!("({}, {}), cur_pos=({}, {})", i, j, cur_x, cur_y);
                                                                    if i == cur_x && j == cur_y {
                                                                        can_eat = false;
                                                                        can_record = false;
                                                                        let cur_player = self.go_band.current_player();
                                                                        match cur_player {
                                                                            Player::BLACK => {
                                                                                self.go_band.set_stone_state(cur_x, cur_y, 0);
                                                                                self.go_band.set_current_player(Player::WHITE);
                                                                            },
                                                                            Player::WHITE => {
                                                                                self.go_band.set_stone_state(cur_x, cur_y, 0);
                                                                                self.go_band.set_current_player(Player::BLACK);
                                                                            },
                                                                        }
                                                                        break;
                                                                    }
                                                                }
                                                            }
                                                            if can_eat {
                                                                for (i, j) in eaten_stones.0 {
                                                                    if cur_state == eaten_state {
                                                                        continue;
                                                                    }
                                                                    self.go_band.set_stone_state(i, j, 0);
                                                                    eaten_stones_vec.push((i, j, eaten_state));
                                                                }
                                                            }
                                                        }
                                                    }
                                                },
                                                None => {},
                                            }
                                            if can_record {
                                                let record_len = self.go_moves.len();
                                                let move_id = record_len;
                                                let mut go_move = GoMove::new(move_id, cur_x, cur_y, cur_state);
                                                go_move.set_eaten_stones(eaten_stones_vec);
                                                self.go_moves.push(go_move);
                                                println!("{}: {:?}", move_id, self.go_moves.last());
                                            }
                                            self.clear_band_view();
                                        }
                                    },
                                    Button::Right => {
                                        match self.go_moves.pop() {
                                            Some(go_move) => {
                                                println!("recored move: {:?}", go_move.move_id());
                                                let (pos_x, pos_y, record_state) = go_move.move_pos();
                                                self.go_band.set_stone_state(pos_x, pos_y, 0);
                                                match record_state {
                                                    1 => self.go_band.set_current_player(Player::BLACK),
                                                    -1 => self.go_band.set_current_player(Player::WHITE),
                                                    _ => {},
                                                }
                                                let eaten_stones = go_move.eaten_stones();
                                                for (i, j, state) in eaten_stones {
                                                    self.go_band.set_stone_state(i, j, state);
                                                }
                                                self.clear_band_view();
                                            },
                                            None => {},
                                        };
                                    }
                                    _ => {},
                                }
                            }
                            _ => {}
                        };
                    } else {
                        if let Event::Window(window::Event::Resized { width, height }) = event {
                            self.go_band.set_window_width((width as f32 * WIDTH_SCALE) as u32);
                            self.go_band.set_window_height(height);
                            self.window_width = width;
                            self.window_height = height;
                        }
                    }
                    Command::none()
                }
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscription::events().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Message> {
        let go_band_width = self.go_band.window_width();
        let canvas = canvas(&self.go_band)
            .width(Length::Fixed(go_band_width as f32))
            .height(Length::Fill);
        container(
            row![
                container(canvas)
                .width(Length::Fixed(go_band_width as f32))
                .height(Length::Fill),
                container(text("Sgf"))
                .width(Length::Fixed((self.window_width - go_band_width) as f32 / 2.0))
                .height(Length::Fill)
            ])
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

}
