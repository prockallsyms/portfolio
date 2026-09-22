//! Route pages: one module per page.

pub mod about;
pub mod home;
pub mod not_found;
pub mod projects;

pub use about::About;
pub use home::Home;
pub use not_found::NotFound;
pub use projects::Projects;
