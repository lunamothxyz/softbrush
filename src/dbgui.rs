use crate::font::print;
use crate::font::Font;
use crate::BitmapARGB32;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

type Callback = fn();

/// A simple UI for debugging purposes
pub struct DbgMenu {
    title: &'static str,
    menu_items: Vec<DbgMenuItem>,
    cursor_index: usize,
}

pub struct DbgMenuItem {
    pub text: String,
    pub callback: Callback,
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

    pub fn cursor_up(&mut self) {
        if self.cursor_index > 0 {
            self.cursor_index -= 1;
        }
    }

    pub fn cursor_down(&mut self) {
        if self.cursor_index < self.menu_items.len() - 1 {
            self.cursor_index += 1;
        }
    }

    /// Executes the callback of the currently selected menu item
    pub fn select(&mut self) {
        let selected_item = &self.menu_items[self.cursor_index];
        (selected_item.callback)();
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

    pub fn add_item(mut self, text: &'static str, callback: Callback) -> Self {
        self.menu_items.push(DbgMenuItem {
            text: text.to_string(),
            callback: callback, 
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
