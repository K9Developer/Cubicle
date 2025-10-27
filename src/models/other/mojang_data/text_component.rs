use std::collections::HashMap;
use fastnbt::Value as NBTValue;
use serde::Deserialize;
use serde_json::Value;
use crate::models::other::mojang_data::color::MinecraftColor;

#[derive(Debug)]
struct TextComponentEvent {
    action: String,
    value: NBTValue,
}

impl TextComponentEvent {
    pub fn from_json(mut val: Value) -> Self {
        let v = val.as_object_mut().unwrap();
        TextComponentEvent {
            action: v.remove("action").and_then(|val| val.as_str().map(str::to_owned)).unwrap_or_default(),
            value: NBTValue::deserialize(val).unwrap_or(NBTValue::Compound(HashMap::new())),
        }
    }
}


// TODO: https://minecraft.wiki/w/Text_component_format more accuracy
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

    pub fn from_string(text: &str) -> Self {
        if let Ok(mut json) = serde_json::from_str::<Value>(text) {
            let v = json.as_object_mut().unwrap();
            TextComponent {
                text: v.remove("text").and_then(|val| val.as_str().map(str::to_owned)).unwrap_or_default(),
                color: v.remove("color").and_then(|val| Some(MinecraftColor::from(val.as_str()))).unwrap_or(MinecraftColor::Black),
                bold: v.remove("bold").and_then(|val| val.as_bool()).unwrap_or(false),
                italic: v.remove("italic").and_then(|val| val.as_bool()).unwrap_or(false),
                underline: v.remove("underline").and_then(|val| val.as_bool()).unwrap_or(false),
                obfuscated: v.remove("obfuscated").and_then(|val| val.as_bool()).unwrap_or(false),
                strikethrough: v.remove("strikethrough").and_then(|val| val.as_bool()).unwrap_or(false),
                hover_event: v.remove("hover_event").and_then(|val| Some(TextComponentEvent::from_json(val))),
                click_event: v.remove("click_event").and_then(|val| Some(TextComponentEvent::from_json(val))),
            }
        } else {
            Self::new(text)
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