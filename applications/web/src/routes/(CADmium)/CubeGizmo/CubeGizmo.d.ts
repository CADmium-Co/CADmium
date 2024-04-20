import type { Key, ThrelteUseTaskOptions } from '@threlte/core'
import { SvelteComponent } from 'svelte'
import type { ColorRepresentation } from 'three'

type TaskOptions = Pick<ThrelteUseTaskOptions, 'after' | 'before' | 'stage'> & { key?: Key }

export type CubeGizmoProps = {
  renderTask?: TaskOptions
  animationTask?: TaskOptions
  turnRate?: number
  center?: [number, number, number]
  verticalPlacement?: 'top' | 'bottom'
  horizontalPlacement?: 'left' | 'right'
  size?: number
  xColor?: ColorRepresentation
  yColor?: ColorRepresentation
  zColor?: ColorRepresentation
  toneMapped?: boolean
  paddingX?: number
  paddingY?: number
}

export type CubeGizmoEvents = Record<string, never>

export type CubeGizmoSlots = Record<string, never>

export default class CubeGizmo extends SvelteComponent<CubeGizmoProps, CubeGizmoEvents, CubeGizmoSlots> {}
