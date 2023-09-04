// place files you want to import through the `$lib` alias in this folder.
import * as THREE from 'three';

export const createScene = (el) => {
    const scene = new THREE.Scene();

    const { width, height } = el.getBoundingClientRect();
    const camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);

    camera.position.z = 5;

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
        cube.rotation.x += 0.01;
        cube.rotation.y += 0.01;
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
        resize();
        animate();
    };

    window.addEventListener('resize', resize);

    getStarted(el);
    console.log("Created scene!");
}



