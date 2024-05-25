import type { SvelteComponent } from "svelte";
import type { Vector2, Vector3 } from "three";

export { default as Arc } from "./Arc.svelte";
export { default as Circle } from "./Circle.svelte";
export { default as Line } from "./Line.svelte";
export { default as Rectangle } from "./Rectangle.svelte";
export { default as Select } from "./Select.svelte";

export interface ToolComponentType extends SvelteComponent {
	click: (event: Event, data: { twoD: Vector2; threeD: Vector3 }) => void;
	mouseMove: (event: Event, data: Vector2) => void;
	cancel: () => void;
}
