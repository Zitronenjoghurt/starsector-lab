use egui::{Frame, Margin, Style};

pub mod bottom;
pub mod center;
pub mod left;
pub mod right;
pub mod top;

pub const PANEL_MARGIN: Margin = Margin {
    left: 10,
    right: 10,
    top: 6,
    bottom: 6,
};

pub fn panel_frame(style: &Style) -> Frame {
    Frame::side_top_panel(style)
        .multiply_with_opacity(0.5)
        .inner_margin(PANEL_MARGIN)
}
