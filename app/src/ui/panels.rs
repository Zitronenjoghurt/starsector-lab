use egui::{Frame, Style};

pub mod bottom;
pub mod center;
pub mod left;
pub mod right;
pub mod top;

pub fn panel_frame(style: &Style) -> Frame {
    Frame::side_top_panel(style).multiply_with_opacity(0.7)
}
