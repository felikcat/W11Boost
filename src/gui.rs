// W11Boost GUI - Modern tree-view tweaker interface

mod app;
mod config;
mod layout;
pub mod remove_w11boost;
mod shared_state;
mod state;
mod theme;
pub mod tweaks;

pub use app::W11BoostApp;
use eframe::egui;

/// Entry point to run the GUI
pub fn run_gui() -> eframe::Result<()>
{
        let options = eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default()
                        .with_title("W11Boost")
                        .with_maximized(true),
                renderer: eframe::Renderer::Glow,
                ..Default::default()
        };

        eframe::run_native("W11Boost", options, Box::new(|cc| Ok(Box::new(W11BoostApp::new(cc)))))
}
