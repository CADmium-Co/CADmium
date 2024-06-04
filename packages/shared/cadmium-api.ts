import { Direction, IDType, Message, MessageResult, Mode, Plane, PlaneDescription, ProjectRename } from "cadmium";
import { sendWasmMessage } from "./projectUtils";

export function projectRename(new_name: string): MessageResult {
	const message: Message = { ProjectRename: { new_name } }
	return sendWasmMessage(message)
}
export function workbenchRename(workbench_id: IDType, new_name: string): MessageResult {
	const message: Message = { WorkbenchRename: { id: workbench_id, inner: { new_name } } }
	return sendWasmMessage(message)
}
export function workbenchPointAdd(workbench_id: IDType, x: number, y: number, z: number): MessageResult {
	const message: Message = { WorkbenchPointAdd: { id: workbench_id, inner: { x, y, z } } }
	return sendWasmMessage(message)
}
export function workbenchPlaneAdd(workbench_id: IDType, plane: Plane, width: number, height: number): MessageResult {
	const message: Message = { WorkbenchPlaneAdd: { id: workbench_id, inner: { plane, width, height } } }
	return sendWasmMessage(message)
}
export function workbenchSketchAdd(workbench_id: IDType, plane_description: PlaneDescription): MessageResult {
	const message: Message = { WorkbenchSketchAdd: { id: workbench_id, inner: { plane_description } } }
	return sendWasmMessage(message)
}
export function workbenchPointUpdate(workbench_id: IDType, point_id: IDType, x: number, y: number, z: number): MessageResult {
	const message: Message = { WorkbenchPointUpdate: { id: workbench_id, inner: { id: point_id, inner: { x, y, z } } } }
	return sendWasmMessage(message)
}
export function sketchAddPoint(workbench_id: IDType, sketch_id: IDType, x: number, y: number, z: number): MessageResult {
	const message: Message = { SketchAddPoint: { id: workbench_id, inner: { id: sketch_id, inner: { x, y, z } } } }
	return sendWasmMessage(message)
}
export function sketchAddArc(workbench_id: IDType, sketch_id: IDType, center: IDType, radius: number, clockwise: boolean, start_angle: number, end_angle: number): MessageResult {
	const message: Message = { SketchAddArc: { id: workbench_id, inner: { id: sketch_id, inner: { center, radius, clockwise, start_angle, end_angle } } } }
	return sendWasmMessage(message)
}
export function sketchAddCircle(workbench_id: IDType, sketch_id: IDType, center: IDType, radius: number): MessageResult {
	const message: Message = { SketchAddCircle: { id: workbench_id, inner: { id: sketch_id, inner: { center, radius } } } }
	return sendWasmMessage(message)
}
export function sketchAddLine(workbench_id: IDType, sketch_id: IDType, start: IDType, end: IDType): MessageResult {
	const message: Message = { SketchAddLine: { id: workbench_id, inner: { id: sketch_id, inner: { start, end } } } }
	return sendWasmMessage(message)
}
export function sketchDeletePrimitive(workbench_id: IDType, sketch_id: IDType, primitive_id: IDType): MessageResult {
	const message: Message = { SketchDeletePrimitive: { id: workbench_id, inner: { id: sketch_id, inner: { id: primitive_id } } } }
	return sendWasmMessage(message)
}
export function solidExtrusionAdd(workbench_id: IDType, sketch_id: IDType, faces: Face[], length: number, offset: number, direction: Direction, mode: Mode): MessageResult {
	const message: Message = { SolidExtrusionAdd: { id: workbench_id, inner: { sketch_id, faces, length, offset, direction, mode } } }
	return sendWasmMessage(message)
}
