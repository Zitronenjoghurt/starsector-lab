mod app;
mod ui;

fn main() {
    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_drag_and_drop(true)
            .with_title("Starsector Lab")
            .with_app_id("io.github.zitronenjoghurt.starsector-lab"),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "Starsector Lab",
        native_options,
        Box::new(|cc| Ok(Box::new(app::Lab::new(cc)))),
    )
    .expect("Failed to run egui application.");
}
