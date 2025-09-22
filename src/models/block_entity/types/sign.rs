use crate::constants::constants::MAX_SIGN_LINE_COUNT;
use crate::models::block_entity::block_entity::GenericBlockEntity;
use crate::models::other::mojang_data::color::MinecraftColor;
use crate::models::other::mojang_data::text_component::TextComponent;

pub struct SignTextBlock {
    is_glowing_text: bool,
    dyed_color: MinecraftColor,
    lines: Vec<TextComponent>,
}

impl SignTextBlock {
    pub fn new(is_glowing_text: bool, dyed_color: MinecraftColor, lines: Vec<TextComponent>) -> Self {
        SignTextBlock {
            is_glowing_text, dyed_color, lines,
        }
    }
}

// sign, hanging sign
pub struct SignBlockEntity {
    base: GenericBlockEntity,
    is_waxed: bool,
    front_text: SignTextBlock,
    back_text: SignTextBlock,
}

impl SignBlockEntity {
    pub fn new(
        base: GenericBlockEntity,
        is_waxed: bool,
        front_text: SignTextBlock,
        back_text: SignTextBlock,
    ) -> Self {
        SignBlockEntity {
            base, is_waxed, front_text, back_text,
        }
    }

    pub fn base(&self) -> &GenericBlockEntity { &self.base }
    pub fn is_waxed(&self) -> &bool { &self.is_waxed }
    pub fn front_text(&self) -> &SignTextBlock { &self.front_text }
    pub fn back_text(&self) -> &SignTextBlock { &self.back_text }
    pub fn all_lines(&self) -> Vec<&TextComponent> {
        let mut lines = Vec::with_capacity(MAX_SIGN_LINE_COUNT);
        lines.extend(self.front_text.lines.iter());
        lines.extend(self.back_text.lines.iter());
        lines
    }
    pub fn front_text_at(&self, line: u8) -> Option<&TextComponent> { self.front_text.lines.get(line as usize) }
    pub fn back_text_at(&self, line: u8) -> Option<&TextComponent> { self.back_text.lines.get(line as usize) }

    pub fn set_is_waxed(&mut self, is_waxed: bool) { self.is_waxed = is_waxed; }
    pub fn set_front_text(&mut self, front_text: SignTextBlock) { self.front_text = front_text; }
    pub fn set_back_text(&mut self, back_text: SignTextBlock) { self.back_text = back_text; }
    pub fn set_front_line_at(&mut self, line: u8, text: TextComponent) { self.front_text.lines.insert(line as usize, text); }
    pub fn set_back_line_at(&mut self, line: u8, text: TextComponent) { self.back_text.lines.insert(line as usize, text); }
    pub fn clear_front(&mut self) { self.front_text.lines.clear(); }
    pub fn clear_back(&mut self) { self.back_text.lines.clear(); }
    pub fn clear_all(&mut self) { self.clear_front(); self.clear_back(); }
}