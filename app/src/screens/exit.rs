use super::Screen;
use indoc::indoc;
use ratatui::{
    layout::{Constraint, Layout},
    prelude::*,
    text::Text,
    widgets::Clear,
};
use std::time::Duration;

const TEXT_DURATION: Duration = Duration::from_millis(1500);
const CLEAR_DURATION: Duration = Duration::from_millis(200);

const EXIT_TEXT: &str = indoc! {"
▗▖   ▄▄▄▄ ▗▄▄▄▖▗▄▄▖
▐▌   █  █ ▐▌   ▐▌ ▐▌
▐▌   █▀▀█ ▐▛▀▀▘▐▛▀▚▖
▐▙▄▄▖█▄▄█ ▐▙▄▄▖▐▌ ▐▌

 ▗▄▄▖▗▖ ▗▖▄▄▄▄ ▗▄▄▄▖▗▄▄▖
▐▌   ▐▌▗▞▘█  █ ▐▌   ▐▌ ▐▌
 ▝▀▚▖▐▛▚▖ █▀▀█ ▐▛▀▀▘▐▛▀▚▖
▗▄▄▞▘▐▌ ▐▌█▄▄█ ▐▙▄▄▖▐▌ ▐▌
"};

#[derive(Debug)]
enum ExitStage {
    ShowingText,
    ShowingClear,
    Finished,
}

#[derive(Debug)]
pub struct ExitScreen {
    stage: ExitStage,
    time_in_stage: Duration,
}

impl ExitScreen {
    pub fn new() -> Self {
        Self {
            stage: ExitStage::ShowingText,
            time_in_stage: Duration::ZERO,
        }
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.stage, ExitStage::Finished)
    }

    fn on_tick(&mut self, elapsed: Duration) {
        self.time_in_stage += elapsed;
        match self.stage {
            ExitStage::ShowingText if self.time_in_stage >= TEXT_DURATION => {
                self.stage = ExitStage::ShowingClear;
                self.time_in_stage = Duration::ZERO;
            }
            ExitStage::ShowingClear if self.time_in_stage >= CLEAR_DURATION => {
                self.stage = ExitStage::Finished;
            }
            _ => {}
        }
    }
}

impl Screen for ExitScreen {
    fn display(&mut self, elapsed: Duration, frame: &mut Frame) {
        self.on_tick(elapsed);

        match self.stage {
            ExitStage::ShowingText => {
                let text = Text::raw(EXIT_TEXT);

                let vertical_chunks = Layout::vertical([
                    Constraint::Fill(1),
                    Constraint::Length(9),
                    Constraint::Fill(1),
                ])
                .split(frame.area());

                let horizontal_chunks = Layout::horizontal([
                    Constraint::Fill(1),
                    Constraint::Length(25),
                    Constraint::Fill(1),
                ])
                .split(vertical_chunks[1]);

                frame.render_widget(text, horizontal_chunks[1]);
            }
            ExitStage::ShowingClear => {
                frame.render_widget(Clear, frame.area());
            }
            ExitStage::Finished => {
                // The main loop will catch this and exit
            }
        }
    }
}
