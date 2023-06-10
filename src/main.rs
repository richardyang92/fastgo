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
