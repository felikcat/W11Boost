pub mod app;
pub mod config;

pub mod shared_state;
pub mod state;
pub mod theme;
pub mod tweaks;
pub mod widgets;

pub fn run_gui() -> eframe::Result<()>
{
        let options = eframe::NativeOptions {
                viewport: eframe::egui::ViewportBuilder::default()
                        .with_title("W11Boost")
                        .with_maximized(true),
                ..Default::default()
        };

        eframe::run_native(
                "W11Boost",
                options,
                Box::new(|cc| {
                        // Add custom fonts or styles here if needed
                        Ok(Box::new(app::W11BoostApp::new(cc)))
                }),
        )
}
