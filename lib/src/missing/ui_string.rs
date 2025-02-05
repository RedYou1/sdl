use std::mem::MaybeUninit;

use anyhow::{anyhow, Result};
use sdl2::{
    pixels::Color,
    rect::{FRect, Rect},
    render::Canvas,
    ttf::Font,
    video::Window,
};

pub fn string_size(font: &Font, text: &str) -> Result<Option<(f32, f32)>> {
    let (width, height) = font.size_of(text).map_err(|e| anyhow!(e))?;
    if width <= 8192 && height <= 8192 {
        Ok(Some((width as f32, height as f32)))
    } else {
        Ok(None)
    }
}

#[derive(Clone)]
pub struct UIString {
    font: &'static Font<'static, 'static>,
    text: String,
}

impl Default for UIString {
    fn default() -> Self {
        Self {
            #[allow(invalid_value)]
            font: unsafe { MaybeUninit::zeroed().assume_init() },
            text: String::new(),
        }
    }
}

impl UIString {
    pub fn new(font: &'static Font<'static, 'static>, text: String) -> Result<Option<Self>> {
        string_size(font, text.as_str()).map(|t| t.map(|_| Self { font, text }))
    }

    pub fn new_const(font: &'static Font<'static, 'static>, text: &str) -> Self {
        Self {
            font,
            text: text.to_owned(),
        }
    }

    pub fn insert(&mut self, index: usize, text: char) -> Result<bool> {
        self.text.insert(index, text);
        if string_size(self.font, &self.text)?.is_some() {
            return Ok(true);
        }
        self.text.remove(index);
        Ok(false)
    }

    pub fn insert_str(&mut self, index: usize, text: &str) -> Result<usize> {
        for i in (1..=text.len()).rev() {
            self.text.insert_str(index, &text[..i]);
            if string_size(self.font, &self.text)?.is_some() {
                return Ok(i);
            }
            self.text.drain(index..i);
        }
        Ok(0)
    }

    pub fn drain(&mut self, start: usize, len: usize) -> Result<Option<String>> {
        let text: String = self.text.drain(start..start + len).collect();
        if string_size(self.font, self.text.as_str())?.is_some() {
            return Ok(Some(text));
        }
        self.text.insert_str(start, text.as_str());
        Ok(None)
    }

    pub fn remove(&mut self, index: usize) -> Result<Option<char>> {
        let text = self.text.remove(index);
        if string_size(self.font, &self.text)?.is_some() {
            return Ok(Some(text));
        }
        self.text.insert(index, text);
        Ok(None)
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn size(&self) -> Result<(f32, f32)> {
        string_size(self.font, self.text.as_str())?.ok_or(anyhow!("Checked"))
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        from: Option<FRect>,
        to: FRect,
        color: Color,
    ) -> Result<()> {
        canvas
            .copy_f(
                &canvas
                    .texture_creator()
                    .create_texture_from_surface(
                        self.font
                            .render(&self.text)
                            .blended(color)
                            .map_err(|e| anyhow!(e))?,
                    )
                    .map_err(|e| anyhow!(e))?,
                from.map(|rect| {
                    Rect::new(
                        rect.x() as i32,
                        rect.y() as i32,
                        rect.width() as u32,
                        rect.height() as u32,
                    )
                }),
                to,
            )
            .map_err(|e| anyhow!(e))
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }
}

impl AsRef<str> for UIString {
    fn as_ref(&self) -> &str {
        self.text.as_ref()
    }
}

impl AsRef<String> for UIString {
    fn as_ref(&self) -> &String {
        &self.text
    }
}
