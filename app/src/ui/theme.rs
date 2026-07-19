use egui::{
    Color32, CornerRadius, FontData, FontDefinitions, FontFamily, Stroke, style::Selection,
};
use std::sync::Arc;

pub struct Theme;

impl Theme {
    pub const SPACE_BLACK: Color32 = Color32::from_rgb(0x05, 0x08, 0x0d);

    pub const VOID_TOP: Color32 = Color32::from_rgb(0x07, 0x0d, 0x16);
    pub const VOID_BOTTOM: Color32 = Color32::from_rgb(0x03, 0x05, 0x0a);
    pub const VIGNETTE: Color32 = Color32::from_black_alpha(120);
    pub const STAR: Color32 = Color32::from_rgb(0x93, 0xb4, 0xc9);
    pub const STAR_BRIGHT: Color32 = Color32::from_rgb(0xc4, 0xeb, 0xf8);

    pub const PANEL: Color32 = Color32::from_rgb(0x0e, 0x1a, 0x24);
    pub const PANEL_TRANSLUCENT: Color32 = Color32::from_rgba_premultiplied(0x0c, 0x17, 0x20, 0xe0);
    pub const PANEL_RAISED: Color32 = Color32::from_rgb(0x17, 0x26, 0x32);
    pub const WIDGET: Color32 = Color32::from_rgb(0x1d, 0x30, 0x40);
    pub const WIDGET_ACTIVE: Color32 = Color32::from_rgb(0x27, 0x45, 0x5c);

    pub const CYAN: Color32 = Color32::from_rgb(0x6f, 0xcf, 0xe8);
    pub const CYAN_BRIGHT: Color32 = Color32::from_rgb(0xa9, 0xe7, 0xf6);
    pub const CYAN_DIM: Color32 = Color32::from_rgb(0x46, 0x78, 0x8c);

    pub const TEXT: Color32 = Color32::from_rgb(0xc6, 0xd4, 0xdf);
    pub const AMBER: Color32 = Color32::from_rgb(0xff, 0xb0, 0x4a);
    pub const RED: Color32 = Color32::from_rgb(0xd8, 0x55, 0x45);
    pub const GREEN: Color32 = Color32::from_rgb(0x5c, 0xc9, 0x7a);

    const CORNER: CornerRadius = CornerRadius::same(2);

    pub fn apply(ctx: &egui::Context) {
        Self::apply_fonts(ctx);
        Self::apply_visuals(ctx);
        ctx.all_styles_mut(|style| {
            style.interaction.selectable_labels = false;
            style.spacing.scroll = egui::style::ScrollStyle::solid();
        });
    }

    fn apply_fonts(ctx: &egui::Context) {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "GeistMono".to_owned(),
            Arc::new(FontData::from_static(include_bytes!(
                "../../../assets/fonts/Geist_Mono/GeistMono-Regular.ttf"
            ))),
        );
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            fonts
                .families
                .entry(family)
                .or_default()
                .insert(0, "GeistMono".to_owned());
        }

        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        ctx.set_fonts(fonts);
    }

    fn apply_visuals(ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();

        visuals.override_text_color = Some(Self::TEXT);
        visuals.hyperlink_color = Self::CYAN_BRIGHT;
        visuals.warn_fg_color = Self::AMBER;
        visuals.error_fg_color = Self::RED;

        visuals.panel_fill = Self::PANEL;
        visuals.window_fill = Self::PANEL_TRANSLUCENT;
        visuals.window_stroke = Stroke::new(1.0, Self::CYAN_DIM);
        visuals.window_corner_radius = Self::CORNER;
        visuals.menu_corner_radius = Self::CORNER;

        visuals.window_shadow = egui::epaint::Shadow {
            offset: [0, 8],
            blur: 24,
            spread: 0,
            color: Color32::from_black_alpha(140),
        };
        visuals.popup_shadow = visuals.window_shadow;

        visuals.faint_bg_color = Self::PANEL_RAISED;
        visuals.extreme_bg_color = Self::SPACE_BLACK;
        visuals.code_bg_color = Self::SPACE_BLACK;

        visuals.selection = Selection {
            bg_fill: Self::CYAN_DIM.linear_multiply(0.5),
            stroke: Stroke::new(1.0, Self::CYAN_BRIGHT),
        };

        let w = &mut visuals.widgets;

        w.noninteractive.bg_fill = Self::PANEL;
        w.noninteractive.weak_bg_fill = Self::PANEL;
        w.noninteractive.bg_stroke = Stroke::new(1.0, Self::CYAN_DIM);
        w.noninteractive.fg_stroke = Stroke::new(1.0, Self::TEXT);
        w.noninteractive.corner_radius = Self::CORNER;

        w.inactive.bg_fill = Self::WIDGET;
        w.inactive.weak_bg_fill = Self::WIDGET;
        w.inactive.bg_stroke = Stroke::new(1.0, Self::CYAN_DIM);
        w.inactive.fg_stroke = Stroke::new(1.0, Self::CYAN);
        w.inactive.corner_radius = Self::CORNER;

        w.hovered.bg_fill = Self::WIDGET_ACTIVE;
        w.hovered.weak_bg_fill = Self::WIDGET_ACTIVE;
        w.hovered.bg_stroke = Stroke::new(1.0, Self::CYAN);
        w.hovered.fg_stroke = Stroke::new(1.5, Self::CYAN_BRIGHT);
        w.hovered.corner_radius = Self::CORNER;

        w.active.bg_fill = Self::WIDGET_ACTIVE;
        w.active.weak_bg_fill = Self::WIDGET_ACTIVE;
        w.active.bg_stroke = Stroke::new(1.0, Self::CYAN_BRIGHT);
        w.active.fg_stroke = Stroke::new(2.0, Self::CYAN_BRIGHT);
        w.active.corner_radius = Self::CORNER;

        w.open.bg_fill = Self::WIDGET_ACTIVE;
        w.open.weak_bg_fill = Self::WIDGET_ACTIVE;
        w.open.bg_stroke = Stroke::new(1.0, Self::CYAN);
        w.open.fg_stroke = Stroke::new(1.0, Self::CYAN_BRIGHT);
        w.open.corner_radius = Self::CORNER;

        ctx.set_visuals(visuals);
    }
}
