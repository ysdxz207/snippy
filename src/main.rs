use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use directories::ProjectDirs;
use eframe::egui::{self, ColorImage, TextureHandle};
use image::{GenericImageView, RgbaImage};
use xcap::Monitor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Settings {
    save_directory: Option<PathBuf>,
}

impl Settings {
    fn path() -> Option<PathBuf> {
        ProjectDirs::from("com", "ysdmh", "snippy").map(|d| d.config_dir().join("settings.json"))
    }

    fn load() -> Self {
        Self::path().and_then(|p| fs::read(p).ok()).and_then(|bytes| serde_json::from_slice(&bytes).ok()).unwrap_or_default()
    }

    fn save_directory(&self) -> PathBuf {
        self.save_directory.clone().unwrap_or_else(|| {
            directories::UserDirs::new().and_then(|d| d.download_dir().map(PathBuf::from)).unwrap_or_else(|| PathBuf::from("."))
        })
    }
}

struct SnippyApp {
    settings: Settings,
    image: Option<RgbaImage>,
    texture: Option<TextureHandle>,
    status: String,
}

impl SnippyApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self { settings: Settings::load(), image: None, texture: None, status: "Ready. Click Capture.".into() }
    }

    fn capture(&mut self, ctx: &egui::Context) {
        match capture_primary() {
            Ok(image) => {
                let size = [image.width() as usize, image.height() as usize];
                self.texture = Some(ctx.load_texture("capture", ColorImage::from_rgba_unmultiplied(size, image.as_raw()), egui::TextureOptions::LINEAR));
                self.status = format!("Captured {}x{}", image.width(), image.height());
                self.image = Some(image);
            }
            Err(error) => self.status = format!("Capture failed: {error:#}"),
        }
    }

    fn save(&mut self) {
        let Some(image) = &self.image else { self.status = "Capture an image first.".into(); return; };
        let dir = self.settings.save_directory();
        let path = dir.join(format!("snippy-{}.png", timestamp()));
        match fs::create_dir_all(&dir).and_then(|_| image.save(&path).map_err(std::io::Error::other)) {
            Ok(()) => self.status = format!("Saved {}", path.display()),
            Err(error) => self.status = format!("Save failed: {error}"),
        }
    }
}

impl eframe::App for SnippyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Capture").clicked() { self.capture(ctx); }
                if ui.button("Save").clicked() { self.save(); }
                ui.separator();
                ui.label(&self.status);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                let available = ui.available_size();
                let original = texture.size_vec2();
                let scale = (available.x / original.x).min(available.y / original.y).min(1.0);
                ui.image((texture.id(), original * scale));
            } else { ui.centered_and_justified(|ui| { ui.label("No capture yet."); }); }
        });
    }
}

fn capture_primary() -> Result<RgbaImage> {
    let screen = Monitor::all().context("enumerating displays")?.into_iter().next().context("no display found")?;
    let image = screen.capture_image().context("capturing primary display")?;
    image_from_buffer(image.width() as u32, image.height() as u32, image.buffer_like().to_vec())
}

fn image_from_buffer(width: u32, height: u32, buffer: Vec<u8>) -> Result<RgbaImage> {
    RgbaImage::from_raw(width, height, buffer).context("capture buffer has an unexpected size")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_malformed_capture_buffer() {
        assert!(image_from_buffer(2, 2, vec![0; 3]).is_err());
    }
}

fn timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs().to_string()).unwrap_or_else(|_| "capture".into())
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_inner_size([960.0, 640.0]).with_min_inner_size([720.0, 480.0]), ..Default::default() };
    eframe::run_native("Snippy", options, Box::new(|cc| Ok(Box::new(SnippyApp::new(cc)))))
}
