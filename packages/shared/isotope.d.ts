import { IDType } from "cadmium"

export interface Sketch {
	primitives: SketchPrimitive[],
	primitive_next_id: number,
	constraints: SketchConstraint[],
}

export type SketchPrimitive = { Point2: Point2 } | { Line: Line } | { Circle: Circle } | { Arc: Arc }

export interface Point2 {
	x: number
	y: number
}

export interface Line {
	start: IDType
	end: IDType
}

export interface Circle {
	center: IDType
	radius: number
}

export interface Arc {
	center: IDType
	radius: number
	start_angle: number
	end_angle: number
	clockwise: boolean
}

// TODO: Constraints
export type SketchConstraint = {};
