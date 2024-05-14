mod health_check;
pub mod user;
pub mod book;
pub mod mainpage;
pub mod bookdisplay;
mod comments;
pub use health_check::*;
pub use book::*;
pub use comments::*;
pub use mainpage::*;
pub use bookdisplay::*;
pub use user::*;
pub const IMAGE_DIRECTORY: &str = "images/";