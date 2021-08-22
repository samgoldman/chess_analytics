use tui::{backend::RustboxBackend, Terminal};

use crate::workflow_step::*;

use tui::{
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, Borders, List, ListItem,
    },
};

pub struct MonitorStep {
    terminal: Terminal<RustboxBackend>,
}

/// chess_analytics_build::register_step_builder "MonitorStep" MonitorStep
impl MonitorStep {
    pub fn try_new(_configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        let backend = RustboxBackend::new().expect("Could not create backend");
        let terminal = Terminal::new(backend).expect("Could not create terminal");

        Ok(Box::new(MonitorStep {
            terminal,
        }))
    }
}

fn shared_as_vec(data: &SharedData) -> Option<Vec<SharedData>> {
    match data {
        SharedData::SharedVec(vec) => Some(vec.to_vec()),
        _ => None,
    }
}

fn shared_as_u64(data: &SharedData) -> Option<u64> {
    match data {
        SharedData::SharedU64(data) => Some(*data),
        _ => None,
    }
}

impl<'a> Step for MonitorStep {
    #[allow(clippy::needless_return)] // Allow for coverage
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        loop {
            let monitored_data = {
                let unlocked_data = data.lock().unwrap();
                vec![
                    (format!("{}: {}", "Pending Games", shared_as_vec(unlocked_data.get("parsed_games").unwrap_or(&SharedData::SharedVec(vec![SharedData::SharedBool(false)]))).unwrap().len())),
                    (format!("{}: {}", "Pending Files", shared_as_vec(unlocked_data.get("raw_file_data").unwrap_or(&SharedData::SharedVec(vec![SharedData::SharedBool(false)]))).unwrap().len())),
                    (format!("{}: {}", "Game Count", shared_as_u64(unlocked_data.get("count_games").unwrap_or(&SharedData::SharedU64(0))).unwrap())),
                    (format!("{}: {:?}", "Done reading files", unlocked_data.get("done_reading_files").unwrap_or(&SharedData::SharedVec(vec![])))),
                    (format!("{}: {:?}", "Done parsing games", unlocked_data.get("done_parsing_games").unwrap_or(&SharedData::SharedVec(vec![])))),
                ]
            };

            let list_items: Vec<ListItem> = monitored_data
                .iter()
                .map(|i| ListItem::new(vec![Spans::from(Span::raw(i))]))
                .collect();

            let list = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            self.terminal.draw(|f| {
                f.render_widget(list, f.size());
            }).expect("Could not draw");

            if let Ok(rustbox::Event::KeyEvent(key)) =
            self.terminal.backend().rustbox().peek_event(std::time::Duration::from_millis(250), false) {
                match key {
                    rustbox::keyboard::Key::Char(c) => {
                        if c == 'q' {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for MonitorStep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MonitorStep TODO") // TODO
    }
}
