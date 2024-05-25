<script lang="ts">
	import { HierarchicalObject, T, useTask, useThrelte } from "@threlte/core"
	import { onDestroy, onMount } from "svelte"
	import {
		CanvasTexture,
		CapsuleGeometry,
		Color,
		Mesh,
		MeshBasicMaterial,
		Object3D,
		OrthographicCamera,
		Quaternion,
		Raycaster,
		Scene,
		Sprite,
		Triangle,
		Vector2,
		Vector3,
		Vector4,
		type ColorRepresentation,
		type Intersection,
		type Object3DEventMap
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
	const mouseoverColor = 0xb1daf2 // light blue
	const white = 0xffffff
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

	const orthoCam = new OrthographicCamera(-2.1, 2.1, 2.1, -2.1, 0, 4)
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
	let cubeRight: MeshBasicMaterial
	let cubeLeft: MeshBasicMaterial
	let cubeBack: MeshBasicMaterial
	let cubeFront: MeshBasicMaterial
	let cubeTop: MeshBasicMaterial
	let cubeBottom: MeshBasicMaterial
	let downTriangle: Sprite
	let leftTriangle: Sprite
	let upTriangle: Sprite
	let rightTriangle: Sprite
	let curvedArrowLeft: Sprite
	let curvedArrowRight: Sprite
	// Whether one of the navigation control sprites (triangles, curvedArrrow) is displaying the on hover color.
	let isHoverColorSet = false
	let navControlClicked = false

	const curvedArrowCanvasWidth = 200
	const curvedArrowCanvasHeight = 200

	const mouse = new Vector2()
	const raycaster = new Raycaster()

	/**
	 * @returns boolean indicating if value is effectively 1.
	 */
	const approachesOne = (num: number) => 0.9999 < num && num < 1.0001

	const setMouseRaycaster = (event: MouseEvent) => {
		// Raycasting is done manually.
		const rect = clickTarget.getBoundingClientRect()
		const offsetX = rect.left + (clickTarget.offsetWidth - size)
		const offsetY = rect.top + (clickTarget.offsetHeight - size)
		mouse.x = ((event.clientX - offsetX) / (rect.right - offsetX)) * 2 - 1
		mouse.y = -((event.clientY - offsetY) / (rect.bottom - offsetY)) * 2 + 1

		raycaster.setFromCamera(mouse, orthoCam)
	}

	const handleMousemove = (event: MouseEvent) => {
		setMouseRaycaster(event)

		const cubeIntersects = raycaster.intersectObject(cube)
		const triangleIntersects = raycaster.intersectObjects([downTriangle, leftTriangle, upTriangle, rightTriangle])
		const curvedArrowIntersects = raycaster.intersectObjects([curvedArrowLeft, curvedArrowRight])

		// No intersection with a navigation control.
		if (
			cubeIntersects.length == 0 &&
			triangleIntersects.length == 0 &&
			(curvedArrowIntersects.length == 0 ||
				(!isCurvedArrowLeftDwgIntersected(curvedArrowIntersects[0]) &&
					!isCurvedArrowRightDwgIntersected(curvedArrowIntersects[0])))
		) {
			if (isHoverColorSet) {
				restoreNonHoverColors()
				invalidate()
			}
			return
		}

		// Intersects with one of the navigation controls.
		restoreNonHoverColors()
		// Set hover color on whichever sprite the ray is interseting.
		if (cubeIntersects.length) {
			const faceIndex = cubeIntersects[0].faceIndex
			// Each cube face consists of 2 faceIndexes
			switch (faceIndex) {
				// Right
				case 0:
				case 1:
					cubeRight.color.setHex(mouseoverColor)
					break
				// Left
				case 2:
				case 3:
					cubeLeft.color.setHex(mouseoverColor)
					break
				// Back
				case 4:
				case 5:
					cubeBack.color.setHex(mouseoverColor)
					break
				// Front
				case 6:
				case 7:
					cubeFront.color.setHex(mouseoverColor)
					break
				// Top
				case 8:
				case 9:
					cubeTop.color.setHex(mouseoverColor)
					break
				// Bottom
				case 10:
				case 11:
					cubeBottom.color.setHex(mouseoverColor)
					break
			}
		} else if (triangleIntersects.length) {
			const triangleSprite = triangleIntersects[0].object
			switch (triangleSprite) {
				case downTriangle:
					downTriangle.material.color.setHex(mouseoverColor)
					break
				case upTriangle:
					upTriangle.material.color.setHex(mouseoverColor)
					break
				case leftTriangle:
					leftTriangle.material.color.setHex(mouseoverColor)
					break
				case rightTriangle:
					rightTriangle.material.color.setHex(mouseoverColor)
					break
			}
		} else if (curvedArrowIntersects.length && isCurvedArrowLeftDwgIntersected(curvedArrowIntersects[0])) {
			curvedArrowLeft.material.color.setHex(mouseoverColor)
		} else if (curvedArrowIntersects.length && isCurvedArrowRightDwgIntersected(curvedArrowIntersects[0])) {
			curvedArrowRight.material.color.setHex(mouseoverColor)
		}
		isHoverColorSet = true
		invalidate()
	}

	// Although we are already calling restoreNonHoverColors on a mousemove event outside of a sprite intersect, if a user
	// moves the mouse too quickly, they will move outside the clickTarget before mousemove registers, so we need to
	// capture that the sprite is no longer being hovered over from the mouseleave event as well.
	const handleMouseleave = (_: MouseEvent) => {
		if (isHoverColorSet) {
			restoreNonHoverColors()
			invalidate()
		}
	}

	// Clear out the light blue hover color from all navigation controls.
	const restoreNonHoverColors = () => {
		cubeRight.color.setHex(gray)
		cubeLeft.color.setHex(gray)
		cubeBack.color.setHex(gray)
		cubeFront.color.setHex(gray)
		cubeTop.color.setHex(gray)
		cubeBottom.color.setHex(gray)
		downTriangle.material.color.setHex(gray)
		upTriangle.material.color.setHex(gray)
		leftTriangle.material.color.setHex(gray)
		rightTriangle.material.color.setHex(gray)
		curvedArrowLeft.material.color.setHex(gray)
		curvedArrowRight.material.color.setHex(gray)
		isHoverColorSet = false
	}

	const handleClick = (event: MouseEvent) => {
		setMouseRaycaster(event)

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

		const curvedArrowIntersects = raycaster.intersectObjects([curvedArrowLeft, curvedArrowRight])
		// Intersects with one of the curvedArrow sprites.
		if (curvedArrowIntersects.length) {
			navControlClicked = true
			const quaternion = new Quaternion()
			const intoPageRotationAxis = camera.current.position.clone().normalize()

			if (isCurvedArrowLeftDwgIntersected(curvedArrowIntersects[0])) {
				quaternion.setFromAxisAngle(intoPageRotationAxis, -Math.PI / 12)
				camera.current.up.applyQuaternion(quaternion)
			} else if (isCurvedArrowRightDwgIntersected(curvedArrowIntersects[0])) {
				quaternion.setFromAxisAngle(intoPageRotationAxis, Math.PI / 12)
				camera.current.up.applyQuaternion(quaternion)
			}
		}
	}

	// Whether the mouse has intersected (hovered or clicked on) the canvas drawing of the curvedArrowLeft, not just the
	// sprite.
	const isCurvedArrowLeftDwgIntersected = (intersect: Intersection<Object3D<Object3DEventMap>>): boolean => {
		// If pixel on sprite is transparent (alpha == 0), the user clicked the area around the arrow and not on the
		// arrow itself. We don't want to capture the area around the arrow because it overlaps with other navigation
		// controls.
		if (
			intersect.uv == null ||
			intersect.object != curvedArrowLeft ||
			pixelAlphaFromUVForCurvedArrow(intersect.uv.x, intersect.uv.y, "left") == 0
		) {
			return false
		}
		return true
	}

	const isCurvedArrowRightDwgIntersected = (intersect: Intersection<Object3D<Object3DEventMap>>): boolean => {
		if (
			intersect.uv == null ||
			intersect.object != curvedArrowRight ||
			pixelAlphaFromUVForCurvedArrow(intersect.uv.x, intersect.uv.y, "right") == 0
		) {
			return false
		}
		return true
	}

	// Returns the alpha of the pixel at a given uv coordinate for the curvedArrow.
	const pixelAlphaFromUVForCurvedArrow = (uvX: number, uvY: number, label: string): number => {
		var curvedArrowImageData
		if (label == "right") {
			curvedArrowImageData = curvedArrowRightImageData
		} else {
			curvedArrowImageData = curvedArrowLeftImageData
		}

		const x = Math.round(uvX * curvedArrowCanvasWidth)
		const y = curvedArrowCanvasHeight - Math.round(uvY * curvedArrowCanvasWidth)

		// https://stackoverflow.com/questions/45963306/html5-canvas-how-to-get-adjacent-pixels-position-from-the-linearized-imagedata/45969661#45969661
		// Read from stored curvedArrow imageData
		// Each pixel is 4 values (r,g, b, alpha)
		const pixelIndex = (x + y * curvedArrowCanvasWidth) * 4
		const pixelAlpha = curvedArrowImageData.data[pixelIndex + 3]

		return pixelAlpha
	}

	onMount(() => {
		renderer.domElement.parentElement?.appendChild(clickTarget)
		clickTarget.addEventListener("click", handleClick)
		clickTarget.addEventListener("mousemove", handleMousemove)
		clickTarget.addEventListener("mouseleave", handleMouseleave)
	})

	onDestroy(() => {
		renderer.domElement.parentElement?.removeChild(clickTarget)
		clickTarget.removeEventListener("click", handleClick)
		clickTarget.removeEventListener("mousemove", handleMousemove)
		clickTarget.removeEventListener("mouseleave", handleMouseleave)
	})

	// Rotate the gizmo as the camera moves.
	const point = new Vector3()
	let p = [0, 0, 0]
	useTask(
		animationTask?.key ?? Symbol("cube-gizmo-animation"),
		() => {
			point.set(0, 0, 1).applyQuaternion(camera.current.quaternion)
			// Under rare orientations, a navigation button could be clicked, and the model could rotate without this point
			// moving, and hence the gizmo would not move, so we also check against a navControlClicked flag.
			// The aforemention rare scenario happens on page load when curvedArrowLeft is clicked immediately.
			if (point.x !== p[0] || point.y !== p[1] || point.z !== p[2] || navControlClicked) {
				if (navControlClicked) {
					navControlClicked = false
				}
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

		// Cube gray color is applied via the mesh material rather than fillColor, we don't want the two to stack.
		const fillColor = new Color(white)
		context.fillStyle = fillColor.convertSRGBToLinear().getStyle()
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

		const fillColor = new Color(white)
		context.fillStyle = fillColor.convertSRGBToLinear().getStyle()
		context.fill()

		const texture = new CanvasTexture(canvas)
		textures[key] = texture
		return texture
	}

	var curvedArrowLeftImageData: ImageData
	var curvedArrowRightImageData: ImageData

	const getCurvedArrowSpriteTexture = (label = "") => {
		const key = `curved-arrow-${label}`
		if (textures[key]) {
			textures[key].dispose()
		}

		const canvas = document.createElement("canvas")
		canvas.width = curvedArrowCanvasWidth
		canvas.height = curvedArrowCanvasHeight

		const context = canvas.getContext("2d")!
		// mirror the arrow
		if (label == "right") {
			context.translate(canvas.width, 0)
			context.scale(-1, 1)
		}

		// Sprite will be positioned later such that the canvas origin is at the center of the gizmo.
		const originX = canvas.width
		const originY = canvas.height
		const arcRadius = 180
		const fillColor = new Color(white)
		const fillStyle = fillColor.convertSRGBToLinear().getStyle()

		// Draw the arc.
		context.beginPath()
		context.strokeStyle = fillStyle
		context.lineWidth = 14
		const startAngle = Math.PI + (Math.PI / 180) * 40 // 180 + 40 deg
		const endAngle = (Math.PI / 180) * (270 - 19) // 270 - 19 deg
		context.arc(originX, originY, arcRadius, startAngle, endAngle)
		context.stroke()

		// Calculate the point location of the tip of the arrow for drawing a triangle. The tip should be on the centerline
		// of the arc.
		// hypotenuse = radius
		// sin angle = opposite / radius -> opposite = radius * sin angle
		// cos angle = adjacent / radius -> adjacent = radius * sin angle
		// angle from the origin to the triangle tip in radians
		const angleForTriangleTip = (55 * Math.PI) / 180
		const opposite = arcRadius * Math.sin(angleForTriangleTip)
		const adjacent = arcRadius * Math.cos(angleForTriangleTip)
		const tipX = originX - opposite
		const tipY = originY - adjacent

		// Draw the triangle (arrow tip).
		const v1 = new Vector2(tipX, tipY)
		// v2 and v3 are rotated because we draw the triangle based on 45 deg, but arc starts at 40 instead of 45 deg
		const triangleLegLength = 30
		const triangleRotationAngle = -5 // deg
		const v2preRotation = new Vector2(tipX, tipY - triangleLegLength)
		const v2 = v2preRotation.rotateAround(v1, (Math.PI / 180) * triangleRotationAngle)
		const v3preRotation = new Vector2(tipX + triangleLegLength, tipY)
		const v3 = v3preRotation.rotateAround(v1, (Math.PI / 180) * triangleRotationAngle)
		context.beginPath()
		context.moveTo(v1.x, v1.y)
		context.lineTo(v2.x, v2.y)
		context.lineTo(v3.x, v3.y)
		context.fillStyle = fillStyle
		context.fill()

		// Store the imageData for use with handleClick.
		if (label == "right") {
			curvedArrowRightImageData = context.getImageData(0, 0, canvas.width, canvas.height)
		} else {
			curvedArrowLeftImageData = context.getImageData(0, 0, canvas.width, canvas.height)
		}

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
				bind:ref={cubeRight}
				color={gray}
				map={getCubeSpriteTexture(textureSize, "Right")}
				attach={(parent, self) => {
					if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
					else parent.material = [self]
				}}
			/>
			<T.MeshBasicMaterial
				bind:ref={cubeLeft}
				color={gray}
				map={getCubeSpriteTexture(textureSize, "Left")}
				attach={(parent, self) => {
					if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
					else parent.material = [self]
				}}
			/>
			<T.MeshBasicMaterial
				bind:ref={cubeBack}
				color={gray}
				map={getCubeSpriteTexture(textureSize, "Back")}
				attach={(parent, self) => {
					if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
					else parent.material = [self]
				}}
			/>
			<T.MeshBasicMaterial
				bind:ref={cubeFront}
				color={gray}
				map={getCubeSpriteTexture(textureSize, "Front")}
				attach={(parent, self) => {
					if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
					else parent.material = [self]
				}}
			/>
			<T.MeshBasicMaterial
				bind:ref={cubeTop}
				color={gray}
				map={getCubeSpriteTexture(textureSize, "Top")}
				attach={(parent, self) => {
					if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
					else parent.material = [self]
				}}
			/>
			<T.MeshBasicMaterial
				bind:ref={cubeBottom}
				color={gray}
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
			<T.SpriteMaterial color={gray} map={getTriangleSpriteTexture("down")} />
		</T.Sprite>
		<T.Sprite bind:ref={leftTriangle} position={[-1.85, 0, 1]} scale={0.4}>
			<T.SpriteMaterial color={gray} map={getTriangleSpriteTexture("left")} rotation={(3 * Math.PI) / 2} />
		</T.Sprite>
		<T.Sprite bind:ref={upTriangle} position={[0, 1.85, 1]} scale={0.4}>
			<T.SpriteMaterial color={gray} map={getTriangleSpriteTexture("up")} rotation={Math.PI} />
		</T.Sprite>
		<T.Sprite bind:ref={rightTriangle} position={[1.85, 0, 1]} scale={0.4}>
			<T.SpriteMaterial color={gray} map={getTriangleSpriteTexture("right")} rotation={Math.PI / 2} />
		</T.Sprite>
		<T.Sprite bind:ref={curvedArrowLeft} position={[-1, 1, 1]} scale={2}>
			<T.SpriteMaterial color={gray} map={getCurvedArrowSpriteTexture("left")} />
		</T.Sprite>
		<T.Sprite bind:ref={curvedArrowRight} position={[1, 1, 1]} scale={2}>
			<T.SpriteMaterial color={gray} map={getCurvedArrowSpriteTexture("right")} />
		</T.Sprite>
	</T>
</HierarchicalObject>
