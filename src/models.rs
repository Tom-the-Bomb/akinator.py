use pyo3::prelude::*;
use akinator_rs::models::Guess as GuessModel;


#[pyclass]
#[derive(Debug, Clone)]
pub struct Guess(
    pub GuessModel,
);

#[pymethods]
impl Guess {
    #[getter]
    fn id(&self) -> &String {
        &self.0.id
    }

    #[getter]
    fn name(&self) -> &String {
        &self.0.name
    }

    #[getter]
    fn award_id(&self) -> &String {
        &self.0.award_id
    }

    #[getter]
    fn flag_photo(&self) -> usize {
        self.0.flag_photo
    }

    #[getter]
    fn confidence(&self) -> PyResult<f32> {
        let conf = self.0.confidence
            .parse::<f32>()?;

        Ok(conf)
    }

    #[getter]
    fn description(&self) -> &String {
        &self.0.description
    }

    #[getter]
    fn ranking(&self) -> &String {
        &self.0.ranking
    }

    #[getter]
    fn picture_path(&self) -> &String {
        &self.0.picture_path
    }

    #[getter]
    fn absolute_picture_path(&self) -> &String {
        &self.0.absolute_picture_path
    }
}