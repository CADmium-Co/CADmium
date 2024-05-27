import {SvelteComponent} from "svelte"
import type {CubeGizmoProps} from "./CubeGizmo"
declare const __propDef: {
  props: CubeGizmoProps
}
type CubeGizmoProps_ = typeof __propDef.props
export type {CubeGizmoProps_ as CubeGizmoProps}
export default class CubeGizmo extends SvelteComponent<CubeGizmoProps> {}
