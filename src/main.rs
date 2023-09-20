#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else,
    clippy::pub_use,
    clippy::partial_pub_fields,
    clippy::question_mark_used,
    clippy::must_use_candidate,
    clippy::pattern_type_mismatch,
    clippy::std_instead_of_core,
    clippy::exhaustive_structs,
    clippy::expect_used,
    clippy::as_conversions,
    clippy::string_slice,
    clippy::return_self_not_must_use,
    clippy::panic,
    clippy::blanket_clippy_restriction_lints
)]
mod document;
mod editor;
mod row;
mod terminal;
pub use document::Document;
use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;
pub use row::Row;

fn main() {
    let _editor = Editor::default();
    Editor::default().run();
}
