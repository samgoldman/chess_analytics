use tui::{backend::RustboxBackend, Terminal};

use crate::workflow_step::*;

use tui::{
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, Borders, List, ListItem,
    },
};

pub struct UiMonitorStep {
    terminal: Terminal<RustboxBackend>,
    raw_fields: Vec<(String, String)>,
    length_fields: Vec<(String, String)>,
}

/// chess_analytics_build::register_step_builder "UiMonitorStep" UiMonitorStep
impl UiMonitorStep {
    pub fn try_new(configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        let backend = RustboxBackend::new().expect("Could not create backend");
        let terminal = Terminal::new(backend).expect("Could not create terminal");

        let matches = load_step_config!("UiMonitorStep", "step_arg_configs/ui_monitor_step.yaml", configuration);

        let raw = matches.values_of("raw").unwrap().map(|val| {
            let mut split = val.split(",");
            (split.next().unwrap().to_string(), split.next().unwrap().to_string())
        }).collect();

        let length = matches.values_of("length").unwrap().map(|val| {
            let mut split = val.split(",");
            (split.next().unwrap().to_string(), split.next().unwrap().to_string())
        }).collect();

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
        loop {
            let monitored_data = {
                let unlocked_data = data.lock().unwrap();
                
                let mut raw = self.raw_fields.iter().map(|(title, field)| {
                    let data = unlocked_data.get(field).unwrap_or(&SharedData::SharedBool(false));
                    format!("{}: {}", title, data)
                }).collect::<Vec<String>>();
                
                let mut length = self.length_fields.iter().map(|(title, field)| {
                    let data = unlocked_data.get(field).unwrap_or(&SharedData::SharedVec(vec![])).to_vec().unwrap_or(vec![]).len();
                    format!("{}: {}", title, data)
                }).collect::<Vec<String>>();

                
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

impl std::fmt::Debug for UiMonitorStep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MonitorStep TODO") // TODO
    }
}
