#![warn(clippy::all, rust_2018_idioms)]

pub mod git_app;
pub use git_app::GitApp;
pub mod git_app_renders;
pub use git_app_renders::render_git_app;
