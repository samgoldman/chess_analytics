use std::{
    collections::HashMap,
    io::{stdout, Stdout},
};
use termion::raw::{IntoRawMode, RawTerminal};
use tui::{backend::CrosstermBackend, Terminal};

use crossterm::event::{self, Event, KeyCode};

use tui::{
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
};

use crate::workflow_step::{SharedData, Step};

pub struct UiMonitorStep {
    terminal: Terminal<CrosstermBackend<RawTerminal<Stdout>>>,
    raw_fields: Vec<(String, String)>,
    length_fields: Vec<(String, String)>,
    finish_flag_name: String,
    final_results_field_name: String,
    start_time: std::time::Instant,
    elapsed: std::time::Duration,
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl UiMonitorStep {
    pub fn try_new(configuration: Option<serde_yaml::Value>) -> Result<Box<dyn Step>, String> {
        let stdout = stdout().into_raw_mode().expect("Could not init stdout");

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).expect("Could not create terminal");

        let params = match configuration {
            Some(value) => value,
            None => return Err("UiMonitorStep: no parameters provided".to_string()),
        };

        let finish_flag_name = params
            .get("finish_flag")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        let final_results = params
            .get("final_results")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

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
            finish_flag_name,
            elapsed: std::time::Duration::from_millis(0),
            start_time: std::time::Instant::now(),
            final_results_field_name: final_results,
        }))
    }
}

#[cfg_attr(feature = "with_mutagen", ::mutagen::mutate)]
impl Step for UiMonitorStep {
    fn process<'a>(&mut self, data: &mut HashMap<String, SharedData>) -> Result<bool, String> {
        self.start_time = std::time::Instant::now();
        self.terminal.clear().unwrap();
        loop {
            let mut done = false;

            let monitored_data = {
                let mut raw = self
                    .raw_fields
                    .iter()
                    .map(|(title, field)| {
                        let data = data.get(field).unwrap_or(&SharedData::Bool(false));
                        format!("{title}: {data}")
                    })
                    .collect::<Vec<String>>();

                let mut length = self
                    .length_fields
                    .iter()
                    .map(|(title, field)| {
                        let data = data
                            .get(field)
                            .unwrap_or(&SharedData::Vec(vec![]))
                            .to_vec()
                            .unwrap_or_default()
                            .len();
                        format!("{title}: {data}")
                    })
                    .collect::<Vec<String>>();

                raw.append(&mut length);

                if data.contains_key(&self.finish_flag_name)
                    && data.get(&self.finish_flag_name).unwrap().to_bool().unwrap()
                {
                    done = true;
                }

                if !done {
                    self.elapsed = self.start_time.elapsed();
                }

                raw.push(format!("Duration: {:?}", self.elapsed));

                let final_results_map = data
                    .get(&self.final_results_field_name)
                    .unwrap()
                    .to_map()
                    .unwrap();

                for (k, v) in &final_results_map {
                    raw.push(format!("{k:?}: {v}\t"));
                }

                raw.sort();

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

        Ok(true)
    }
}

impl std::fmt::Debug for UiMonitorStep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MonitorStep TODO") // TODO
    }
}
