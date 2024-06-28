use thiserror::Error;

#[derive(Error, Debug)]
pub enum CADmiumError {
	// Message errors
	#[error("The project ID {0} was not found")]
	ProjectIDNotFound(usize),
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

	#[error(
		"Node {0} in evtree of project {1} workbench {2} is not a concrete LoroValue but a Handler"
	)]
	EvTreeNodeNotValue(usize, usize, usize),
	#[error("Node {0} in evtree of project {1} workbench {2} is not a map")]
	EvTreeNodeNotMap(usize, usize, usize),
	#[error("The `prev` map key in node {0} in evtree of project {1} workbench {2} was not found")]
	EvTreeNodePrevNotFound(usize, usize, usize),
	#[error("The `this` map key in node {0} in evtree of project {1} workbench {2} was not found")]
	EvTreeNodeThisNotFound(usize, usize, usize),
	#[error("The `hash` meta key in node {0} was not found")]
	EvTreeHashNotFound(i32),
	#[error("The `hash` meta key in node {0} is a container and not a value")]
	EvTreeHashIsContainer(i32),
	#[error("The `hash` meta key is not an I64")]
	EvTreeHashNotI64,

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

	#[error(transparent)]
	Other(#[from] anyhow::Error),
}
