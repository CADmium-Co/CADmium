import { newExtrusion, newSketchOnPlane } from "shared/projectUtils";

export { default as Extrusion } from "./Extrusion.svelte";
export { default as Plane } from "./Plane.svelte";
export { default as Point } from "./Point.svelte";
export { default as Sketch } from "./Sketch.svelte";

export const createFeatureList = [
	{ name: "Extrusion", new: newExtrusion },
	{ name: "Sketch", new: newSketchOnPlane },
]
