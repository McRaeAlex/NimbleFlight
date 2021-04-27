import Chart from 'chart.js/auto';

const options = {
    responsive: true,
    // animations should be off....
    animations: false, // unfortunately, there seems to be a bug with chart.js when animation is off it throws errors
    plugins: {
        legend: {
            position: 'top',
        },
        title: {
            text: 'Magnetometer',
            display: true,
        }
    },
    scales: {
        y: {
            display: true,
            title: {
                text: 'Guass (mG)',
                display: true,
            },
            min: -1000,
            max: 1000,
        },
        x: {
            display: true,
            title: {
                text: 'Time',
                display: true,
            }
        },
    }
};

const data = {
    labels: [],
    datasets: [
        {
            label: 'Guass x',
            data: [],
            borderColor: 'red',
            backgroundColor: 'red',
        },
        {
            label: 'Guass y',
            data: [],
            borderColor: 'blue',
            backgroundColor: 'blue',
        },
        {
            label: 'Guass z',
            data: [],
            borderColor: 'green',
            backgroundColor: 'green',
        }
    ]
};

const chart_acceleration = document.getElementById('chart_magnetometer');
const chart = new Chart(chart_acceleration, {
    type: 'line',
    data: data,
    options: options,
});

/**
 * update_chart updates the chart
 * @param {*} data an array of the data we are looking at
 */
const update_chart = (data) => {
    chart.data.labels = data.map((elem) => elem.time);
    chart.data.datasets[0].data = data.map((elem) => elem.magnetometer.x);
    chart.data.datasets[1].data = data.map((elem) => elem.magnetometer.y);
    chart.data.datasets[2].data = data.map((elem) => elem.magnetometer.z);
    chart.update();
};

const subscribe = (subject) => {
    subject.subscribe({
        next: update_chart,
    });
}
export { subscribe };
