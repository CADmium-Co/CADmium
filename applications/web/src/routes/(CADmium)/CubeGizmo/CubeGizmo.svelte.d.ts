import { SvelteComponent } from "svelte";
import type { CubeGizmoEvents, CubeGizmoProps, CubeGizmoSlots } from './CubeGizmo';
declare const __propDef: {
    props: CubeGizmoProps;
    slots: CubeGizmoSlots;
    events: CubeGizmoEvents;
};
type CubeGizmoProps_ = typeof __propDef.props;
export { CubeGizmoProps_ as CubeGizmoProps };
type CubeGizmoEvents_ = typeof __propDef.events;
export { CubeGizmoEvents_ as CubeGizmoEvents };
type CubeGizmoSlots_ = typeof __propDef.slots;
export { CubeGizmoSlots_ as CubeGizmoSlots };
export default class CubeGizmo extends SvelteComponent<CubeGizmoProps, CubeGizmoEvents, CubeGizmoSlots> {
}
