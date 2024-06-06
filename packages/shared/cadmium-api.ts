import { Direction, IDType, MessageResult, Mode, Plane, PlaneDescription } from "cadmium";
import { sendWasmMessage } from "./projectUtils";

interface ProjectRename { new_name: string };
export function projectRename(new_name: string): MessageResult {
	const message: Message = { ProjectRename: { new_name } }
	return sendWasmMessage(message)
}
interface WorkbenchRename { workbench_id: IDType, new_name: string };
export function workbenchRename(workbench_id: IDType, new_name: string): MessageResult {
	const message: Message = { WorkbenchRename: { workbench_id, new_name } }
	return sendWasmMessage(message)
}
interface WorkbenchPointAdd { workbench_id: IDType, x: number, y: number, z: number };
export function workbenchPointAdd(workbench_id: IDType, x: number, y: number, z: number): MessageResult {
	const message: Message = { WorkbenchPointAdd: { workbench_id, x, y, z } }
	return sendWasmMessage(message)
}
interface WorkbenchPlaneAdd { workbench_id: IDType, plane: Plane, width: number, height: number };
export function workbenchPlaneAdd(workbench_id: IDType, plane: Plane, width: number, height: number): MessageResult {
	const message: Message = { WorkbenchPlaneAdd: { workbench_id, plane, width, height } }
	return sendWasmMessage(message)
}
interface WorkbenchSketchAdd { workbench_id: IDType, plane_description: PlaneDescription };
export function workbenchSketchAdd(workbench_id: IDType, plane_description: PlaneDescription): MessageResult {
	const message: Message = { WorkbenchSketchAdd: { workbench_id, plane_description } }
	return sendWasmMessage(message)
}
interface WorkbenchSketchSetPlane { workbench_id: IDType, sketch_id: IDType, plane_description: PlaneDescription };
export function WorkbenchSketchSetPlane(workbench_id: IDType, sketch_id: IDType, plane_description: PlaneDescription): MessageResult {
	const message: Message = { WorkbenchSketchSetPlane: { workbench_id, sketch_id, plane_description } }
	return sendWasmMessage(message)
}
interface WorkbenchPointUpdate { workbench_id: IDType, point_id: IDType, x: number, y: number, z: number };
export function workbenchPointUpdate(workbench_id: IDType, point_id: IDType, x: number, y: number, z: number): MessageResult {
	const message: Message = { WorkbenchPointUpdate: { workbench_id, point_id, x, y, z } }
	return sendWasmMessage(message)
}
interface SketchAddPoint { workbench_id: IDType, sketch_id: IDType, x: number, y: number, z: number };
export function sketchAddPoint(workbench_id: IDType, sketch_id: IDType, x: number, y: number, z: number): MessageResult {
	const message: Message = { SketchAddPoint: { workbench_id, sketch_id, x, y, z } }
	return sendWasmMessage(message)
}
interface SketchAddArc { workbench_id: IDType, sketch_id: IDType, center: IDType, radius: number, clockwise: boolean, start_angle: number, end_angle: number };
export function sketchAddArc(workbench_id: IDType, sketch_id: IDType, center: IDType, radius: number, clockwise: boolean, start_angle: number, end_angle: number): MessageResult {
	const message: Message = { SketchAddArc: { workbench_id, sketch_id, center, radius, clockwise, start_angle, end_angle } }
	return sendWasmMessage(message)
}
interface SketchAddCircle { workbench_id: IDType, sketch_id: IDType, center: IDType, radius: number };
export function sketchAddCircle(workbench_id: IDType, sketch_id: IDType, center: IDType, radius: number): MessageResult {
	const message: Message = { SketchAddCircle: { workbench_id, sketch_id, center, radius } }
	return sendWasmMessage(message)
}
interface SketchAddLine { workbench_id: IDType, sketch_id: IDType, start: IDType, end: IDType };
export function sketchAddLine(workbench_id: IDType, sketch_id: IDType, start: IDType, end: IDType): MessageResult {
	const message: Message = { SketchAddLine: { workbench_id, sketch_id, start, end } }
	return sendWasmMessage(message)
}
interface SketchDeletePrimitive { workbench_id: IDType, sketch_id: IDType, primitive_id: IDType };
export function sketchDeletePrimitive(workbench_id: IDType, sketch_id: IDType, primitive_id: IDType): MessageResult {
	const message: Message = { SketchDeletePrimitive: { workbench_id, sketch_id, primitive_id } }
	return sendWasmMessage(message)
}
interface SolidExtrusionAdd { workbench_id: IDType, sketch_id: IDType, faces: IDType[], length: number, offset: number, direction: Direction, mode: Mode };
export function solidExtrusionAdd(workbench_id: IDType, sketch_id: IDType, faces: IDType[], length: number, offset: number, direction: Direction, mode: Mode): MessageResult {
	const message: Message = { SolidExtrusionAdd: { workbench_id, sketch_id, faces, length, offset, direction, mode } }
	return sendWasmMessage(message)
}
interface SolidExtrusionUpdateFaces { workbench_id: IDType, extrusion_id: IDType, sketch_id: IDType, faces: IDType[] };
export function solidExtrusionUpdateFaces(workbench_id: IDType, extrusion_id: IDType, sketch_id: IDType, faces: IDType[]): MessageResult {
	const message: Message = { SolidExtrusionUpdateFaces: { workbench_id, extrusion_id, sketch_id, faces } }
	return sendWasmMessage(message)
}
interface StepRename { workbench_id: IDType, step_id: IDType, new_name: string };
export function stepRename(workbench_id: IDType, step_id: IDType, new_name: string): MessageResult {
	const message: Message = { StepRename: { workbench_id, step_id, new_name } }
	return sendWasmMessage(message)
}
interface StepDelete { workbench_id: IDType, step_id: IDType };
export function stepDelete(workbench_id: IDType, step_id: IDType): MessageResult {
	const message: Message = { StepDelete: { workbench_id, step_id } }
	return sendWasmMessage(message)
}

export type Message =
	{ ProjectRename: ProjectRename } |
	{ WorkbenchRename: WorkbenchRename } |
	{ WorkbenchPointAdd: WorkbenchPointAdd } |
	{ WorkbenchPlaneAdd: WorkbenchPlaneAdd } |
	{ WorkbenchSketchAdd: WorkbenchSketchAdd } |
	{ WorkbenchSketchSetPlane: WorkbenchSketchSetPlane } |
	{ WorkbenchPointUpdate: WorkbenchPointUpdate } |
	{ SketchAddPoint: SketchAddPoint } |
	{ SketchAddArc: SketchAddArc } |
	{ SketchAddCircle: SketchAddCircle } |
	{ SketchAddLine: SketchAddLine } |
	{ SketchDeletePrimitive: SketchDeletePrimitive } |
	{ SolidExtrusionAdd: SolidExtrusionAdd } |
	{ SolidExtrusionUpdateFaces: SolidExtrusionUpdateFaces } |
	{ StepRename: StepRename } |
	{ StepDelete: StepDelete }
