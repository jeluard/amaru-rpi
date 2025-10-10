use std::time::Duration;

use ratatui::Frame;

pub mod chart;
pub mod color;
pub mod logo;
pub mod scan;

pub trait Screen {
    fn display(&mut self, duration: Duration, frame: &mut Frame);
}
