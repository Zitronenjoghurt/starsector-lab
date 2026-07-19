use crate::ui::icons;
use crate::ui::theme::Theme;
use egui::{
    Align2, Color32, ColorImage, FontId, Rect, Sense, TextureHandle, TextureOptions, Ui, Vec2,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct SpriteCache {
    textures: HashMap<PathBuf, Option<TextureHandle>>,
}

impl SpriteCache {
    pub fn get(&mut self, ctx: &egui::Context, path: &Path) -> Option<TextureHandle> {
        if let Some(entry) = self.textures.get(path) {
            return entry.clone();
        }
        let texture = load_texture(ctx, path);
        self.textures.insert(path.to_owned(), texture.clone());
        texture
    }
}

fn load_texture(ctx: &egui::Context, path: &Path) -> Option<TextureHandle> {
    let bytes = std::fs::read(path).ok()?;
    let image = image::load_from_memory(&bytes).ok()?.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let color = ColorImage::from_rgba_unmultiplied(size, image.as_raw());
    Some(ctx.load_texture(path.to_string_lossy(), color, TextureOptions::LINEAR))
}

pub fn show_sprite(ui: &mut Ui, cache: &mut SpriteCache, path: Option<&Path>, size: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());

    let texture = path.and_then(|p| cache.get(ui.ctx(), p));
    match texture {
        Some(texture) => {
            let image_size = texture.size_vec2();
            let scale = (size / image_size.x).min(size / image_size.y).min(1.0);
            let draw = Rect::from_center_size(rect.center(), image_size * scale);
            ui.painter().image(
                texture.id(),
                draw,
                Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        }
        None => {
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                icons::IMAGE,
                FontId::proportional(size * 0.6),
                Theme::CYAN_DIM,
            );
        }
    }
}
