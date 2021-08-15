use crate::workflow_step::*;

use std::any::*;
use std::io::Write;

pub struct LabelledPrintStep<'a> {
    label: String,
    destination: Box<dyn Write + 'a>,
    destination_description: String,
    passthrough: bool,
    input_type: TypeId,
    input_type_description: String,
}

/// chess_analytics_build::register_step_builder "LabelledPrintStep" LabelledPrintStep
impl<'a> LabelledPrintStep<'a> {
    pub fn try_new(configuration: Vec<String>) -> Result<Box<dyn Step>, String> {
        let input_type_description = configuration
            .get(0)
            .unwrap_or(&"usize".to_string())
            .to_string();
        let input_type = match input_type_description.as_ref() {
            "usize" => TypeId::of::<usize>(),
            _ => TypeId::of::<usize>(), // Probably should just fail here
        };

        let label = configuration
            .get(1)
            .unwrap_or(&"Usize".to_string())
            .to_string();

        let step = LabelledPrintStep {
            label,
            destination: Box::new(std::io::stdout()),
            destination_description: "stdout".to_string(),
            passthrough: false,
            input_type,
            input_type_description,
        };

        Ok(Box::new(step))
    }
}

// macro_rules! downcast_attempt {
//     ($type:ident, $input:ident) => {
//         if $type == TypeId::of::<usize>() {
//             (&*$input).downcast_ref::<usize>()
//         } else {
//             return Err("LabelledPrintStep: could not downcast".to_string());
//         }
//     };
// }

impl<'a> Step for LabelledPrintStep<'a> {
    fn process(&mut self, _data: StepGeneric) -> Result<(), String> {
        // let input_type = self.input_type;
        // let downcast_attempt = downcast_attempt!(input_type, raw_input);
        // match downcast_attempt {
        //     Some(downcast) => {
        //         writeln!(self.destination, "{}: {}", self.label, downcast).unwrap();
        //         Ok(Box::new(()))
        //     }
        //     None => Err("LabelledPrintStep: Could not downcast input!".to_string()),
        // }
        Ok(())
    }
}

impl<'a> std::fmt::Debug for LabelledPrintStep<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "LabelledPrintStep {{label: {}, destination_description: {}, passthrough: {}, input_type_description: {}}}",
            self.label, self.destination_description, self.passthrough, self.input_type_description,
        )
    }
}
