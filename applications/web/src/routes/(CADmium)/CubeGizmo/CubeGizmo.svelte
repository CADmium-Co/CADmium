<script lang="ts">
	import { HierarchicalObject, T, useTask, useThrelte } from "@threlte/core"
	import { onDestroy, onMount } from "svelte"
	import {
		CanvasTexture,
		CapsuleGeometry,
		Color,
		Mesh,
		OrthographicCamera,
		Quaternion,
		Raycaster,
		Scene,
		Sprite,
		Triangle,
		Vector2,
		Vector3,
		Vector4,
		type ColorRepresentation
	} from "three"

	import type { SetCameraFocus } from "shared/types"
	import type { CubeGizmoProps } from "./CubeGizmo"

	type $$Props = CubeGizmoProps

	export let renderTask: $$Props["renderTask"] = undefined
	export let animationTask: $$Props["animationTask"] = undefined
	export let setCameraFocus: SetCameraFocus

	export let verticalPlacement: Required<$$Props>["verticalPlacement"] = "bottom"
	export let horizontalPlacement: Required<$$Props>["horizontalPlacement"] = "right"
	export let size: Required<$$Props>["size"] = 128
	export let xColor: Required<$$Props>["xColor"] = 0xff0000 // red
	export let yColor: Required<$$Props>["yColor"] = 0x179316 // green
	export let zColor: Required<$$Props>["zColor"] = 0x0000ff // blue
	export let toneMapped: Required<$$Props>["toneMapped"] = false
	export let paddingX: Required<$$Props>["paddingX"] = 0
	export let paddingY: Required<$$Props>["paddingY"] = 0

	const origin = new Vector3(0, 0, 0)
	const textureSize = 64
	const gray = 0xdde6ed
	const black = 0x000000

	const { autoRenderTask, renderer, camera, invalidate } = useThrelte()

	// invalidate the frame when any of the following values change
	$: size, horizontalPlacement, verticalPlacement, toneMapped, paddingX, paddingY, invalidate()

	const orthoCam = new OrthographicCamera(-2, 2, 2, -2, 0, 4)
	orthoCam.position.set(0, 0, 2)

	const rotationRoot = new Scene()
	const triangleControls = new Scene()

	const viewport = new Vector4()

	// Position and render the gizmo in the parent element.
	useTask(
		renderTask?.key ?? Symbol("cube-gizmo-render"),
		() => {
			const autoClear = renderer.autoClear
			renderer.autoClear = false
			renderer.getViewport(viewport)
			const toneMapping = renderer.toneMapping
			renderer.toneMapping = toneMapped ? renderer.toneMapping : 0

			const x = horizontalPlacement === "left" ? paddingX : renderer.domElement.offsetWidth - size - paddingX
			const y = verticalPlacement === "bottom" ? paddingY : renderer.domElement.offsetHeight - size - paddingY

			renderer.setViewport(x, y, size, size)
			renderer.render(rotationRoot, orthoCam)
			renderer.render(triangleControls, orthoCam)
			renderer.setViewport(viewport.x, viewport.y, viewport.z, viewport.w)
			renderer.autoClear = autoClear
			renderer.toneMapping = toneMapping
		},
		{
			...(renderTask ?? { after: autoRenderTask }),
			autoInvalidate: false
		}
	)

	// User interaction must be handled manually because
	// the gizmo is not in the main scene. The click
	// target is added as a sibling of the renderer's
	// dom element.
	const clickTarget = document.createElement("div")
	// We need to know the bounding rect of the renderer's dom element
	const renderTarget = renderer.domElement
	const boundingRect = renderTarget.getBoundingClientRect()

	clickTarget.style.position = "absolute"
	$: {
		if (horizontalPlacement === "right") {
			clickTarget.style.right = `${paddingX}px`
			clickTarget.style.left = ""
		} else {
			clickTarget.style.right = ""
			clickTarget.style.left = `${paddingX + boundingRect.left}px`
		}

		if (verticalPlacement === "bottom") {
			clickTarget.style.bottom = ""
			clickTarget.style.top = `${boundingRect.bottom - size - paddingY}px`
		} else {
			clickTarget.style.bottom = ""
			clickTarget.style.top = `${paddingY + boundingRect.top}px`
		}

		clickTarget.style.height = `${size}px`
		clickTarget.style.width = `${size}px`
	}

	let xAxisLabel: Sprite
	let yAxisLabel: Sprite
	let zAxisLabel: Sprite
	let cube: Mesh
	let downTriangle: Sprite
	let leftTriangle: Sprite
	let upTriangle: Sprite
	let rightTriangle: Sprite

	const mouse = new Vector2()
	const raycaster = new Raycaster()

	/**
	 * @returns boolean indicating if value is effectively 1.
	 */
	const approachesOne = (num: number) => 0.9999 < num && num < 1.0001

	const handleClick = (event: MouseEvent) => {
		// Raycasting is done manually.
		const rect = clickTarget.getBoundingClientRect()
		const offsetX = rect.left + (clickTarget.offsetWidth - size)
		const offsetY = rect.top + (clickTarget.offsetHeight - size)
		mouse.x = ((event.clientX - offsetX) / (rect.right - offsetX)) * 2 - 1
		mouse.y = -((event.clientY - offsetY) / (rect.bottom - offsetY)) * 2 + 1

		raycaster.setFromCamera(mouse, orthoCam)

		const cubeIntersects = raycaster.intersectObject(cube)
		if (cubeIntersects.length) {
			const faceIndex = cubeIntersects[0].faceIndex

			// Each cube face consists of 2 faceIndexes
			// TODO: add slerp
			switch (faceIndex) {
				// Right
				case 0:
				case 1:
					setCameraFocus({ x: 1, y: 0, z: 0 }, origin, { x: 0, y: 0, z: 1 })
					break
				// Left
				case 2:
				case 3:
					setCameraFocus({ x: -1, y: 0, z: 0 }, origin, { x: 0, y: 0, z: 1 })
					break
				// Back
				case 4:
				case 5:
					setCameraFocus({ x: 0, y: 1, z: 0 }, origin, { x: 0, y: 0, z: 1 })
					break
				// Front
				case 6:
				case 7:
					setCameraFocus({ x: 0, y: -1, z: 0 }, origin, { x: 0, y: 0, z: 1 })
					break
				// Top
				case 8:
				case 9:
					setCameraFocus({ x: 0, y: 0, z: 1 }, origin, { x: 0, y: 1, z: 0 })
					break
				// Bottom
				case 10:
				case 11:
					setCameraFocus({ x: 0, y: 0, z: -1 }, origin, { x: 0, y: -1, z: 0 })
					break
			}
		}

		const triangleIntersects = raycaster.intersectObjects([downTriangle, leftTriangle, upTriangle, rightTriangle])
		if (triangleIntersects.length) {
			const cameraUp = camera.current.up.clone()
			const quaternion = new Quaternion()

			const triangleSprite = triangleIntersects[0].object
			switch (triangleSprite) {
				case downTriangle:
				case upTriangle:
					const angle = triangleSprite == downTriangle ? -Math.PI / 12 : Math.PI / 12 // 15 deg

					// Rotate camera to new position.
					const planeTriangle = new Triangle(origin, cameraUp, camera.current.position)
					const upDownRotationAxis = planeTriangle.getNormal(origin)
					quaternion.setFromAxisAngle(upDownRotationAxis, angle)
					camera.current.position.applyQuaternion(quaternion)

					// Set camera up vector (prevents orientation inversion).
					// Crossing the vectors follows left-hand rule for calculating the perpendicular up vector.
					const normalizedCameraPos = camera.current.position.clone().normalize()
					const newUp = normalizedCameraPos.cross(upDownRotationAxis)
					camera.current.up.set(newUp.x, newUp.y, newUp.z)
					break
				case leftTriangle:
					quaternion.setFromAxisAngle(cameraUp, Math.PI / 12)
					camera.current.position.applyQuaternion(quaternion)
					break
				case rightTriangle:
					quaternion.setFromAxisAngle(cameraUp, -Math.PI / 12)
					camera.current.position.applyQuaternion(quaternion)
					break
			}
		}
	}

	onMount(() => {
		renderer.domElement.parentElement?.appendChild(clickTarget)
		clickTarget.addEventListener("click", handleClick)
	})

	onDestroy(() => {
		renderer.domElement.parentElement?.removeChild(clickTarget)
		clickTarget.removeEventListener("click", handleClick)
	})

	// Rotate the gizmo as the camera moves.
	const point = new Vector3()
	let p = [0, 0, 0]
	useTask(
		animationTask?.key ?? Symbol("cube-gizmo-animation"),
		() => {
			point.set(0, 0, 1).applyQuaternion(camera.current.quaternion)
			if (point.x !== p[0] || point.y !== p[1] || point.z !== p[2]) {
				p = [point.x, point.y, point.z]
				rotationRoot.quaternion.copy(camera.current.quaternion).invert()
				invalidate()
			}
		},
		{
			...animationTask,
			autoInvalidate: false
		}
	)

	/**
	 * Keep track of the textures to be able to dispose them when they are no
	 * longer needed.
	 */
	const textures: Record<string, CanvasTexture> = {}

	const color = new Color()
	const getAxisLabelSpriteTexture = (size: number, colorRepresentation: ColorRepresentation, text = "") => {
		color.set(colorRepresentation)
		const key = `${color.getHexString()}-${text}`
		if (textures[key]) {
			textures[key].dispose()
		}
		const canvas = document.createElement("canvas")
		canvas.width = size
		canvas.height = size

		const context = canvas.getContext("2d")!

		if (text) {
			const textSize = Math.abs(size * (22 / 64))
			context.font = `${textSize}px Arial`
			context.textAlign = "center"
			context.fillStyle = color.convertSRGBToLinear().getStyle()
			const textY = size * (41 / 64)
			context.fillText(text, size / 2, textY)
		}

		const texture = new CanvasTexture(canvas)
		textures[key] = texture
		return texture
	}

	const getCubeSpriteTexture = (size: number, text = "") => {
		const key = `cube-${text}`
		if (textures[key]) {
			textures[key].dispose()
		}
		const canvas = document.createElement("canvas")
		canvas.width = size
		canvas.height = size

		const context = canvas.getContext("2d")!

		const backgroundColor = new Color(gray)
		context.fillStyle = backgroundColor.convertSRGBToLinear().getStyle()
		context.fillRect(0, 0, canvas.width, canvas.height)

		if (text) {
			const textSize = Math.abs(size * (16 / 64))
			context.font = `${textSize}px Arial`
			context.textAlign = "center"
			const textColor = new Color(black)
			context.fillStyle = textColor.convertSRGBToLinear().getStyle()
			const textXPos = size / 2
			const textYPos = size * (41 / 64)
			context.fillText(text, textXPos, textYPos)
		}

		const texture = new CanvasTexture(canvas)
		texture.generateMipmaps = false // makes text sharper

		// Rotate text.
		texture.center = new Vector2(0.5, 0.5)
		switch (text) {
			case "Right":
				texture.rotation = Math.PI / 2 // 90 deg
				break
			case "Back":
			case "Bottom":
				texture.rotation = Math.PI
				break
			case "Left":
				texture.rotation = (3 * Math.PI) / 2
				break
			// other faces don't need to be rotated.
		}

		textures[key] = texture
		return texture
	}

	const getTriangleSpriteTexture = (label = "") => {
		const key = `triangle-${label}`
		if (textures[key]) {
			textures[key].dispose()
		}

		const canvas = document.createElement("canvas")
		canvas.width = 100
		canvas.height = 73

		const context = canvas.getContext("2d")!
		const v1 = new Vector2(0, 0)
		const v2 = new Vector2(50, 72.1)
		const v3 = new Vector2(100, 0)
		context.beginPath()
		context.moveTo(v1.x, v1.y)
		context.lineTo(v2.x, v2.y)
		context.lineTo(v3.x, v3.y)

		const backgroundColor = new Color(gray)
		context.fillStyle = backgroundColor.convertSRGBToLinear().getStyle()
		context.fill()

		const texture = new CanvasTexture(canvas)
		textures[key] = texture
		return texture
	}

	const axisLine = new CapsuleGeometry(0.02, 1.5)
	axisLine.rotateZ(Math.PI / 2)
</script>

<HierarchicalObject>
	<T is={rotationRoot}>
		<T.Mesh bind:ref={cube}>
			<T.BoxGeometry args={[1.5, 1.5, 1.5]} />
			<T.MeshBasicMaterial
				map={getCubeSpriteTexture(textureSize, "Right")}
				attach={(parent, self) => {
					if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
					else parent.material = [self]
				}}
			/>
			<T.MeshBasicMaterial
				map={getCubeSpriteTexture(textureSize, "Left")}
				attach={(parent, self) => {
					if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
					else parent.material = [self]
				}}
			/>
			<T.MeshBasicMaterial
				map={getCubeSpriteTexture(textureSize, "Back")}
				attach={(parent, self) => {
					if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
					else parent.material = [self]
				}}
			/>
			<T.MeshBasicMaterial
				map={getCubeSpriteTexture(textureSize, "Front")}
				attach={(parent, self) => {
					if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
					else parent.material = [self]
				}}
			/>
			<T.MeshBasicMaterial
				map={getCubeSpriteTexture(textureSize, "Top")}
				attach={(parent, self) => {
					if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
					else parent.material = [self]
				}}
			/>
			<T.MeshBasicMaterial
				map={getCubeSpriteTexture(textureSize, "Bottom")}
				attach={(parent, self) => {
					if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
					else parent.material = [self]
				}}
			/>
		</T.Mesh>

		<T.Sprite bind:ref={xAxisLabel} position={[1, -0.75, -0.75]}>
			<!-- hide the text 'X' when xAxisLabel is orthogonal to viewport -->
			<T.SpriteMaterial
				map={getAxisLabelSpriteTexture(textureSize, xColor, "X")}
				opacity={approachesOne(-1 * p[0]) || approachesOne(p[0]) ? 0 : 1}
			/>
		</T.Sprite>
		<T.Mesh position={[0, -0.75, -0.75]}>
			<T is={axisLine} />
			<T.MeshBasicMaterial transparent color={xColor} />
		</T.Mesh>

		<T.Sprite bind:ref={yAxisLabel} position={[-0.75, 1, -0.75]}>
			<!-- hide the text 'Y' when yAxisLabel is orthogonal to viewport -->
			<T.SpriteMaterial
				map={getAxisLabelSpriteTexture(textureSize, yColor, "Y")}
				opacity={approachesOne(-1 * p[1]) || approachesOne(p[1]) ? 0 : 1}
			/>
		</T.Sprite>
		<T.Mesh position={[-0.75, 0, -0.75]} rotation.z={Math.PI / 2}>
			<T is={axisLine} />
			<T.MeshBasicMaterial transparent color={yColor} />
		</T.Mesh>

		<T.Sprite bind:ref={zAxisLabel} position={[-0.75, -0.75, 1]}>
			<!-- hide the text 'Z' when zAxisLabel is orthogonal to viewport -->
			<T.SpriteMaterial
				map={getAxisLabelSpriteTexture(textureSize, zColor, "Z")}
				opacity={approachesOne(-1 * p[2]) || approachesOne(p[2]) ? 0 : 1}
			/>
		</T.Sprite>
		<T.Mesh position={[-0.75, -0.75, 0]} rotation.y={-Math.PI / 2}>
			<T is={axisLine} />
			<T.MeshBasicMaterial transparent color={zColor} />
		</T.Mesh>
	</T>

	<T is={triangleControls}>
		<T.Sprite bind:ref={downTriangle} position={[0, -1.85, 1]} scale={0.4}>
			<T.SpriteMaterial map={getTriangleSpriteTexture("down")} />
		</T.Sprite>
		<T.Sprite bind:ref={leftTriangle} position={[-1.85, 0, 1]} scale={0.4}>
			<T.SpriteMaterial map={getTriangleSpriteTexture("left")} rotation={(3 * Math.PI) / 2} />
		</T.Sprite>
		<T.Sprite bind:ref={upTriangle} position={[0, 1.85, 1]} scale={0.4}>
			<T.SpriteMaterial map={getTriangleSpriteTexture("up")} rotation={Math.PI} />
		</T.Sprite>
		<T.Sprite bind:ref={rightTriangle} position={[1.85, 0, 1]} scale={0.4}>
			<T.SpriteMaterial map={getTriangleSpriteTexture("right")} rotation={Math.PI / 2} />
		</T.Sprite>
	</T>
</HierarchicalObject>
