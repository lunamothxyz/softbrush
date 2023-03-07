use crate::font::print;
use crate::font::Font;
use crate::BitmapARGB32;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;

/// A simple UI for debugging purposes
pub struct DbgMenu {
    title: &'static str,
    menu_items: Vec<DbgMenuItem>,
    cursor_index: usize,
}

pub struct DbgMenuItem {
    pub text: String,
    pub callback: Box<dyn Fn(&mut DbgMenu)>,
}

impl DbgMenu {
    pub fn builder() -> DbgMenuBuilder {
        DbgMenuBuilder::new()
    }

    fn assemble_string(&self) -> String {
        let mut result = String::new();
        result.push('[');
        result.push_str(self.title);
        result.push_str("]\n");
        for (i, item) in self.menu_items.iter().enumerate() {
            if i == self.cursor_index {
                result.push('*');
            } else {
                result.push(' ');
            }
            result.push_str(&item.text);
            result.push('\n');
        }
        result
    }

    pub fn draw(&self, font: &Font, dest: &mut BitmapARGB32, x: usize, y: usize) {
        let text = self.assemble_string();
        print(x, y, font, dest, &text);
    }
}

pub struct DbgMenuBuilder {
    title: &'static str,
    font: Option<Font>,
    menu_items: Vec<DbgMenuItem>,
}

impl DbgMenuBuilder {
    pub fn new() -> Self {
        Self {
            title: "???",
            font: None,
            menu_items: Vec::new(),
        }
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn add_item(mut self, text: &str, callback: impl Fn(&mut DbgMenu) + 'static) -> Self {
        self.menu_items.push(DbgMenuItem {
            text: text.to_string(),
            callback: Box::new(callback),
        });
        self
    }

    pub fn build(self) -> DbgMenu {
        DbgMenu {
            title: self.title,
            menu_items: self.menu_items,
            cursor_index: 0,
        }
    }
}
