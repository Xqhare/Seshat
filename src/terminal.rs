use crate::Position;
use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::color;

pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
    // (Xqhare): _var is a variable that just isn't used anywhere else in the program but need to exist, it is initialised in impl
    // (Xqhare): e.g. here where into_raw_mode returns something and as long as it is held onto, raw_mode is enabled.
    _stdout: RawTerminal<io::Stdout>,

}

impl Terminal {
    #[allow(clippy::should_implement_trait)]
    /// # Errors
///
/// Will return `Err` if the terminal is not initialised, or `stdout().into_raw_mode` errors.
    pub fn default() -> Result<Self, io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }
    // (Xqhare): Now what does cast_possible_truncation mean?
    // (Xqhare): It has something to do with type casting and not having a catch for the wrong type?
    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position{mut x, mut y} = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }
    /// # Errors
///
/// Will return `Err` if the flush fails.
    pub fn flush() -> Result<(), io::Error> {
        stdout().flush()
    }
    /// # Errors
///
/// Will return `Err` if the next key to be read can not be read.
    pub fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }
    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }
    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }
    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }
}