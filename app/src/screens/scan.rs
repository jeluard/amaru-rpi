//! A Ratatui example that shows the full range of RGB colors that can be displayed in the terminal.
//!
//! Requires a terminal that supports 24-bit color (true color) and unicode.
//!
//! This example also demonstrates how implementing the Widget trait on a mutable reference
//! allows the widget to update its state while it is being rendered. This allows the fps
//! widget to update the fps calculation and the colors widget to update a cached version of
//! the colors to render instead of recalculating them every frame.
//!
//! This is an alternative to using the `StatefulWidget` trait and a separate state struct. It
//! is useful when the state is only used by the widget and doesn't need to be shared with
//! other widgets.
//!
//! This example runs with the Ratatui library code in the branch that you are currently reading.
//! See the [`latest`] branch for the code which works with the most recent Ratatui release.
//!
//! [`latest`]: https://github.com/ratatui/ratatui/tree/latest

use std::time::Duration;

use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::Widget;

#[derive(Debug, Default)]
pub struct ScanScreen {
}

impl crate::screens::Screen for ScanScreen {
    fn display(&mut self, _duration: Duration, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl ScanScreen {
    pub fn new() -> Self {
        Self {
        }
    }
}

/// Implement the Widget trait for &mut App so that it can be rendered
///
/// This is implemented on a mutable reference so that the app can update its state while it is
/// being rendered. This allows the fps widget to update the fps calculation and the colors widget
/// to update the colors to render.
impl Widget for &mut ScanScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let [top] = area.layout(&Layout::vertical([Length(1), Min(0)]));
        let [title] = top.layout(&Layout::horizontal([Min(0), Length(8)]));
       /*
       let widget = BigText::builder()
                .pixel_size(PixelSize::Full)
                .style(Style::new())
                .lines(vec!["AMARU".green().into()])
                .build();
            let qr_code = QrCode::new("https://ratatui.rs").expect("failed to create QR code");
            let widget = QrCodeWidget::new(qr_code).colors(Colors::Inverted);
            frame.render_widget(widget, area);
        */
        Text::from("colors_rgb example. Press q to quit")
            .centered()
            .render(title, buf);
    }
}
