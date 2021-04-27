import './index.css';
import { webSocket } from 'rxjs/webSocket';
import { BehaviorSubject } from 'rxjs';

import {subscribe as chart_accel_sub} from './src/charts/acceleration';
import {subscribe as chart_magnetometer_sub} from './src/charts/magnetometer';
import {subscribe as chart_gyro_sub} from './src/charts/gyroscope';
import {subscribe as threejs_sub} from './src/3d_orientation';

import {} from './src/command_prompt';
import {} from './src/gps_map';
import {} from './src/camera.js';

const time_interval = 180;
const data = [];
const current_data_sub = new BehaviorSubject([]);
// We can also have a timeslice data sub which will return a array of data between two times
// it should be another subject
// I think ideally this is all encapsulated into a datapipeline where we can swap out things

// Websocket
const sub = webSocket('ws://localhost:3001');

sub.subscribe({
  next: (val) => {
    data.push(val); // This is the 'long' term storage of the data
    
    // Take the last n datapoints
    const start_index = Math.max(data.length - time_interval, 0);
    const curr_data = data.slice(start_index, data.length);

    // Update the subscription
    current_data_sub.next(curr_data);
  }
});

chart_accel_sub(current_data_sub);
chart_gyro_sub(current_data_sub);
chart_magnetometer_sub(current_data_sub);
// chart_orientation_sub(current_data_sub);
// chart_altitude_sub(current_data_sub);
// threejs_sub(current_data_sub);