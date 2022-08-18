use pyo3::prelude::*;
use akinator_rs::models::Guess as GuessModel;


#[pyclass]
#[derive(Debug, Clone)]
pub struct Guess(
    pub GuessModel,
);

#[pymethods]
impl Guess {

    fn __repr__(&self) -> String {
        format!(
            "<Guess id={} name={} ranking={}>",
            self.id(),
            self.name(),
            self.ranking(),
        )
    }

    #[getter]
    const fn id(&self) -> &String {
        &self.0.id
    }

    #[getter]
    const fn name(&self) -> &String {
        &self.0.name
    }

    #[getter]
    const fn award_id(&self) -> &String {
        &self.0.award_id
    }

    #[getter]
    const fn flag_photo(&self) -> usize {
        self.0.flag_photo
    }

    #[getter]
    fn confidence(&self) -> PyResult<f32> {
        let conf = self.0.confidence
            .parse::<f32>()?;

        Ok(conf)
    }

    #[getter]
    const fn description(&self) -> &String {
        &self.0.description
    }

    #[getter]
    const fn ranking(&self) -> &String {
        &self.0.ranking
    }

    #[getter]
    const fn picture_path(&self) -> &String {
        &self.0.picture_path
    }

    #[getter]
    const fn absolute_picture_path(&self) -> &String {
        &self.0.absolute_picture_path
    }
}