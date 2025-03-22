use iced::widget::text;
use iced::widget::Text;
use iced::Color;
use iced::Font;

use crate::colors::*;

// Define the custom font
pub const CUSTOM_FONT: Font = Font {
    family: iced::font::Family::SansSerif,
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    monospaced: false,
};

pub fn styled_text<'a>(content: &str) -> Text<'a> {
    text(content).size(14).font(CUSTOM_FONT).style(TEXT_COLOR)
}

pub fn styled_text_with_color<'a>(content: &str, color: Color) -> Text<'a> {
    text(content).size(14).font(CUSTOM_FONT).style(color)
}

pub fn styled_text_with_size<'a>(content: &str, size: u16) -> Text<'a> {
    text(content).size(size).font(CUSTOM_FONT).style(TEXT_COLOR)
}
