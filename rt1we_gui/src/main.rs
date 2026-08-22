extern crate rt1we_renderer;

mod history;

use eframe::egui;
use history::{HistoryEntry, RenderMeta};
use rt1we_renderer::render::{render, render_parallel};
use rt1we_renderer::image::flipv;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

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
    zoom: f32,
    pan: egui::Vec2,
    history_dir: PathBuf,
    history: Vec<HistoryEntry>,
    thumb_textures: HashMap<u128, egui::TextureHandle>,
    selected_history_id: Option<u128>,
}

impl Default for MyApp {
    fn default() -> Self {
        let history_dir = history::ensure_history_dir();
        let history = history::load_history(&history_dir);
        Self {
            width: 160,
            height: 120,
            max_depth: 50,
            samples_per_pixel: 100,
            use_parallel: true,
            texture: None,
            zoom: 1.0,
            pan: egui::Vec2::ZERO,
            history_dir,
            history,
            thumb_textures: HashMap::new(),
            selected_history_id: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut clicked_history_id: Option<u128> = None;

        egui::SidePanel::right("history_panel").resizable(true).default_width(280.0).show(ctx, |ui| {
            ui.heading("History");
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.history.is_empty() {
                    ui.weak("No renders yet");
                }

                for entry in &self.history {
                    let id = entry.id;
                    if !self.thumb_textures.contains_key(&id) {
                        let thumb = history::load_image(&entry.thumb_path);
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            [thumb.width, thumb.height],
                            &thumb.pixels,
                        );
                        let tex = ctx.load_texture(
                            format!("thumb_{id}"),
                            color_image,
                            egui::TextureOptions::NEAREST,
                        );
                        self.thumb_textures.insert(id, tex);
                    }
                    let tex = &self.thumb_textures[&id];

                    let selected = self.selected_history_id == Some(id);
                    let fill = if selected {
                        ui.visuals().selection.bg_fill
                    } else {
                        ui.visuals().panel_fill
                    };
                    let resp = egui::Frame::group(ui.style())
                        .fill(fill)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.image((tex.id(), egui::vec2(48.0, 36.0)));
                                ui.vertical(|ui| {
                                    ui.label(format!("{}×{}", entry.meta.width, entry.meta.height));
                                    ui.label(format!(
                                        "depth {} · spp {}",
                                        entry.meta.max_depth, entry.meta.samples_per_pixel
                                    ));
                                    ui.label(if entry.meta.use_parallel { "parallel" } else { "scalar" });
                                });
                            });
                        })
                        .response
                        .interact(egui::Sense::click());

                    if resp.clicked() {
                        clicked_history_id = Some(id);
                    }
                }
            });
        });

        if let Some(id) = clicked_history_id {
            if let Some(entry) = self.history.iter().find(|e| e.id == id) {
                let img = history::load_image(&entry.image_path);
                let color_image =
                    egui::ColorImage::from_rgba_unmultiplied([img.width, img.height], &img.pixels);
                self.texture =
                    Some(ctx.load_texture("render", color_image, egui::TextureOptions::NEAREST));
                self.width = entry.meta.width as u32;
                self.height = entry.meta.height as u32;
                self.max_depth = entry.meta.max_depth as u32;
                self.samples_per_pixel = entry.meta.samples_per_pixel as u32;
                self.use_parallel = entry.meta.use_parallel;
                self.selected_history_id = Some(id);
            }
        }

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

            ui.horizontal(|ui| {
                if ui.button("Render one frame").clicked() {
                    let start = Instant::now();
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
                    let render_ms = start.elapsed().as_millis();
                    println!("Render complete: {}x{}", img.width, img.height);
                    let img = flipv(&img);

                    let meta = RenderMeta {
                        width: self.width as usize,
                        height: self.height as usize,
                        max_depth: self.max_depth as usize,
                        samples_per_pixel: self.samples_per_pixel as usize,
                        use_parallel: self.use_parallel,
                        render_ms,
                    };
                    let entry = history::save_render(&self.history_dir, &img, meta);
                    self.selected_history_id = Some(entry.id);
                    self.history.insert(0, entry);

                    let color_image = egui::ColorImage::from_rgba_unmultiplied(
                        [img.width, img.height],
                        &img.pixels,
                    );
                    self.texture =
                        Some(ctx.load_texture("render", color_image, egui::TextureOptions::NEAREST));
                }

                if ui.button("Reset view").clicked() {
                    self.zoom = 1.0;
                    self.pan = egui::Vec2::ZERO;
                }
            });

            if let Some(tex) = &self.texture {
                let (rect, response) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());

                self.pan += response.drag_delta();

                if let Some(pointer) = response.hover_pos() {
                    let scroll = ui.input(|i| i.smooth_scroll_delta.y);
                    if scroll != 0.0 {
                        let old_zoom = self.zoom;
                        self.zoom = (self.zoom * (scroll * 0.002).exp()).clamp(0.05, 40.0);

                        // Keep the point under the cursor stationary while zooming.
                        let center = rect.center() + self.pan;
                        let cursor_from_center = pointer - center;
                        self.pan -= cursor_from_center * (self.zoom / old_zoom - 1.0);
                    }
                }

                let img_size = tex.size_vec2() * self.zoom;
                let img_rect = egui::Rect::from_center_size(rect.center() + self.pan, img_size);

                let painter = ui.painter_at(rect);
                painter.image(
                    tex.id(),
                    img_rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
        });
    }
}
