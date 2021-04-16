import 'leaflet/dist/leaflet.css';
import L from 'leaflet';

// Create the map
const map = L.map('gps_map', { center: [49.309862, -123.068316], zoom: 13 });
// Add tiles to the map
L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
  attribution: '© OpenStreetMap contributors'
}).addTo(map);

