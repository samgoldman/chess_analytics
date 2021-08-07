use crate::workflow_step::Step;

use std::any::*;
use std::io::Write;

pub struct UsizePrintStep<'a> {
    label: &'a str,
    destination: Box<dyn Write + 'a>,
}

/// chess_analytics_build::register_step_builder "UsizePrintStep" UsizePrintStep
impl<'a> UsizePrintStep<'a> {
    pub fn try_new(configuration: Vec<&'static str>) -> Result<Box<dyn Step>, String> {
        let step = UsizePrintStep {
            label: configuration.get(0).unwrap_or(&"Usize"),
            destination: Box::new(std::io::stdout()),
        };

        Ok(Box::new(step))
    }
}

impl<'a> Step for UsizePrintStep<'a> {
    #[allow(clippy::needless_return)] // Allow for coverage
    fn process(&mut self, raw_input: &dyn Any) -> Result<Box<dyn Any>, String> {
        match (&*raw_input).downcast_ref::<usize>() {
            Some(downcast) => {
                writeln!(self.destination, "{}: {}", self.label, downcast).unwrap();
                Ok(Box::new(()))
            }
            None => Err("UsizePrintStep: Could not downcast input!".to_string()),
        }
    }

    fn get_input_type(&self) -> TypeId {
        TypeId::of::<usize>()
    }

    fn get_output_type(&self) -> TypeId {
        TypeId::of::<()>()
    }
}

impl<'a> std::fmt::Debug for UsizePrintStep<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "UsizePrintStep {{label: {}}}", self.label)
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

        let mut test_step = UsizePrintStep {
            label: "test",
            destination: Box::new(mock_writer),
        };

        let _x = test_step.process(&(0 as usize));

        test_step.label = "Label here";
        test_step.destination = Box::new(mock_writer2);
        let _x = test_step.process(&(255 as usize));

        let bad_result = test_step.process(&(42 as u32));
        assert_eq!(
            bad_result.unwrap_err(),
            "UsizePrintStep: Could not downcast input!"
        )
    }
}
