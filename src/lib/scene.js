// place files you want to import through the `$lib` alias in this folder.
import * as THREE from 'three';
import { TrackballControls } from 'three/addons/controls/TrackballControls.js';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

export const createScene = (el) => {
    const scene = new THREE.Scene();

    const { width, height } = el.getBoundingClientRect();
    // const camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
    const aspectRatio = width / height;
    const worldWidth = 10;
    const worldHeight = worldWidth / aspectRatio;
    const camera = new THREE.OrthographicCamera(worldWidth / - 2, worldWidth / 2, worldHeight / 2, worldHeight / - 2, 0.1, 1000);

    camera.position.z = -5;
    camera.lookAt(0, 0, 0);

    console.log("el:", el)
    const controls = new TrackballControls(camera, el);
    controls.rotateSpeed = 3.0;
    // const controls = new OrbitControls(camera, el);


    const geometry = new THREE.BoxGeometry();

    const material = new THREE.MeshStandardMaterial({
        color: 0x00ff00,
        metalness: 0.13
    });

    const cube = new THREE.Mesh(geometry, material);
    scene.add(cube);

    const directionalLight = new THREE.DirectionalLight(0x9090aa);
    directionalLight.position.set(-10, 10, -10).normalize();
    scene.add(directionalLight);

    const hemisphereLight = new THREE.HemisphereLight(0xffffff, 0x444444);
    hemisphereLight.position.set(1, 1, 1);
    scene.add(hemisphereLight);

    let renderer;

    const animate = () => {
        requestAnimationFrame(animate);
        // cube.rotation.x += 0.01;
        // cube.rotation.y += 0.01;

        // required if controls.enableDamping or controls.autoRotate are set to true
        controls.update();

        renderer.render(scene, camera);
    };

    const resize = () => {
        const { width, height } = el.getBoundingClientRect();
        renderer.setSize(width, height);
        camera.aspect = width / height;
        camera.updateProjectionMatrix();
    };

    const getStarted = (el) => {
        renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });
        renderer.setClearColor("#EEEEEE");
        resize();
        animate();
    };

    window.addEventListener('resize', resize);

    getStarted(el);
    console.log("Created scene!");
}



