//! Configuration for common color values bundled into a theme

use crossterm::{
    cursor::SetCursorStyle,
    style::{ContentStyle, Stylize},
};

pub struct Theme {
    pub out_style: ContentStyle,
    pub err_style: ContentStyle,
    pub selection_style: ContentStyle,
    pub completion_style: ContentStyle,
    pub suggestion_style: ContentStyle,
    pub normal_cursor_style: SetCursorStyle,
    pub insert_cursor_style: SetCursorStyle,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            out_style: ContentStyle::new().white(),
            err_style: ContentStyle::new().red(),
            selection_style: ContentStyle::new().white(),
            completion_style: ContentStyle::new().red(),
            suggestion_style: ContentStyle::new().dark_grey(),
            normal_cursor_style: SetCursorStyle::BlinkingBlock,
            insert_cursor_style: SetCursorStyle::BlinkingBar,
        }
    }
}
