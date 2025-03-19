use iced::application::StyleSheet;
use iced::font::Weight;
use iced::theme::{self, Container as ThemeContainer, Text as TextTheme};
use iced::widget::{column, container, row, text, Container, Text};
use iced::Color;
use iced::Font;
use iced::{Element, Length, Sandbox, Settings, Theme};

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
    clan: String,
    winrate: f32,
    battles: u32,
    karma: i32,
    pr: u32,
    avg_damage: f32,
    wins: u32,
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
                name: "NKstardream".to_string(),
                clan: "[KA-ZE]".to_string(),
                winrate: 49.96,
                battles: 2754,
                karma: 0,
                pr: 856,
                avg_damage: 84849.0,
                wins: 19,
            },
            Player {
                name: "Satsuma".to_string(),
                clan: "[NAVA]".to_string(),
                winrate: 47.22,
                battles: 4852,
                karma: 0,
                pr: 1425,
                avg_damage: 132932.0,
                wins: 282,
            },
            Player {
                name: "Vermont".to_string(),
                clan: "[DAR]".to_string(),
                winrate: 43.45,
                battles: 1991,
                karma: 0,
                pr: 485,
                avg_damage: 57493.0,
                wins: 64,
            },
            Player {
                name: "Musashi".to_string(),
                clan: "[TH_P]".to_string(),
                winrate: 45.02,
                battles: 844,
                karma: 0,
                pr: 892,
                avg_damage: 71441.0,
                wins: 7,
            },
            Player {
                name: "Jinan".to_string(),
                clan: "[SSR-I]".to_string(),
                winrate: 48.53,
                battles: 5110,
                karma: 0,
                pr: 447,
                avg_damage: 45591.0,
                wins: 4,
            },
            Player {
                name: "Petropavlovsk".to_string(),
                clan: "".to_string(),
                winrate: 47.3,
                battles: 9349,
                karma: 0,
                pr: 1248,
                avg_damage: 51313.0,
                wins: 42,
            },
        ];

        let team2 = vec![
            Player {
                name: "PandaSlime".to_string(),
                clan: "[DWBH]".to_string(),
                winrate: 49.92,
                battles: 2644,
                karma: 0,
                pr: 1350,
                avg_damage: 103170.0,
                wins: 122,
            },
            Player {
                name: "warm_light37".to_string(),
                clan: "[SIGN]".to_string(),
                winrate: 49.49,
                battles: 2623,
                karma: 0,
                pr: 1121,
                avg_damage: 139917.0,
                wins: 59,
            },
            Player {
                name: "AlbertvonBismarck".to_string(),
                clan: "[SJTU]".to_string(),
                winrate: 49.74,
                battles: 2280,
                karma: 0,
                pr: 1236,
                avg_damage: 105548.0,
                wins: 45,
            },
            Player {
                name: "Shikishima".to_string(),
                clan: "[EZ]".to_string(),
                winrate: 47.21,
                battles: 2923,
                karma: 0,
                pr: 892,
                avg_damage: 92047.0,
                wins: 39,
            },
            Player {
                name: "Minnesota".to_string(),
                clan: "[ROYAL]".to_string(),
                winrate: 53.43,
                battles: 3509,
                karma: 0,
                pr: 962,
                avg_damage: 66757.0,
                wins: 6,
            },
            Player {
                name: "Bremus".to_string(),
                clan: "[SAIKO]".to_string(),
                winrate: 51.96,
                battles: 4419,
                karma: 0,
                pr: 1009,
                avg_damage: 119177.0,
                wins: 19,
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
                column![
                    row![
                        text(&player.clan)
                            .size(16)
                            .style(Color::from_rgb(0.7, 0.7, 0.7))
                            .font(CUSTOM_FONT),
                        text(&player.name)
                            .size(16)
                            .style(Color::from_rgb(0.9, 0.9, 0.9))
                            .font(CUSTOM_FONT)
                    ]
                    .spacing(8)
                    .width(Length::Fill),
                    row![
                        text(format!("Winrate: {:.2}%", player.winrate))
                            .size(14)
                            .style(if player.winrate >= 50.0 {
                                Color::from_rgb(0.2, 0.8, 0.2)
                            } else {
                                Color::from_rgb(0.8, 0.2, 0.2)
                            })
                            .font(CUSTOM_FONT),
                        text(format!("Battles: {}", player.battles))
                            .size(14)
                            .style(Color::from_rgb(0.8, 0.8, 0.8))
                            .font(CUSTOM_FONT),
                        text(format!("PR: {}", player.pr))
                            .size(14)
                            .style(Color::from_rgb(0.8, 0.8, 0.2))
                            .font(CUSTOM_FONT)
                    ]
                    .spacing(15)
                    .width(Length::Fill),
                    row![
                        text(format!("Avg Damage: {:.0}", player.avg_damage))
                            .size(14)
                            .style(Color::from_rgb(0.8, 0.8, 0.8))
                            .font(CUSTOM_FONT),
                        text(format!("Wins: {}", player.wins))
                            .size(14)
                            .style(Color::from_rgb(0.8, 0.8, 0.8))
                            .font(CUSTOM_FONT)
                    ]
                    .spacing(15)
                    .width(Length::Fill)
                ]
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

        let team1_column = column(self.team1.iter().map(create_player_view).collect())
            .spacing(5)
            .width(Length::Fill);
        let team2_column = column(self.team2.iter().map(create_player_view).collect())
            .spacing(5)
            .width(Length::Fill);

        container(
            row![
                container(team1_column)
                    .width(Length::FillPortion(1))
                    .padding(5),
                container(team2_column)
                    .width(Length::FillPortion(1))
                    .padding(5)
            ]
            .spacing(10)
            .width(Length::Fill)
            .height(Length::Fill),
        )
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
                background: Some(iced::Background::Color(Color::from_rgb(
                    0.169, 0.176, 0.192, // #2B2D31 - Discord background
                ))),
                ..Default::default()
            },
            CustomContainer::PlayerCard => container::Appearance {
                background: Some(iced::Background::Color(Color::from_rgb(
                    0.208, 0.216, 0.235, // #35373C - Discord message/card color
                ))),
                border_radius: 8.0.into(),
                border_width: 0.0,
                ..Default::default()
            },
        }
    }
}

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (800, 600);
    StatsViewer::run(settings)
}
