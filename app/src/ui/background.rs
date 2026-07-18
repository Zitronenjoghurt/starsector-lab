use crate::ui::theme::Theme;
use egui::epaint::{Vertex, WHITE_UV};
use egui::{Color32, Mesh, Painter, Pos2, Rect, Shape, Ui, Vec2, lerp, pos2, vec2};
use std::f32::consts::TAU;
use std::time::Duration;

const DRIFT_SCALE: f32 = 0.001;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct BackgroundSettings {
    pub star_count: usize,
    pub star_size: f32,
    pub min_radius: f32,
    pub max_radius: f32,
    pub min_opacity: f32,
    pub max_opacity: f32,
    pub cyan_chance: f32,
    pub drift_speed: f32,
    pub drift_angle: f32,
    pub parallax: f32,
    pub twinkle_speed: f32,
    pub twinkle_depth: f32,
    pub mouse_parallax: f32,
    pub mouse_response: f32,
    pub vignette_strength: f32,
    pub vignette_reach: f32,
    pub vignette_segments: usize,
}

impl Default for BackgroundSettings {
    fn default() -> Self {
        Self {
            star_count: 1000,
            star_size: 1.0,
            min_radius: 0.4,
            max_radius: 1.7,
            min_opacity: 0.15,
            max_opacity: 0.9,
            cyan_chance: 0.15,
            drift_speed: 0.2,
            drift_angle: 0.0,
            parallax: 15.0,
            twinkle_speed: 1.4,
            twinkle_depth: 0.25,
            mouse_parallax: 0.05,
            mouse_response: 0.1,
            vignette_strength: 1.0,
            vignette_reach: 1.05,
            vignette_segments: 48,
        }
    }
}

struct Star {
    pos: Vec2,
    depth: f32,
    opacity_roll: f32,
    cyan_roll: f32,
    phase: f32,
}

pub struct Background {
    stars: Vec<Star>,
    seed: u64,
    generated_count: usize,
    mouse_offset: Vec2,
}

impl Default for Background {
    fn default() -> Self {
        Self::new(fastrand::u64(..), BackgroundSettings::default().star_count)
    }
}

impl Background {
    pub fn new(seed: u64, star_count: usize) -> Self {
        Self {
            stars: generate_stars(seed, star_count),
            seed,
            generated_count: star_count,
            mouse_offset: Vec2::ZERO,
        }
    }

    pub fn paint(&mut self, ui: &Ui, rect: Rect, settings: &BackgroundSettings) {
        if settings.star_count != self.generated_count {
            self.stars = generate_stars(self.seed, settings.star_count);
            self.generated_count = settings.star_count;
        }

        let (time, pointer) = ui.input(|i| (i.time as f32, i.pointer.latest_pos()));
        self.update_mouse(rect, pointer, settings.mouse_response);

        let painter = ui.painter();
        paint_void(painter, rect);
        self.paint_stars(painter, rect, time, settings);
        paint_vignette(painter, rect, settings);

        ui.ctx().request_repaint_after(Duration::from_millis(16));
    }

    fn update_mouse(&mut self, rect: Rect, pointer: Option<Pos2>, response: f32) {
        let target = match pointer {
            Some(p) => vec2(
                ((p.x - rect.center().x) / rect.width().max(1.0)).clamp(-1.0, 1.0),
                ((p.y - rect.center().y) / rect.height().max(1.0)).clamp(-1.0, 1.0),
            ),
            None => Vec2::ZERO,
        };
        self.mouse_offset += (target - self.mouse_offset) * response.clamp(0.0, 1.0);
    }

    fn paint_stars(&self, painter: &Painter, rect: Rect, time: f32, settings: &BackgroundSettings) {
        let dir = Vec2::angled(settings.drift_angle.to_radians());
        let mouse = self.mouse_offset * settings.mouse_parallax;

        for star in &self.stars {
            let speed = settings.drift_speed * DRIFT_SCALE * (1.0 + settings.parallax * star.depth);
            let moved = star.pos + dir * (time * speed) - mouse * star.depth;
            let center = pos2(
                rect.left() + moved.x.rem_euclid(1.0) * rect.width(),
                rect.top() + moved.y.rem_euclid(1.0) * rect.height(),
            );

            let opacity = lerp(
                settings.min_opacity..=settings.max_opacity,
                star.opacity_roll,
            );
            let twinkle =
                1.0 + settings.twinkle_depth * (time * settings.twinkle_speed + star.phase).sin();
            let alpha = (opacity * twinkle).clamp(0.0, 1.0);

            let base = if star.cyan_roll < settings.cyan_chance {
                Theme::STAR_BRIGHT
            } else {
                Theme::STAR
            };

            let radius =
                lerp(settings.min_radius..=settings.max_radius, star.depth) * settings.star_size;
            painter.circle_filled(center, radius, base.gamma_multiply(alpha));
        }
    }
}

fn generate_stars(seed: u64, star_count: usize) -> Vec<Star> {
    let mut rng = fastrand::Rng::with_seed(seed);
    (0..star_count)
        .map(|_| Star {
            pos: vec2(rng.f32(), rng.f32()),
            depth: rng.f32(),
            opacity_roll: rng.f32(),
            cyan_roll: rng.f32(),
            phase: rng.f32() * TAU,
        })
        .collect()
}

fn vertex(pos: Pos2, color: Color32) -> Vertex {
    Vertex {
        pos,
        uv: WHITE_UV,
        color,
    }
}

fn paint_void(painter: &Painter, rect: Rect) {
    let mut mesh = Mesh::default();
    mesh.vertices.push(vertex(rect.left_top(), Theme::VOID_TOP));
    mesh.vertices
        .push(vertex(rect.right_top(), Theme::VOID_TOP));
    mesh.vertices
        .push(vertex(rect.right_bottom(), Theme::VOID_BOTTOM));
    mesh.vertices
        .push(vertex(rect.left_bottom(), Theme::VOID_BOTTOM));
    mesh.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
    painter.add(Shape::mesh(mesh));
}

fn paint_vignette(painter: &Painter, rect: Rect, settings: &BackgroundSettings) {
    if settings.vignette_strength <= 0.0 {
        return;
    }
    let color = Theme::VIGNETTE.gamma_multiply(settings.vignette_strength);
    let segments = settings.vignette_segments.max(3);
    let center = rect.center();
    let radius = rect.size().length() * 0.5 * settings.vignette_reach;

    let mut mesh = Mesh::default();
    mesh.vertices.push(vertex(center, Color32::TRANSPARENT));
    for i in 0..=segments {
        let angle = i as f32 / segments as f32 * TAU;
        let point = center + vec2(angle.cos(), angle.sin()) * radius;
        mesh.vertices.push(vertex(point, color));
    }
    for i in 1..=segments as u32 {
        mesh.indices.extend_from_slice(&[0, i, i + 1]);
    }
    painter.add(Shape::mesh(mesh));
}
