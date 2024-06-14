<script lang="ts">
  import {Matrix4, Euler, MeshStandardMaterial, DoubleSide, Vector2, Vector3, type Vector3Like} from "three"
  import {T, extend, useThrelte} from "@threlte/core"
  import {Text, Suspense} from "@threlte/extras"

  import {Line2} from "three/addons/lines/Line2.js"
  import {LineMaterial} from "three/addons/lines/LineMaterial.js"
  import {LineGeometry} from "three/addons/lines/LineGeometry.js"

  import {currentlySelected, currentlyMousedOver, selectingFor, selectionMin, selectionMax} from "shared/stores"
  import type {EntityType} from "shared/types"

  const log = (function () { const context = "[Plane.svelte]"; const color="gray"; return Function.prototype.bind.call(console.log, console, `%c${context}`, `font-weight:bold;color:${color};`)})() // prettier-ignore

  export let name: string, id: string, width: number, height: number, origin: Vector3Like, primary: Vector3Like, secondary: Vector3Like, tertiary: Vector3Like

  // log("[props]","name:",name,"id:",id,"width:",width,"height:",height,"origin:",origin,"primary:",primary,"secondary:",secondary,"tertiary:",tertiary)

  extend({Line2})

  const {size, dpr} = useThrelte()

  // Build some Three.js vectors from the props
  const origin_point = new Vector3(origin.x, origin.y, origin.z)
  const primaryV3 = new Vector3(primary.x, primary.y, primary.z)
  const secondaryV3 = new Vector3(secondary.x, secondary.y, secondary.z)
  const tertiaryV3 = new Vector3(tertiary.x, tertiary.y, tertiary.z)

  // Use those to make the rotation matrix and euler angles
  const rotationMatrix = new Matrix4()
  rotationMatrix.makeBasis(primaryV3, secondaryV3, tertiaryV3)
  const eulerAngles = new Euler(0, 0, 0, "XYZ")
  eulerAngles.setFromRotationMatrix(rotationMatrix, "XYZ")

  // Lastly, make the Plane Material
  const standardMaterial = new MeshStandardMaterial({
    color: "#525292",
    side: DoubleSide,
    metalness: 0.0,
    transparent: true,
    opacity: 0.05,
    depthWrite: false,
    depthTest: true,
    wireframe: false,
    polygonOffset: true,
    polygonOffsetFactor: -4,
  })

  const hoveredMaterial = new MeshStandardMaterial({
    color: "#525292",
    side: DoubleSide,
    metalness: 0.0,
    transparent: true,
    opacity: 0.15,
    depthWrite: false,
    depthTest: true,
    wireframe: false,
    polygonOffset: true,
    polygonOffsetFactor: -4,
  })

  // this is x, y, z for each of five points, making a closed rectangle
  const points = [-width / 2, -height / 2, 0, width / 2, -height / 2, 0, width / 2, height / 2, 0, -width / 2, height / 2, 0, -width / 2, -height / 2, 0]

  $: standardLineMaterial = new LineMaterial({
    color: "#42a7eb",
    linewidth: 2.0 * $dpr,
    depthTest: true,
    transparent: true,
    dashed: false,
    resolution: new Vector2($size.width * $dpr, $size.height * $dpr),
  })

  $: hoveredLineMaterial = new LineMaterial({
    color: "#fcba03",
    linewidth: 3.0 * $dpr,
    depthTest: true,
    transparent: true,
    dashed: false,
    resolution: new Vector2($size.width * $dpr, $size.height * $dpr),
  })

  const lineGeometry = new LineGeometry()
  lineGeometry.setPositions(points)

  const type: EntityType = "plane"

  let hovered = false
  let selected = false
  // currentlySelected.subscribe(() => {
  // 	// if (!id) return
  // 	// if (!$currentlySelected.length) return
  // 	selected = $currentlySelected.some((e) => e.id === id && e.type === type) ? true : false
  // 	log('recomputed whether plane', id, 'was selected: ', selected)
  // })
  $: selected = $currentlySelected.some(e => e.id === id && e.type === type) ? true : false

  $: if ($currentlyMousedOver.length === 0) hovered = false
</script>

<T.Group
  rotation.x={eulerAngles.x}
  rotation.y={eulerAngles.y}
  rotation.z={eulerAngles.z}
  position.x={origin_point.x}
  position.y={origin_point.y}
  position.z={origin_point.z}
  visible={!name.startsWith("derived_plane_for:")}
>
  <T.Mesh
    material={hovered ? hoveredMaterial : standardMaterial}
    on:pointerenter={e => {
      if ($selectingFor.includes(type)) {
        e.stopPropagation()
        hovered = true
        $currentlyMousedOver = [...$currentlyMousedOver, {type, id: id}]
      }
    }}
    on:pointerleave={() => {
      if ($selectingFor.includes(type)) {
        hovered = false
        $currentlyMousedOver = $currentlyMousedOver.filter(item => !(item.id === id && item.type === type))
      } else {
        hovered = false
      }
    }}
    on:click={e => {
      if ($selectingFor.includes(type)) {
        e.stopPropagation()
        if ($currentlySelected.some(e => e.id === id && e.type === type)) {
          if ($currentlySelected.length - 1 < $selectionMin) {
            // we can't deselect if doing so puts us below the minimum
            // number of selected entities
            return
          }

          $currentlySelected = $currentlySelected.filter(item => !(+item.id === +id && item.type === type))
        } else {
          // if selecting this entity puts us above the maximum
          // number of selected entities, boot the oldest one
          if ($currentlySelected.length + 1 > $selectionMax) $currentlySelected.shift()

          /**   cadmium wants a string for id whereas for most ids it wants number, u64 iirc 
								we should use number for all entity ids? seems cleaner to use one type. otherwise we could do:
					
					  		interface Entity {
									id: number | string
									type: EntityType
								}

								really it could be a uuid. certainly it shouldn't be the name of the plane because it
								shouldn't be adjustable by the user. it's data. perhaps:

								interface Entity {
									id: number
									name?: string
									type: EntityType
								}

								for now I'll leave id as number and tell typescript to ignore this line:

								interface Entity {
									id: number
									type: EntityType
								}

								edit: stick with strings in the ui - the dom only has strings and some of our ids are not numeric strings
								simply convert to number before sending to rust as required
					*/
          // @ts-ignore
          $currentlySelected = [...$currentlySelected, {type, id: id.toString()}]
        }
      }
    }}
  >
    <T.PlaneGeometry args={[width, height]} />
  </T.Mesh>

  <T.Line2
    geometry={lineGeometry}
    material={selected ? hoveredLineMaterial : hovered ? hoveredLineMaterial : standardLineMaterial}
    on:create={({ref}) => {
      ref.computeLineDistances()
    }}
  />

  <T.Group position.x={(-width / 2) * 0.99} position.y={(height / 2) * 0.99}>
    <Suspense>
      <Text text={name} color="#42a7eb" fontSize={5} anchorX="0%" anchorY="0%" />
    </Suspense>
  </T.Group>
</T.Group>
