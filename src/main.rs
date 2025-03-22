use iced::application::StyleSheet;
use iced::font::Weight;
use iced::theme::{self, Container as ThemeContainer, Text as TextTheme};
use iced::widget::{column, container, row, scrollable, text, Container, Text};
use iced::Color;
use iced::Font;
use iced::{Element, Length, Sandbox, Settings, Theme};

mod colors;
mod my_text;
use colors::*;
use my_text::*;

// Define the custom font
const CUSTOM_FONT: Font = Font {
    family: iced::font::Family::SansSerif,
    weight: Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    monospaced: false,
};

// Player statistics structure
#[derive(Debug, Clone)]
struct Player {
    name: String,
    winrate: f32,
    battles: u32,
    ship_name: String,
    ship_winrate: f32,
    ship_battles: u32,
    pr: u32,
    avg_damage: f32,
    frags: f32,
}

// Main application state
struct StatsViewer {
    team1: Vec<Player>,
    team2: Vec<Player>,
}

#[derive(Debug, Clone)]
enum Message {
    // Add messages here when we need interaction
}

impl Sandbox for StatsViewer {
    type Message = Message;

    fn new() -> Self {
        // Initialize with sample data
        let team1 = vec![
            Player {
                name: "Alpha".to_string(),
                winrate: 49.96,
                battles: 2754,
                ship_name: "Ship1".to_string(),
                ship_winrate: 48.5,
                ship_battles: 156,
                pr: 856,
                avg_damage: 84849.0,
                frags: 0.8,
            },
            Player {
                name: "Beta".to_string(),
                winrate: 47.22,
                battles: 4852,
                ship_name: "Ship2".to_string(),
                ship_winrate: 51.2,
                ship_battles: 342,
                pr: 1425,
                avg_damage: 132932.0,
                frags: 1.2,
            },
            Player {
                name: "Charlie".to_string(),
                winrate: 43.45,
                battles: 1991,
                ship_name: "Ship3".to_string(),
                ship_winrate: 46.8,
                ship_battles: 89,
                pr: 485,
                avg_damage: 57493.0,
                frags: 0.6,
            },
            Player {
                name: "Delta".to_string(),
                winrate: 45.02,
                battles: 844,
                ship_name: "Ship4".to_string(),
                ship_winrate: 44.9,
                ship_battles: 234,
                pr: 892,
                avg_damage: 71441.0,
                frags: 0.7,
            },
            Player {
                name: "Echo".to_string(),
                winrate: 48.53,
                battles: 5110,
                ship_name: "Ship5".to_string(),
                ship_winrate: 52.1,
                ship_battles: 445,
                pr: 447,
                avg_damage: 45591.0,
                frags: 0.9,
            },
            Player {
                name: "Foxtrot".to_string(),
                winrate: 47.3,
                battles: 9349,
                ship_name: "Ship6".to_string(),
                ship_winrate: 49.9,
                ship_battles: 678,
                pr: 1248,
                avg_damage: 51313.0,
                frags: 1.1,
            },
            Player {
                name: "Mike".to_string(),
                winrate: 51.23,
                battles: 3245,
                ship_name: "Ship13".to_string(),
                ship_winrate: 52.8,
                ship_battles: 234,
                pr: 1256,
                avg_damage: 98765.0,
                frags: 1.3,
            },
            Player {
                name: "November".to_string(),
                winrate: 48.76,
                battles: 4123,
                ship_name: "Ship14".to_string(),
                ship_winrate: 47.5,
                ship_battles: 345,
                pr: 892,
                avg_damage: 65432.0,
                frags: 0.8,
            },
            Player {
                name: "Oscar".to_string(),
                winrate: 52.34,
                battles: 2876,
                ship_name: "Ship15".to_string(),
                ship_winrate: 53.2,
                ship_battles: 456,
                pr: 1456,
                avg_damage: 112345.0,
                frags: 1.4,
            },
            Player {
                name: "Papa".to_string(),
                winrate: 46.78,
                battles: 5678,
                ship_name: "Ship16".to_string(),
                ship_winrate: 45.9,
                ship_battles: 567,
                pr: 678,
                avg_damage: 45678.0,
                frags: 0.7,
            },
            Player {
                name: "Quebec".to_string(),
                winrate: 50.12,
                battles: 3456,
                ship_name: "Ship17".to_string(),
                ship_winrate: 51.5,
                ship_battles: 678,
                pr: 1234,
                avg_damage: 87654.0,
                frags: 1.0,
            },
            Player {
                name: "Romeo".to_string(),
                winrate: 49.87,
                battles: 4321,
                ship_name: "Ship18".to_string(),
                ship_winrate: 48.7,
                ship_battles: 789,
                pr: 987,
                avg_damage: 76543.0,
                frags: 0.9,
            },
        ];

        let team2 = vec![
            Player {
                name: "Golf".to_string(),
                winrate: 49.92,
                battles: 2644,
                ship_name: "Ship7".to_string(),
                ship_winrate: 53.4,
                ship_battles: 223,
                pr: 1350,
                avg_damage: 103170.0,
                frags: 1.2,
            },
            Player {
                name: "Hotel".to_string(),
                winrate: 49.49,
                battles: 2623,
                ship_name: "Ship8".to_string(),
                ship_winrate: 47.8,
                ship_battles: 167,
                pr: 1121,
                avg_damage: 139917.0,
                frags: 1.1,
            },
            Player {
                name: "India".to_string(),
                winrate: 49.74,
                battles: 2280,
                ship_name: "Ship9".to_string(),
                ship_winrate: 50.2,
                ship_battles: 445,
                pr: 1236,
                avg_damage: 105548.0,
                frags: 1.0,
            },
            Player {
                name: "Juliet".to_string(),
                winrate: 47.21,
                battles: 2923,
                ship_name: "Ship10".to_string(),
                ship_winrate: 46.9,
                ship_battles: 332,
                pr: 892,
                avg_damage: 92047.0,
                frags: 0.8,
            },
            Player {
                name: "Kilo".to_string(),
                winrate: 53.43,
                battles: 3509,
                ship_name: "Ship11".to_string(),
                ship_winrate: 55.6,
                ship_battles: 221,
                pr: 962,
                avg_damage: 66757.0,
                frags: 1.3,
            },
            Player {
                name: "Lima".to_string(),
                winrate: 51.96,
                battles: 4419,
                ship_name: "Ship12".to_string(),
                ship_winrate: 50.8,
                ship_battles: 554,
                pr: 1009,
                avg_damage: 119177.0,
                frags: 1.4,
            },
            Player {
                name: "Sierra".to_string(),
                winrate: 48.45,
                battles: 3789,
                ship_name: "Ship19".to_string(),
                ship_winrate: 49.2,
                ship_battles: 456,
                pr: 876,
                avg_damage: 67890.0,
                frags: 0.9,
            },
            Player {
                name: "Tango".to_string(),
                winrate: 52.67,
                battles: 2987,
                ship_name: "Ship20".to_string(),
                ship_winrate: 54.1,
                ship_battles: 567,
                pr: 1345,
                avg_damage: 98765.0,
                frags: 1.5,
            },
            Player {
                name: "Uniform".to_string(),
                winrate: 47.89,
                battles: 4567,
                ship_name: "Ship21".to_string(),
                ship_winrate: 46.8,
                ship_battles: 678,
                pr: 765,
                avg_damage: 54321.0,
                frags: 0.7,
            },
            Player {
                name: "Victor".to_string(),
                winrate: 50.34,
                battles: 3456,
                ship_name: "Ship22".to_string(),
                ship_winrate: 51.7,
                ship_battles: 789,
                pr: 1123,
                avg_damage: 87654.0,
                frags: 1.1,
            },
            Player {
                name: "Whiskey".to_string(),
                winrate: 49.56,
                battles: 4321,
                ship_name: "Ship23".to_string(),
                ship_winrate: 48.9,
                ship_battles: 890,
                pr: 987,
                avg_damage: 76543.0,
                frags: 0.8,
            },
            Player {
                name: "Xray".to_string(),
                winrate: 51.78,
                battles: 2987,
                ship_name: "Ship24".to_string(),
                ship_winrate: 52.5,
                ship_battles: 567,
                pr: 1234,
                avg_damage: 98765.0,
                frags: 1.2,
            },
        ];

        StatsViewer { team1, team2 }
    }

    fn title(&self) -> String {
        String::from("Okay You Very Pro")
    }

    fn update(&mut self, _message: Message) {
        // Handle updates when we add interaction
    }

    fn view(&self) -> Element<Message> {
        let create_player_view = |player: &Player| {
            container(
                column![row![
                    column![styled_text_with_size(&player.name, 16),]
                        .spacing(4)
                        .width(Length::FillPortion(1)),
                    column![
                        styled_text(&player.ship_name),
                        row![
                            styled_text("PR: "),
                            styled_text_with_color(&format!("{}", player.pr), ORANGE_COLOR)
                        ]
                    ]
                    .spacing(4)
                    .width(Length::FillPortion(1)),
                    column![
                        row![
                            styled_text("Account Battles: "),
                            styled_text_with_color(&format!("{}", player.battles), GREEN_COLOR)
                        ],
                        row![
                            styled_text("Account WR: "),
                            styled_text_with_color(
                                &format!("{:.1}%", player.winrate),
                                if player.winrate >= 50.0 {
                                    ORANGE_COLOR
                                } else {
                                    RED_COLOR
                                }
                            )
                        ],
                    ]
                    .spacing(4)
                    .width(Length::FillPortion(1)),
                    column![
                        row![
                            styled_text("Ship Battles: "),
                            styled_text_with_color(
                                &format!("{}", player.ship_battles),
                                GREEN_COLOR
                            )
                        ],
                        row![
                            styled_text("Ship WR: "),
                            styled_text_with_color(
                                &format!("{:.1}%", player.ship_winrate),
                                if player.ship_winrate >= 50.0 {
                                    ORANGE_COLOR
                                } else {
                                    RED_COLOR
                                }
                            )
                        ],
                    ]
                    .spacing(4)
                    .width(Length::FillPortion(1)),
                    // Right column (battles)
                    column![
                        row![
                            styled_text("Avg Damage: "),
                            styled_text_with_color(
                                &format!("{:.0}", player.avg_damage),
                                ORANGE_COLOR
                            )
                        ],
                        row![
                            styled_text("frags: "),
                            styled_text_with_color(&format!("{}", player.frags), GREEN_COLOR)
                        ]
                    ]
                    .spacing(4)
                    .width(Length::FillPortion(1))
                ]
                .spacing(20)
                .width(Length::Fill)]
                .spacing(5)
                .width(Length::Fill),
            )
            .padding(10)
            .width(Length::Fill)
            .style(theme::Container::Custom(Box::new(
                CustomContainer::PlayerCard,
            )))
            .into()
        };

        let content = row![
            column(self.team1.iter().map(create_player_view).collect())
                .spacing(5)
                .width(Length::FillPortion(1)),
            column(self.team2.iter().map(create_player_view).collect())
                .spacing(5)
                .width(Length::FillPortion(1))
        ]
        .spacing(10)
        .width(Length::Fill);

        let scrollable_content = scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::Scrollable::Custom(Box::new(CustomScrollable)));

        container(scrollable_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(theme::Container::Custom(Box::new(
                CustomContainer::Background,
            )))
            .into()
    }
}

// Discord-like theme implementation
#[derive(Debug, Clone, Copy)]
pub enum CustomContainer {
    Background,
    PlayerCard,
}

impl container::StyleSheet for CustomContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        match self {
            CustomContainer::Background => container::Appearance {
                background: Some(iced::Background::Color(DISCORD_BACKGROUND)),
                ..Default::default()
            },
            CustomContainer::PlayerCard => container::Appearance {
                background: Some(iced::Background::Color(DISCORD_CARD)),
                border_radius: 8.0.into(),
                border_width: 0.0,
                ..Default::default()
            },
        }
    }
}

// Add custom scrollable style
#[derive(Debug, Clone, Copy)]
struct CustomScrollable;

impl scrollable::StyleSheet for CustomScrollable {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            background: Some(iced::Background::Color(DISCORD_BACKGROUND)),
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: DISCORD_BLACK,
                border_radius: 4.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, _style: &Self::Style, is_mouse_over: bool) -> scrollable::Scrollbar {
        self.active(_style)
    }

    fn dragging(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        self.active(_style)
    }
}

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (800, 600);
    StatsViewer::run(settings)
}
