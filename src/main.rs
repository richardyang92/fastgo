use config::Config;
use game_tree::{GameTree, SgfReader, ReadFile};
use go_band::{GoBand, Play};

use iced::mouse::Button;
use iced::{executor, Settings, Event, window, subscription, theme, application};
use iced::widget::{canvas, container, row, text};
use iced::{
    Application, Color, Command, Element, Length, Theme,
};

use crate::game_tree::Parse;

mod go_band;
mod go_move;
mod game_tree;
mod config;

macro_rules! GoBand {
    ($go_sz: expr, $settings: expr) => {
        match $go_sz {
            9 => GoBandView::<9>::run($settings),
            13 => GoBandView::<13>::run($settings),
            19 => GoBandView::<19>::run($settings),
            _ => Ok(())
        }
    };
}

fn main() -> iced::Result {
    let args: Vec<String> = std::env::args().collect();
    let config = if args.len() == 0 {
        Config::default()
    } else {
        Config::from(args)
    };
    println!("config={:?}", config);
    let go_sz = config.go_sz();

    let window_width = config.window_width();
    let window_height = config.window_height();
    let mut settings = Settings::with_flags(config);
    settings.window.size = (window_width, window_height);
    GoBand!(go_sz, settings)
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
}

struct GoBandView<const D: usize> {
    window_width: u32,
    window_height: u32,
    scale_factor: f32,
    go_band: GoBand<D>,
    game_tree: GameTree,
    move_count: i32,
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
    type Flags = Config;

    fn new(config: Config) -> (Self, Command<Message>) {
        let window_width = config.window_width();
        let window_height = config.window_height();
        let scale_factor = config.scale_factor();
        let go_band_width = (window_width as f32 * scale_factor) as u32;
        let go_sz = config.go_sz();
        let sgf_path = config.sgf_path();

        let game_tree = if let Ok(sgf_reader) = SgfReader::read_from(sgf_path) {
            let sgf_tokens = sgf_reader.parse();
            let game_tree = GameTree::from_sgf_tokens(&sgf_tokens, 0, sgf_tokens.len() - 1, true, true);
            match game_tree {
                Some(game_tree) => game_tree,
                None => GameTree::from(config)
            }
        } else {
            GameTree::from(config)
        };
        (
            GoBandView {
                window_width,
                window_height,
                scale_factor,
                go_band: GoBand::new(
                    go_band_width,
                    window_height,
                    0,
                    0,
                    go_sz as usize,
                ),
                game_tree,
                move_count: 0,
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
                                        let recorded_move = self.go_band.forward();
                                        match recorded_move {
                                            Some(go_move) => {
                                                self.move_count += 1;
                                                let move_id = go_move.move_id();
                                                let res_move = if self.move_count == move_id as i32 + 1 {
                                                    move_id as i32
                                                } else {
                                                    move_id as i32 - 1
                                                };
                                                GameTree::record_move(&mut self.game_tree, res_move, go_move);
                                                println!("game_tree={}", json::stringify(self.game_tree.to_json()))
                                            },
                                            None => {},
                                        }
                                    },
                                    Button::Right => {
                                        self.go_band.back();
                                        self.move_count -= 1;
                                    }
                                    _ => {},
                                }
                            }
                            _ => {}
                        };
                    } else {
                        if let Event::Window(window::Event::Resized { width, height }) = event {
                            self.go_band.set_window_width((width as f32 * self.scale_factor) as u32);
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
