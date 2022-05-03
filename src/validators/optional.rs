use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::build_tools::SchemaDict;
use crate::input::{CombinedInput, Input};

use super::{build_validator, BuildValidator, CombinedValidator, Extra, SlotsBuilder, ValResult, Validator};

#[derive(Debug, Clone)]
pub struct OptionalValidator {
    validator: Box<CombinedValidator>,
}

impl BuildValidator for OptionalValidator {
    const EXPECTED_TYPE: &'static str = "optional";

    fn build(
        schema: &PyDict,
        config: Option<&PyDict>,
        slots_builder: &mut SlotsBuilder,
    ) -> PyResult<CombinedValidator> {
        let schema: &PyAny = schema.get_as_req("schema")?;
        Ok(Self {
            validator: Box::new(build_validator(schema, config, slots_builder)?.0),
        }
        .into())
    }
}

impl Validator for OptionalValidator {
    fn validate<'s, 'data>(
        &'s self,
        py: Python<'data>,
        input: CombinedInput<'data>,
        extra: &Extra,
        slots: &'data [CombinedValidator],
    ) -> ValResult<'data, PyObject> {
        match input.is_none() {
            true => Ok(py.None()),
            false => self.validator.validate(py, input, extra, slots),
        }
    }

    fn validate_strict<'s, 'data>(
        &'s self,
        py: Python<'data>,
        input: CombinedInput<'data>,
        extra: &Extra,
        slots: &'data [CombinedValidator],
    ) -> ValResult<'data, PyObject> {
        match input.is_none() {
            true => Ok(py.None()),
            false => self.validator.validate_strict(py, input, extra, slots),
        }
    }

    fn get_name(&self, _py: Python) -> String {
        Self::EXPECTED_TYPE.to_string()
    }
}
