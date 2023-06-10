use config::Config;
use go_band::{GoBand, Play};

use iced::mouse::Button;
use iced::{executor, Settings, Event, window, subscription, theme, application};
use iced::widget::{canvas, container, row, text};
use iced::{
    Application, Color, Command, Element, Length, Theme,
};

mod go_band;
mod go_move;
mod game_tree;
mod config;

static mut WINDOW_WIDTH: u32 = config::WINDOW_WIDTH;
static mut WINDOW_HEIGHT: u32 = config::WINDOW_HEIGHT;
static mut SCALE_FACTOR: f32 = config::SCALE_FACTOR;
static mut DIM: usize = config::GO_SZ as usize;

macro_rules! GoBand {
    ($go_sz: expr, $win_width: expr, $win_height: expr) => {
        GoBandView::<$go_sz>::run(Settings {
            antialiasing: true,
            window: window::Settings {
                size: ($win_width, $win_height),
                ..window::Settings::default()
            },
            ..Settings::default()
        })
    };
}

fn main() -> iced::Result {
    let args: Vec<String> = std::env::args().collect();
    let config = if args.len() == 0 {
        Config::default()
    } else {
        Config::from(args)
    };

    println!("conf={:?}", config);

    unsafe {
        WINDOW_WIDTH = config.window_width();
        WINDOW_HEIGHT = config.window_height();
        SCALE_FACTOR = config.scale_factor();
        DIM = config.go_sz() as usize;
    }

    match config.go_sz() {
        9 => GoBand!(9, config.window_width(), config.window_height()),
        13 => GoBand!(13, config.window_width(), config.window_height()),
        19 => GoBand!(19, config.window_width(), config.window_height()),
        _ => Ok(())
    }
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
        unsafe {
            let go_band_width = (WINDOW_WIDTH as f32 * SCALE_FACTOR) as u32;
            (
                GoBandView {
                    window_width: WINDOW_WIDTH,
                    window_height: WINDOW_HEIGHT,
                    scale_factor: SCALE_FACTOR,
                    go_band: GoBand::new(
                        go_band_width,
                        WINDOW_HEIGHT,
                        0,
                        0,
                        DIM,
                    ),
                },
                Command::none(),
            )
        }
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
                                        self.go_band.forward();
                                    },
                                    Button::Right => {
                                        self.go_band.back();
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
