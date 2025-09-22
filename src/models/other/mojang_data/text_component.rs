use std::collections::HashMap;
use fastnbt::Value;
use crate::models::other::mojang_data::color::MinecraftColor;

#[derive(Debug)]
struct TextComponentEvent {
    action: String,
    value: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct TextComponent {
    text: String,
    color: MinecraftColor,
    bold: bool,
    italic: bool,
    underline: bool,
    obfuscated: bool,
    strikethrough: bool,
    hover_event: Option<TextComponentEvent>,
    click_event: Option<TextComponentEvent>,
}

impl TextComponent {
    pub fn new(text: &str) -> Self {
        TextComponent {
            text: text.to_string(),
            color: MinecraftColor::Black,
            bold: false,
            italic: false,
            underline: false,
            obfuscated: false,
            strikethrough: false,
            hover_event: None,
            click_event: None,
        }
    }

    pub fn is_bold(&self) -> bool { self.bold }
    pub fn is_italic(&self) -> bool { self.italic }
    pub fn is_underline(&self) -> bool { self.underline }
    pub fn is_obfuscated(&self) -> bool { self.obfuscated }
    pub fn is_strikethrough(&self) -> bool { self.strikethrough }
    pub fn hover_event(&self) -> &Option<TextComponentEvent> { &self.hover_event }
    pub fn click_event(&self) -> &Option<TextComponentEvent> { &self.click_event }
    pub fn text(&self) -> &String { &self.text }
    pub fn color(&self) -> &MinecraftColor { &self.color }

    pub fn set_bold(&mut self, b: bool) { self.bold = b; }
    pub fn set_italic(&mut self, i: bool) { self.italic = i; }
    pub fn set_underline(&mut self, u: bool) { self.underline = u; }
    pub fn set_obfuscated(&mut self, o: bool) { self.obfuscated = o; }
    pub fn set_strikethrough(&mut self, s: bool) { self.strikethrough = s; }
    pub fn set_hover_event(&mut self, h: Option<TextComponentEvent>) { self.hover_event = h; }
    pub fn set_click_event(&mut self, h: Option<TextComponentEvent>) { self.click_event = h; }

    pub fn clear_style(&mut self) {
        self.bold = false;
        self.italic = false;
        self.underline = false;
        self.obfuscated = false;
        self.strikethrough = false;
    }
}