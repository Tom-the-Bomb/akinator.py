use pyo3::prelude::*;
use akinator_rs::models::Guess as GuessModel;


/// a model class representing an akinator's guess
/// not meant for the user to construct, but is returned in various properties and methods in the :class:`Akinator` class
#[pyclass]
#[derive(Debug, Clone)]
pub struct Guess(
    pub GuessModel,
);

#[pymethods]
impl Guess {
    fn __repr__(&self) -> String {
        format!(
            "<Guess id=\"{}\" name=\"{}\" ranking={}>",
            self.id(),
            self.name(),
            self.ranking(),
        )
    }

    /// :class:`str`: the unique ID of the specific guess's entity
    #[getter]
    const fn id(&self) -> &String {
        &self.0.id
    }

    /// :class:`str`: the common name of the specific guess's entity
    #[getter]
    const fn name(&self) -> &String {
        &self.0.name
    }

    /// :class:`str`: award id
    #[getter]
    const fn award_id(&self) -> &String {
        &self.0.award_id
    }

    /// :class:`int`: flag photo
    #[getter]
    const fn flag_photo(&self) -> usize {
        self.0.flag_photo
    }

    /// :class:`float`: the accuracy / confidence of the akinator that this guess is correct
    #[getter]
    fn confidence(&self) -> PyResult<f32> {
        let conf = self.0.confidence
            .parse::<f32>()?;

        Ok(conf)
    }

    /// :class:`str`: a brief description of the specific guess's entity
    #[getter]
    const fn description(&self) -> &String {
        &self.0.description
    }

    /// :class:`str`: the rank of the specific guess's entity
    #[getter]
    const fn ranking(&self) -> &String {
        &self.0.ranking
    }

    /// :class:`str`: a relative path to a picture of the guess's entity
    #[getter]
    const fn picture_path(&self) -> &String {
        &self.0.picture_path
    }

    /// :class:`str`: an absolute url to the picture of the guess's entity
    #[getter]
    const fn absolute_picture_path(&self) -> &String {
        &self.0.absolute_picture_path
    }
}