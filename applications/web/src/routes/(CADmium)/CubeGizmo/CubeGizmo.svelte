<script lang="ts">
  import { HierarchicalObject, T, useTask, useThrelte } from '@threlte/core'
  import { onDestroy, onMount } from 'svelte'
  import {
    CanvasTexture,
    CapsuleGeometry,
    Color,
    Euler,
    Mesh,
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
    type Intersection
  } from 'three'

  import type { SetCameraFocus } from "shared/types"
  import type { CubeGizmoEvents, CubeGizmoProps, CubeGizmoSlots } from './CubeGizmo'

  type $$Props = CubeGizmoProps
  type $$Events = CubeGizmoEvents
  type $$Slots = CubeGizmoSlots

  export let renderTask: $$Props['renderTask'] = undefined
  export let animationTask: $$Props['animationTask'] = undefined
  export let setCameraFocus: SetCameraFocus

  export let turnRate: Required<$$Props>['turnRate'] = 2 * Math.PI
  export let center: Required<$$Props>['center'] = [0, 0, 0]
  export let verticalPlacement: Required<$$Props>['verticalPlacement'] = 'bottom'
  export let horizontalPlacement: Required<$$Props>['horizontalPlacement'] = 'right'
  export let size: Required<$$Props>['size'] = 128
  export let xColor: Required<$$Props>['xColor'] = 0xff0000 // red
  export let yColor: Required<$$Props>['yColor'] = 0x179316 // green
  export let zColor: Required<$$Props>['zColor'] = 0x0000ff // blue
  let gray = 0xdde6ed
  // let white = 0xffffff // TODO(sosho): Set backgroundColor to white after adding rotation edges.
  let black = 0x000000
  export let toneMapped: Required<$$Props>['toneMapped'] = false
  export let paddingX: Required<$$Props>['paddingX'] = 0
  export let paddingY: Required<$$Props>['paddingY'] = 0
  const origin = new Vector3(0,0,0)

  $: centerVec = new Vector3(...center)

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
    renderTask?.key ?? Symbol('cube-gizmo-render'),
    () => {
      const autoClear = renderer.autoClear
      renderer.autoClear = false
      renderer.getViewport(viewport)
      const toneMapping = renderer.toneMapping
      renderer.toneMapping = toneMapped ? renderer.toneMapping : 0

      const x =
        horizontalPlacement === 'left'
          ? paddingX
          : renderer.domElement.offsetWidth - size - paddingX
      const y =
        verticalPlacement === 'bottom'
          ? paddingY
          : renderer.domElement.offsetHeight - size - paddingY

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
  const clickTarget = document.createElement('div')
  // We need to know the bounding rect of the renderer's dom element
  const renderTarget = renderer.domElement
  const boundingRect = renderTarget.getBoundingClientRect()

  clickTarget.style.position = 'absolute'
  $: {
    if (horizontalPlacement === 'right') {
      clickTarget.style.right = `${paddingX}px`
      clickTarget.style.left = ''
    } else {
      clickTarget.style.right = ''
      clickTarget.style.left = `${paddingX + boundingRect.left}px`
    }

    if (verticalPlacement === 'bottom') {
      clickTarget.style.bottom = ''
      clickTarget.style.top = `${boundingRect.bottom - size - paddingY}px`
    } else {
      clickTarget.style.bottom = ''
      clickTarget.style.top = `${paddingY + boundingRect.top}px`
    }

    clickTarget.style.height = `${size}px`
    clickTarget.style.width = `${size}px`
  }

  let posX: Sprite
  let posY: Sprite
  let posZ: Sprite
  let negX: Sprite
  let negY: Sprite
  let negZ: Sprite
  let cube: Mesh
  let downTriangle: Sprite
  let leftTriangle: Sprite
  let upTriangle: Sprite
  let rightTriangle: Sprite

  const targetPosition = new Vector3()
  const targetQuaternion = new Quaternion()
  const currentQuaternion = new Quaternion()
  const finalQuaternion = new Quaternion()
  let radius = 0

  let animating = false
  const mouse = new Vector2()
  const raycaster = new Raycaster()

  /**
   * Floating point operations make it hard to compare quaternions, controls
   * (such as the OrbitControls) may also restrict the rotation of the camera on
   * certain axes. To allow for loose equality checks, we use a sensible
   * threshold to compare quaternions.
   *
   * @param a - Quaternion a
   * @param b - Quaternion b
   * @param threshold - Threshold to use for comparison
   */
  const quaternionsAreEqual = (a: Quaternion, b: Quaternion, threshold: number) => {
    const delta =
      Math.abs(a.x - b.x) + Math.abs(a.y - b.y) + Math.abs(a.z - b.z) + Math.abs(a.w - b.w)
    return delta < threshold
  }

  /**
   * @returns boolean that indicates if the target and the current rotation are equal.
   */
  const handleIntersection = (intersection: Intersection<Object3D>): boolean => {
    const object = intersection.object
    const targetPos = object.userData.targetPosition as [number, number, number]
    const targetEuler = object.userData.targetEuler as [number, number, number]

    radius = camera.current.position.distanceTo(centerVec)
    targetPosition
      .set(...targetPos)
      .multiplyScalar(radius)
      .add(centerVec)
    targetQuaternion.setFromEuler(new Euler(...targetEuler))

    const dummy = new Object3D()
    dummy.position.copy(centerVec)

    dummy.lookAt(camera.current.position)
    currentQuaternion.copy(dummy.quaternion)

    dummy.lookAt(targetPosition)
    finalQuaternion.copy(dummy.quaternion)

    if (quaternionsAreEqual(finalQuaternion, currentQuaternion, 0.0001)) {
      return true
    }

    animating = true
    return false
  }

  /**
   * @returns boolean indicating if value is effectively 1.
   */
  const approachesOne = (num: number) => 0.9999 < num && num < 1.0001

  const handleClick = (event: MouseEvent) => {
    if (animating) {
      return
    }

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
      // TODO(sosho): add slerp
      switch (faceIndex) {
        // Right
        case 0:
        case 1:
          setCameraFocus({x: 1, y:0, z:0}, origin, {x: 0, y:0, z:1})
          break
        // Left
        case 2:
        case 3:
          setCameraFocus({x: -1, y:0, z:0}, origin, {x: 0, y:0, z:1})
          break
        // Back
        case 4:
        case 5:
          setCameraFocus({x: 0, y:1, z:0}, origin, {x: 0, y:0, z:1})
          break
        // Front
        case 6:
        case 7:
          setCameraFocus({x: 0, y:-1, z:0}, origin, {x: 0, y:0, z:1})
          break
        // Top
        case 8:
        case 9:
          setCameraFocus({x: 0, y:0, z:1}, origin, {x: 0, y:1, z:0})
          break
        // Bottom
        case 10:
        case 11:
          setCameraFocus({x: 0, y:0, z:-1}, origin, {x: 0, y:-1, z:0})
          break
        default:
          break
      }
    }

    const triangleIntersects = raycaster.intersectObjects([downTriangle, leftTriangle, upTriangle, rightTriangle])
    if (triangleIntersects.length) {
      const cameraUp = camera.current.up.clone()
      const quaternion = new Quaternion();

      const triangleSprite = triangleIntersects[0].object
      switch(triangleSprite) {
        case downTriangle:
        case upTriangle:
          const angle = triangleSprite == downTriangle ? -Math.PI/12 : Math.PI/12 // 15 deg
      
          // Rotate camera to new position.
          const planeTriangle = new Triangle(origin, cameraUp, camera.current.position)
          const upDownRotationAxis = planeTriangle.getNormal(origin)
          quaternion.setFromAxisAngle(upDownRotationAxis, angle)
          camera.current.position.applyQuaternion(quaternion)

          // Set camera up vector (prevents orientation inversion).
          // Crossing the vectors follows left-hand rule for calculating the perpendicular up vector.
          const normalizedCameraPos = camera.current.position.clone().normalize()
          const newUp = normalizedCameraPos.cross(upDownRotationAxis)
          break
        case leftTriangle:
          quaternion.setFromAxisAngle(cameraUp, Math.PI/12)
          camera.current.position.applyQuaternion(quaternion)
          break
        case rightTriangle:
          quaternion.setFromAxisAngle(cameraUp, -Math.PI/12)
          camera.current.position.applyQuaternion(quaternion)
          break
      }
    }

    // TODO(sosho): I think this can be removed or modified/replaced with the rotation controls
    // const intersects = raycaster.intersectObjects([posX, posY, posZ, negX, negY, negZ])

    // if (intersects.length > 0) {
    //   const alreadyReached = handleIntersection(intersects[0])
    //   if (alreadyReached) {
    //     // get the second closest intersection
    //     if (intersects.length > 1) {
    //       handleIntersection(intersects[1])
    //     }
    //   }
    // }
  }

  onMount(() => {
    renderer.domElement.parentElement?.appendChild(clickTarget)
    clickTarget.addEventListener('click', handleClick)
  })

  onDestroy(() => {
    renderer.domElement.parentElement?.removeChild(clickTarget)
    clickTarget.removeEventListener('click', handleClick)
  })

  // Used to test which axis (pos or neg) are closer to the camera.
  const point = new Vector3()
  let p = [0, 0, 0]

  useTask(
    animationTask?.key ?? Symbol('cube-gizmo-animation'),
    (delta) => {
      point.set(0, 0, 1).applyQuaternion(camera.current.quaternion)
      if (point.x !== p[0] || point.y !== p[1] || point.z !== p[2]) {
        p = [point.x, point.y, point.z]
        rotationRoot.quaternion.copy(camera.current.quaternion).invert()
        invalidate()
      }

      if (animating) {
        const step = delta * turnRate
        // animate position by doing a slerp and then scaling the position on the unit sphere
        currentQuaternion.rotateTowards(finalQuaternion, step)
        camera.current.position
          .set(0, 0, 1)
          .applyQuaternion(currentQuaternion)
          .multiplyScalar(radius)
          .add(centerVec)

        // animate orientation
        camera.current.quaternion.rotateTowards(targetQuaternion, step)

        if (currentQuaternion.angleTo(finalQuaternion) === 0) {
          animating = false
        }

        invalidate()
      }
    },
    {
      ...animationTask,
      autoInvalidate: false
    }
  )

  const findClosestPow2LargerThan = (x: number) => {
    if (x <= 0) {
      return 1
    }
    let pow2 = 1
    while (pow2 < x) {
      pow2 <<= 1
    }
    return pow2
  }

  $: textureSize = findClosestPow2LargerThan(size * 0.3 * renderer.getPixelRatio())

  /**
   * Keep track of the textures to be able to dispose them when they are no
   * longer needed.
   */
  const textures: Record<string, CanvasTexture> = {}

  const color = new Color()
  const getAxisLabelSpriteTexture = (size: number, colorRepresentation: ColorRepresentation, text = '') => {
    color.set(colorRepresentation)
    const key = `${color.getHexString()}-${text}`
    if (textures[key]) {
      textures[key].dispose()
    }
    const canvas = document.createElement('canvas')
    canvas.width = size
    canvas.height = size

    const context = canvas.getContext('2d')!

    if (text) {
      const textSize = Math.abs(size * (22 / 64))
      context.font = `${textSize}px Arial`
      context.textAlign = 'center'
      context.fillStyle = color.convertSRGBToLinear().getStyle()
      const textY = size * (41 / 64)
      context.fillText(text, size / 2, textY)
    }

    const texture = new CanvasTexture(canvas)
    textures[key] = texture
    return texture
  }

  const getCubeSpriteTexture = (size: number, text = '') => {
    const key = `cube-${text}`
    if (textures[key]) {
      textures[key].dispose()
    }
    const canvas = document.createElement('canvas')
    canvas.width = size
    canvas.height = size

    const context = canvas.getContext('2d')!

    const backgroundColor = new Color(gray);
    context.fillStyle = backgroundColor.convertSRGBToLinear().getStyle()
    context.fillRect(0,0, canvas.width, canvas.height)

    if (text) {
      const textSize = Math.abs(size * (16 / 64))
      context.font = `${textSize}px Arial`
      context.textAlign = 'center'
      const textColor = new Color(black)
      context.fillStyle = textColor.convertSRGBToLinear().getStyle()
      const textXPos = size / 2
      const textYPos = size * (41 / 64)
      context.fillText(text, textXPos, textYPos)
    }

    const texture = new CanvasTexture(canvas)
    texture.generateMipmaps = false // makes text sharper

    // Rotate text.
    texture.center = new Vector2(0.5, 0.5);
    switch (text) {
      case 'Right':
        texture.rotation = Math.PI/2 // 90 deg
        break
      case 'Back':
      case 'Bottom':
        texture.rotation = Math.PI
        break
      case 'Left':
      texture.rotation = 3 * Math.PI/2
        break
      // other faces don't need to be rotated.
    }

    textures[key] = texture
    return texture
  }

  const getTriangleSpriteTexture = (label = '') => {
    const key = `triangle-${label}`
    if (textures[key]) {
      textures[key].dispose()
    }

    const canvas = document.createElement('canvas')
    canvas.width = 100
    canvas.height = 73

    const context = canvas.getContext('2d')!
    const v1 = new Vector2(0, 0)
    const v2 = new Vector2(50, 72.1)
    const v3 = new Vector2(100, 0)
    context.beginPath()
    context.moveTo(v1.x, v1.y)
    context.lineTo(v2.x, v2.y)
    context.lineTo(v3.x, v3.y)

    const backgroundColor = new Color(gray);
    context.fillStyle = backgroundColor.convertSRGBToLinear().getStyle()
    context.fill()

    const texture = new CanvasTexture(canvas)
    textures[key] = texture
    return texture
  }

  const stemGeometry = new CapsuleGeometry(0.02, 1.5)
  stemGeometry.rotateZ(Math.PI / 2)

  // Used to decrease atifacts of intersecting axis stems.
  $: frontMostAxisIndex = p.indexOf(Math.max(...p))
  $: usePolygonOffset = p.some((v) => v < 0)
</script>

<HierarchicalObject>
  <T is={rotationRoot}>
    {@const polygonOffsetFactor = -20}

    <T.Mesh bind:ref={cube}>
      <T.BoxGeometry args={[1.5, 1.5, 1.5]} />
      <T.MeshBasicMaterial 
        map={getCubeSpriteTexture(textureSize, 'Right')}
        attach={(parent, self) => {
          if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
          else parent.material = [self]
        }}
      />
      <T.MeshBasicMaterial 
        map={getCubeSpriteTexture(textureSize, 'Left')}
        attach={(parent, self) => {
          if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
          else parent.material = [self]
        }}
      />
      <T.MeshBasicMaterial 
      map={getCubeSpriteTexture(textureSize, 'Back')}
      attach={(parent, self) => {
        if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
        else parent.material = [self]
      }}
      />
      <T.MeshBasicMaterial 
      map={getCubeSpriteTexture(textureSize, 'Front')}
      attach={(parent, self) => {
        if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
        else parent.material = [self]
      }}
      />
      <T.MeshBasicMaterial 
      map={getCubeSpriteTexture(textureSize, 'Top')}
      attach={(parent, self) => {
        if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
        else parent.material = [self]
      }}
      />
      <T.MeshBasicMaterial 
      map={getCubeSpriteTexture(textureSize, 'Bottom')}
      attach={(parent, self) => {
        if (Array.isArray(parent.material)) parent.material = [...parent.material, self]
        else parent.material = [self]
      }}
      />
    </T.Mesh>

    <!-- xAxis -->
    <T.Sprite
      renderOrder={1}
      bind:ref={posX}
      position={[1, -0.75, -0.75]}
      userData.targetPosition={[1, 0, 0]}
      userData.targetEuler={[0, Math.PI * 0.5, 0]}
    >
      <!-- hide the text 'X' when xAxis is not visible(is orthogonal to view) -->
      <T.SpriteMaterial
        map={getAxisLabelSpriteTexture(textureSize, xColor, 'X')}
        opacity={approachesOne(-1 * p[0]) || approachesOne(p[0]) ? 0 : 1}
      />
    </T.Sprite>

    <T.Mesh
      position={[0,-0.75,-0.75]}
      renderOrder={frontMostAxisIndex === 0 ? -1 : 0}
    >
      <T is={stemGeometry} />
      <T.MeshBasicMaterial
        transparent
        color={xColor}
        polygonOffset={usePolygonOffset && frontMostAxisIndex === 0 && p[0] < 0.75}
        {polygonOffsetFactor}
      />
    </T.Mesh>

    <!-- yAxis -->
    <T.Sprite
      renderOrder={1}
      bind:ref={posY}
      position={[-0.75, 1, -0.75]}
      userData.targetPosition={[0, 1, 0]}
      userData.targetEuler={[-Math.PI * 0.5, 0, 0]}
    >
      <!-- hide the text 'Y' when yAxis is not visible(is orthogonal to view) -->
      <T.SpriteMaterial
        map={getAxisLabelSpriteTexture(textureSize, yColor, 'Y')}
        opacity={approachesOne(-1 * p[1]) || approachesOne(p[1]) ? 0 : 1}
      />
    </T.Sprite>

    <T.Mesh
      position={[-0.75,0,-0.75]}
      rotation.z={Math.PI / 2}
      renderOrder={frontMostAxisIndex === 1 ? -1 : 0}
    >
      <T is={stemGeometry} />
      <T.MeshBasicMaterial
        transparent
        color={yColor}
        polygonOffset={usePolygonOffset && frontMostAxisIndex === 1 && p[1] < 0.75}
        {polygonOffsetFactor}
      />
    </T.Mesh>

    <!-- zAxis -->
    <T.Sprite
      renderOrder={1}
      bind:ref={posZ}
      position={[-0.75, -0.75, 1]}
      userData.targetPosition={[0, 0, 1]}
      userData.targetEuler={[0, 0, 0]}
    >
      <!-- hide the text 'Z' when zAxis is not visible(is orthogonal to view) -->
      <T.SpriteMaterial
        map={getAxisLabelSpriteTexture(textureSize, zColor, 'Z')}
        opacity={approachesOne(-1 * p[2]) || approachesOne(p[2]) ? 0 : 1}
      />
    </T.Sprite>

    <T.Mesh
    position={[-0.75,-0.75,0]}
      rotation.y={-Math.PI / 2}
      renderOrder={frontMostAxisIndex === 2 ? -1 : 0}
    >
      <T is={stemGeometry} />
      <T.MeshBasicMaterial
        transparent
        color={zColor}
        polygonOffset={usePolygonOffset && frontMostAxisIndex === 2 && p[2] < 0.75}
        {polygonOffsetFactor}
      />
    </T.Mesh>
  </T>
  <T is={triangleControls}>
    <T.Sprite
      bind:ref={downTriangle}
      position={[0,-1.85,1]}
      scale={0.4}
    >
      <T.SpriteMaterial
        map={getTriangleSpriteTexture('down')}
      />
    </T.Sprite>
    <T.Sprite
      bind:ref={leftTriangle}
      position={[-1.85,0,1]}
      scale={0.4}
    >
      <T.SpriteMaterial
        map={getTriangleSpriteTexture('left')}
        rotation={3*Math.PI/2}
      />
    </T.Sprite>
    <T.Sprite
      bind:ref={upTriangle}
      position={[0,1.85,1]}
      scale={0.4}
    >
      <T.SpriteMaterial
        map={getTriangleSpriteTexture('up')}
        rotation={Math.PI}
      />
    </T.Sprite>
    <T.Sprite
      bind:ref={rightTriangle}
      position={[1.85,0,1]}
      scale={0.4}
    >
      <T.SpriteMaterial
        map={getTriangleSpriteTexture('right')}
        rotation={Math.PI/2}
      />
    </T.Sprite>
  </T> 
</HierarchicalObject>
