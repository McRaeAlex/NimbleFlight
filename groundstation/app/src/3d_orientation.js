import { BoxGeometry, Mesh, MeshBasicMaterial, PerspectiveCamera, Scene, WebGLRenderer } from 'three';

const canvas = document.getElementById('3d_orientation');

// Basically we want the height and width once its computed
const parent_computed_styles = getComputedStyle(canvas.parentElement);

console.info(parent_computed_styles.width, parent_computed_styles.height);
canvas.width = parseInt(parent_computed_styles.width) - 4;
canvas.height = parseInt(parent_computed_styles.height) - 1;

const scene = new Scene();
const camera = new PerspectiveCamera(75, canvas.width / canvas.height , 0.1, 1000);
const renderer = new WebGLRenderer({canvas: canvas});
renderer.setSize(canvas.width, canvas.height);
// const loader = new GLTFLoader();
const geometry = new BoxGeometry();
const material = new MeshBasicMaterial({color: 0x0ff011});
const cube = new Mesh(geometry, material);

// Create the scene
scene.add(cube);

camera.position.z = 5;

function animate() {
  requestAnimationFrame(animate);
  renderer.render(scene, camera);
}
animate();

const update_scene = (data) => {
   console.log(data); 
}
const subscribe = (subject) => {
    subject.subscribe({
        next: update_scene,
    })
}
export { subscribe };

