extern crate rt1we_renderer;

use eframe::egui;
use rt1we_renderer::render::{render, render_parallel};
use rt1we_renderer::image::flipv;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    width: u32,
    height: u32,
    max_depth: u32,
    samples_per_pixel: u32,
    use_parallel: bool,
    texture: Option<egui::TextureHandle>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { width: 160, height: 120, max_depth: 50, samples_per_pixel: 100, use_parallel: true, texture: None }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("rt1we-gui");

            ui.horizontal(|ui| {
                ui.label("Resolution:");
                if ui.button("160×120").clicked() { self.width = 160; self.height = 120; }
                if ui.button("320×240").clicked() { self.width = 320; self.height = 240; }
                if ui.button("640×480").clicked() { self.width = 640; self.height = 480; }
                if ui.button("1280×720").clicked() { self.width = 1280; self.height = 720; }
                if ui.button("1920×1080").clicked() { self.width = 1920; self.height = 1080; }
            });

            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.add(egui::DragValue::new(&mut self.width).range(1..=4000));
            });
            ui.horizontal(|ui| {
                ui.label("Height:");
                ui.add(egui::DragValue::new(&mut self.height).range(1..=4000));
            });
            ui.horizontal(|ui| {
                ui.label("Max depth:");
                ui.add(egui::DragValue::new(&mut self.max_depth).range(1..=200));
            });
            ui.horizontal(|ui| {
                ui.label("Samples per pixel:");
                ui.add(egui::DragValue::new(&mut self.samples_per_pixel).range(1..=1000));
            });

            ui.checkbox(&mut self.use_parallel, "Use parallel renderer");

            ui.separator();

            if ui.button("Render one frame").clicked() {
                let img = if self.use_parallel {
                    render_parallel(
                        self.width as usize,
                        self.height as usize,
                        self.max_depth as usize,
                        self.samples_per_pixel as usize,
                        &rt1we_renderer::geometry::Vec3::new(0.0, 0.0, 0.0),
                        false,
                    )
                } else {
                    render(
                        self.width as usize,
                        self.height as usize,
                        self.max_depth as usize,
                        self.samples_per_pixel as usize,
                        &rt1we_renderer::geometry::Vec3::new(0.0, 0.0, 0.0),
                        false,
                    )
                };
                println!("Render complete: {}x{}", img.width, img.height);
                let img = flipv(&img);
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [img.width, img.height],
                    &img.pixels,
                );
                self.texture = Some(ctx.load_texture("render", color_image, Default::default()));
            }

            if let Some(tex) = &self.texture {
                ui.image(tex);
            }
        });
    }
}
