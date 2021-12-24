use std::io::{stdout, Stdout};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::{backend::TermionBackend, Terminal};

use crossterm::event::{self, Event, KeyCode};

use crate::workflow_step::*;

use tui::{
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
};

pub struct UiMonitorStep {
    terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>,
    raw_fields: Vec<(String, String)>,
    length_fields: Vec<(String, String)>,
}

/// chess_analytics_build::register_step_builder "UiMonitorStep" UiMonitorStep
impl UiMonitorStep {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let stdout = stdout().into_raw_mode().expect("Could not init stdout");

        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend).expect("Could not create terminal");

        let params = match configuration {
            Some(value) => value,
            None => return Err("UiMonitorStep: no parameters provided".to_string()),
        };

        let raw = match params.get("raw").unwrap().as_sequence() {
            Some(values) => values
                .iter()
                .map(|val| {
                    (
                        val.get("display_name")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                        val.get("field").unwrap().as_str().unwrap().to_string(),
                    )
                })
                .collect(),
            None => vec![],
        };

        let length = match params.get("length").unwrap().as_sequence() {
            Some(values) => values
                .iter()
                .map(|val| {
                    (
                        val.get("display_name")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                        val.get("field").unwrap().as_str().unwrap().to_string(),
                    )
                })
                .collect(),
            None => vec![],
        };

        Ok(Box::new(UiMonitorStep {
            terminal,
            raw_fields: raw,
            length_fields: length,
        }))
    }
}

impl<'a> Step for UiMonitorStep {
    #[allow(clippy::needless_return)] // Allow for coverage
    fn process(&mut self, data: StepGeneric) -> Result<(), String> {
        self.terminal.clear().unwrap();
        loop {
            let monitored_data = {
                let unlocked_data = data.lock().unwrap();
                let mut raw = self
                    .raw_fields
                    .iter()
                    .map(|(title, field)| {
                        let data = unlocked_data
                            .get(field)
                            .unwrap_or(&SharedData::Bool(false));
                        format!("{}: {}", title, data)
                    })
                    .collect::<Vec<String>>();

                let mut length = self
                    .length_fields
                    .iter()
                    .map(|(title, field)| {
                        let data = unlocked_data
                            .get(field)
                            .unwrap_or(&SharedData::Vec(vec![]))
                            .to_vec()
                            .unwrap_or_default()
                            .len();
                        format!("{}: {}", title, data)
                    })
                    .collect::<Vec<String>>();

                raw.append(&mut length);

                raw
            };

            let list_items: Vec<ListItem> = monitored_data
                .iter()
                .map(|i| ListItem::new(vec![Spans::from(Span::raw(i))]))
                .collect();

            let list = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            self.terminal
                .draw(|f| {
                    f.render_widget(list, f.size());
                })
                .expect("Could not draw");

            let mut quit = false;
            if event::poll(std::time::Duration::from_millis(30)).unwrap_or(false) {
                if let Event::Key(key) = event::read().unwrap() {
                    if let KeyCode::Char('q') = key.code {
                        quit = true;
                    }
                }
            }

            if quit {
                self.terminal.clear().unwrap();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(332));
        }

        Ok(())
    }
}

impl std::fmt::Debug for UiMonitorStep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MonitorStep TODO") // TODO
    }
}
