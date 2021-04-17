import { Spherical, DoubleSide, GridHelper, MathUtils, Mesh, MeshBasicMaterial, PerspectiveCamera, PlaneGeometry, Scene, Sphere, Vector3, WebGLRenderer } from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';

// Get the canvas and compass image
const canvas = document.getElementById('3d_orientation');
const compass = document.getElementById('compass');

// Basically we want the height and width once its computed
const parent_computed_styles = getComputedStyle(canvas.parentElement);

// Get the width and height of the parent
canvas.width = parseInt(parent_computed_styles.width) - 4;
canvas.height = parseInt(parent_computed_styles.height) - 1;

const scene = new Scene();

// Create a camera and add arcball controls
const camera = new PerspectiveCamera(75, canvas.width / canvas.height, 0.1, 1000);
const controls = new OrbitControls(camera, canvas);

// Create a renderer
const renderer = new WebGLRenderer({ canvas: canvas });
renderer.setSize(canvas.width, canvas.height);

// Load the models
// const loader = new GLTFLoader();
const geometry = new PlaneGeometry(5, 5, 32);
const material = new MeshBasicMaterial({ color: 0xffffff, side: DoubleSide });
const cube = new Mesh(geometry, material);

const gridHelper = new GridHelper(10, 10, 'aqua', 'gray');

// For the compass
const sph = new Spherical();
const dir = new Vector3();

// Create the scene
scene.add(cube);
scene.add(gridHelper);

camera.position.z = 10;
controls.update();

function animate() {
  requestAnimationFrame(animate);

  camera.getWorldDirection(dir);
  sph.setFromVector3(dir);
  compass.style.transform = `rotate(${MathUtils.radToDeg(sph.theta) - 180}deg)`;
  controls.update();
  renderer.render(scene, camera);
}
animate();

const update_scene = (data) => {
  // we take the orientation and apply it to the plane representing the drone 
}

const subscribe = (subject) => {
  subject.subscribe({
    next: update_scene,
  })
}
export { subscribe };

