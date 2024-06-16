pub use app::App;
pub use event::Event;
pub use event::EventHandler;
pub use tui::Tui;

/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;
