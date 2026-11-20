//Here is the state of our mod.rs, thats controling the tui;
pub mod app;
mod draw_app;
mod events;

pub use app::App;
pub use draw_app::draw;
pub use events::events_on_keyboard;