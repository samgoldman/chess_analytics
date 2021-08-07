use crate::workflow_step::Step;

use std::any::*;
use std::io::Write;

pub struct LabelledPrintStep<'a> {
    label: &'a str,
    destination: Box<dyn Write + 'a>,
    destination_description: String,
    passthrough: bool,
    input_type: TypeId,
}

/// chess_analytics_build::register_step_builder "LabelledPrintStep" LabelledPrintStep
impl<'a> LabelledPrintStep<'a> {
    pub fn try_new(configuration: Vec<&'static str>) -> Result<Box<dyn Step>, String> {
        let input_type = match configuration.get(0).unwrap_or(&"usize") {
            &"usize" => TypeId::of::<usize>(),
            &_ => TypeId::of::<usize>(), // Probably should just fail here
        };

        let step = LabelledPrintStep {
            label: configuration.get(1).unwrap_or(&"Usize"),
            destination: Box::new(std::io::stdout()),
            destination_description: "stdout".to_string(),
            passthrough: false,
            input_type: input_type,
        };

        Ok(Box::new(step))
    }
}

macro_rules! downcast_attempt {
    ($type:ident, $input:ident) => {
        if $type == TypeId::of::<usize>() {
            (&*$input).downcast_ref::<usize>()
        } else {
            panic!()
        }
    };
}

impl<'a> Step for LabelledPrintStep<'a> {
    #[allow(clippy::needless_return)] // Allow for coverage
    fn process(&mut self, raw_input: &dyn Any) -> Result<Box<dyn Any>, String> {
        let input_type = self.input_type;
        let downcast_attempt = downcast_attempt!(input_type, raw_input);
        match downcast_attempt {
            Some(downcast) => {
                writeln!(self.destination, "{}: {}", self.label, downcast).unwrap();
                Ok(Box::new(()))
            }
            None => Err("LabelledPrintStep: Could not downcast input!".to_string()),
        }
    }

    fn get_input_type(&self) -> TypeId {
        self.input_type
    }

    fn get_output_type(&self) -> TypeId {
        if self.passthrough {
            self.input_type
        } else {
            TypeId::of::<()>()
        }
    }
}

impl<'a> std::fmt::Debug for LabelledPrintStep<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "LabelledPrintStep {{label: {}, destination_description: {}}}",
            self.label, self.destination_description
        )
    }
}

#[cfg(test)]
mod test_usize_print_step {
    use super::*;
    use mockall::mock;
    use mockall::predicate::eq;
    use std::io::Result;
    use std::str;

    macro_rules! expect_write_call {
        ($mock:ident, $expected:literal) => {
            let s1: &[u8] = $expected;
            $mock
                .expect_write()
                .times(1)
                .with(eq(s1))
                .returning(|input: &[u8]| Result::Ok(input.len()));
        };
    }

    mock! {
        Writer {}
        impl Write for Writer {
            fn write(&mut self, fmt: &[u8]) -> Result<usize>;
            fn flush(&mut self) -> Result<()>;
        }
    }

    #[test]
    fn test_process() {
        let mut mock_writer = MockWriter::new();
        let mut mock_writer2 = MockWriter::new();

        expect_write_call!(mock_writer, b"test");
        expect_write_call!(mock_writer, b": ");
        expect_write_call!(mock_writer, b"0");
        expect_write_call!(mock_writer, b"\n");

        expect_write_call!(mock_writer2, b"Label here");
        expect_write_call!(mock_writer2, b": ");
        expect_write_call!(mock_writer2, b"255");
        expect_write_call!(mock_writer2, b"\n");

        let mut test_step = LabelledPrintStep {
            label: "test",
            destination: Box::new(mock_writer),
            destination_description: "mock_write".to_string(),
            passthrough: false,
            input_type: TypeId::of::<usize>(),
        };

        let _x = test_step.process(&(0 as usize));

        test_step.label = "Label here";
        test_step.destination = Box::new(mock_writer2);
        let _x = test_step.process(&(255 as usize));

        let bad_result = test_step.process(&(42 as u32));
        assert_eq!(
            bad_result.unwrap_err(),
            "LabelledPrintStep: Could not downcast input!"
        )
    }
}
