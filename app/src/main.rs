use crate::button::Button;
use crate::button::ButtonPressEvent;
use crate::screens::Screen;
use crate::screens::chart::ChartScreen;
use crate::screens::color::ColorScreen;
use crate::screens::exit::ExitScreen;
use crate::screens::logo::LogoScreen;
use ratatui::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

mod backends;
mod button;
mod screens;

// Demos from https://github.com/j-g00da/mousefood-esp32-demo/
// https://github.com/j-g00da/mousefood/tree/main/examples/simulator
// https://github.com/orhun/embedded-ratatui-workshop/blob/main/apps/simulator/src/main.rs

enum CurrentScreen {
    Chart,
    Color,
    Exit,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let splash_duration = Duration::from_millis(2000);
    let mut logo = LogoScreen::new(splash_duration);
    let mut chart_screen = ChartScreen::new();
    let mut color_screen = ColorScreen::default();
    let mut exit_screen = ExitScreen::new();

    #[cfg(feature = "display_hat")]
    let (backend, pins) = backends::display_hat::create_backend();

    #[cfg(feature = "simulator")]
    let backend = backends::simulator::create_backend();

    let mut terminal = Terminal::new(backend).unwrap();

    let mut button_a = Button::new();
    let mut button_b = Button::new();
    let mut button_x = Button::new();
    let mut button_y = Button::new();

    let mut current_screen = CurrentScreen::Chart;

    let startup = Instant::now();
    let mut last_frame = Instant::now();
    let running = Arc::new(AtomicBool::new(true));
    while running.load(Ordering::SeqCst) {
        let elapsed = last_frame.elapsed();
        last_frame = Instant::now();
        let show_first = startup.elapsed() < splash_duration;

        #[cfg(feature = "display_hat")]
        {
            // Check Button A to quit
            if let Some(_event) = button_a.update(!pins.button_a.is_high()) {
                println!("Button A pressed. Exiting...");
                current_screen = CurrentScreen::Exit;
            }

            // Check Button B to switch screens
            if let Some(event) = button_b.update(!pins.button_b.is_high())
                && matches!(event, ButtonPressEvent::Short)
            {
                println!("Button B pressed. Switching screen...");
                current_screen = match current_screen {
                    CurrentScreen::Chart => CurrentScreen::Color,
                    CurrentScreen::Color => CurrentScreen::Chart,
                    CurrentScreen::Exit => CurrentScreen::Exit,
                };
            }

            // Placeholders for other buttons
            if let Some(_event) = button_x.update(!pins.button_x.is_high()) {
                println!("Button X pressed.");
            }
            if let Some(_event) = button_y.update(!pins.button_y.is_high()) {
                println!("Button Y pressed.");
            }
        }

        terminal.draw(|frame| {
            if show_first {
                logo.display(elapsed, frame);
            } else {
                match current_screen {
                    CurrentScreen::Chart => chart_screen.display(elapsed, frame),
                    CurrentScreen::Color => color_screen.display(elapsed, frame),
                    CurrentScreen::Exit => exit_screen.display(elapsed, frame),
                }
            }
        })?;

        if matches!(current_screen, CurrentScreen::Exit) && exit_screen.is_finished() {
            running.store(false, Ordering::SeqCst);
        }
    }

    Ok(())
}
