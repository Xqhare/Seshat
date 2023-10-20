use crate::Position;
use crate::Row;
// (Xqhare): fs stands for File System; it's basically the cross-platform os module from python.
use std::fs;
use std::io::{Error, Write};

// (Xqhare): #[derive(Default)] makes a simple [JS-like] constructor for the [JS-like] class Document.
#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
}

impl Document {
    // (Xqhare): That's how you write the doc's error section!
   /// # Errors
///
/// Will return `Err` if `filename` does not exist or the user does not have
/// permission to read it.
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_owned()),
            dirty: false,
        })
    }
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    fn insert_newline(&mut self, at: &Position) {
        // (Xqhare): Check if position is larger than the length of the row. I'm just aborting the function atm if that is true, as it never should be
        // (Xqhare): first check if at end of file, if so add new line below last; if not add new line at y + 1
        if at.y > self.rows.len() {
            return;
        }
        if at.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }
        #[allow(clippy::indexing_slicing)]
        let new_row = self.rows[at.y].split(at.x);
        #[allow(clippy::arithmetic_side_effects)]
        self.rows.insert(at.y + 1, new_row);
    }
    pub fn insert(&mut self, at: &Position, key: char) {
        if at.y > self.rows.len() {
            return;
        }
        self.dirty = true;
        // (Xqhare): Just end the function if a newline was inserted after inserting the newline; nothing will happen with the current row now.
        if key == '\n' {
            self.insert_newline(at);
            return;
        }
        if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, key);
            self.rows.push(row);
        } else {
           #[allow(clippy::indexing_slicing)]
            let row = &mut self.rows[at.y];
            row.insert(at.x, key);
        }
    }
    #[allow(clippy::arithmetic_side_effects, clippy::indexing_slicing)]
    pub fn delete(&mut self, at: &Position) {
        let len = self.rows.len();
        // (Xqhare): if y is larger than length it is out of scope anyway
        if at.y >= len {
            return;
        }
        self.dirty = true;
        if at.x == self.rows[at.y].len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.append(&next_row);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x);
        }
    }
    /// # Errors
///
/// Will return `Err` if the filesystem produced an error when creating or writing the file.
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                // (Xqhare): I do not save newlines, only rows -> so we insert a byte array of \n with b"\n".
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn find(&self, query: &str, after: &Position) -> Option<Position> {
        let mut x = after.x;
        for (y, row) in self.rows.iter().enumerate().skip(after.y) {
            if let Some(x) = row.find(query, x) {
                return Some(Position { x, y });
            }
            x = 0;
        }
        None
    }
}
