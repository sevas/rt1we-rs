extern crate rt1we_renderer;

use eframe::egui;
use rt1we_renderer::render::render;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    width: u32,
    height: u32,
    max_depth: u32,
    samples_per_pixel: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { width: 160, height: 120, max_depth: 50, samples_per_pixel: 100 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("rt1we-gui");

            ui.add(egui::Slider::new(&mut self.width, 0..=4000).text("Width"));
            ui.add(egui::Slider::new(&mut self.height, 0..=4000).text("Height"));
            ui.add(egui::Slider::new(&mut self.max_depth, 0..=200).text("Height"));
            ui.add(egui::Slider::new(&mut self.samples_per_pixel, 0..=1000).text("Height"));

            ui.separator();

            if ui.button("Render one frame").clicked() {
                let img = render(
                    self.width as usize,
                    self.height as usize,
                    self.max_depth as usize,
                    self.samples_per_pixel as usize,
                    &rt1we_renderer::geometry::Vec3::new(0.0, 0.0, 0.0),
                );
            }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
