<script lang="ts">
	import { sketchTool } from "shared/stores"
	import * as AllTools from "./";
	import type { IDictionary, PointsLikeById, ProjectToPlane, SketchPoint, ToolType } from "shared/types"
	import type { Vector2, Vector3 } from "three"

	// prettier-ignore
	const log = (function () { const context = "[ToolInstance.svelte]"; const color="aliceblue"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})()

	export let pointsById: PointsLikeById & IDictionary<SketchPoint>, sketchIndex: string, projectToPlane: ProjectToPlane

	interface ToolComponentInstance {
		name: string
		// component: (new (...args: any[]) => AllTools.ToolComponentType)
		component: AllTools.ToolComponentType
	}

	const instances: ToolComponentInstance[] = Object
		.entries(AllTools)
		.map(([name, component]) => ({
			name,
			component: new component({
				// Is document.body the right target?
				target: document.body,
				props: {
					pointsById,
					sketchIndex,
					active: $sketchTool === name,
					projectToPlane
				}
			})
		}))

	export function meshMouseMove(event: Event, data: Vector2) {
		const inst = instances.find(i => i.name === $sketchTool)
		inst !== undefined && inst.component.mouseMove(event, data)
	}

	export function meshClick(event: Event, data: { twoD: Vector2; threeD: Vector3 }) {
		const inst = instances.find(i => i.name === $sketchTool)
		inst !== undefined && inst.component.click(event, data)
	}
</script>
