use thiserror::Error;

#[derive(Error, Debug)]
pub enum CADmiumError {
    // Message errors
    #[error("The workbench ID {0} was not found")]
    WorkbenchIDNotFound(u64),
    #[error("The workbench name {0} was not found")]
    WorkbenchNameNotFound(String),
    #[error("The step ID {0} was not found")]
    StepIDNotFound(String),
    #[error("The step name {0} was not found")]
    StepNameNotFound(String),
    #[error("The sketch ID {0} was not found")]
    SketchIDNotFound(u64),

    // RealSketch errors
    #[error("The primitive could not be found inside the sketch")]
    PrimitiveNotInSketch,
    #[error("Couldn't calculate the 3D position of the supplied point")]
    Point3DCalculationFailed,
    #[error("The calculated 3D point was not found in the sketch")]
    Point3DNotFound,

    // StepData errors
    #[error("The step {0} data type is not as expected")]
    IncorrectStepDataType(String),

    #[error("This function is not implemented yet")]
    NotImplemented,
}
