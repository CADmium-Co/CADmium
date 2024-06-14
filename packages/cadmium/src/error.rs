use thiserror::Error;

use crate::sketch::SketchFeatureType;

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

	// StepData errors
	#[error("The step {0} data type is not as expected")]
	IncorrectStepDataType(String),

	// Sketch errors
	#[error("The {0} with ID {1} already exists in the sketch")]
	SketchFeatureAlreadyExists(SketchFeatureType, u64),
	#[error("The {0} ID is too low for {1}")]
	SketchFeatureIDTooLow(SketchFeatureType, u64),
	#[error("The {0} with ID {1} has a start point that doesn't exist in the current sketch")]
	SketchFeatureMissingStart(SketchFeatureType, u64),
	#[error("The {0} with ID {1} has an end point that doesn't exist in the current sketch")]
	SketchFeatureMissingEnd(SketchFeatureType, u64),


	#[error("This function is not implemented yet")]
	NotImplemented,
}
