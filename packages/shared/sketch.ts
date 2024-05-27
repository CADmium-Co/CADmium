import { Message, Primitive } from "cadmium";
import { get } from "svelte/store";
import { workbenchIndex } from "./stores";
import { sendWasmMessage } from "./projectUtils";

export class ISketch {
	id: string;

	constructor(id: string) {
		this.id = id
	}

	addPrimitive(primitive: Primitive): number {
		const message: Message = {
			AddSketchPrimitive: {
				workbench_id: get(workbenchIndex),
				sketch_id: this.id,
				primitive
			}
		}

		const reply = sendWasmMessage(message)

		if (!reply.success)
			console.error("ERROR [projectUtils.ts addPrimitiveToSketch sendWasmMessage]", "message:", message, "reply:", reply)

		return JSON.parse(reply.data).id
	}

	deletePrimitives(ids: number[]) {
		const message: Message = {
			DeleteSketchPrimitives: {
				workbench_id: get(workbenchIndex),
				sketch_id: this.id,
				ids
			}
		}

		sendWasmMessage(message)
	}

	setPlane(plane_id: number) {
		const message: Message = {
			SetSketchPlane: {
				workbench_id: get(workbenchIndex),
				sketch_id: this.id,
				plane_id: `${plane_id}`
			}
		}

		sendWasmMessage(message)
	}

	addCircle(center: number, external: number): number {
		return this.addPrimitive({
			Circle.new()
		})
	}
}
