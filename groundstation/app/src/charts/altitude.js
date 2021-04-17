import Chart from 'chart.js/auto';

const options = {
    responsive: true,
    // animations should be off....
    animation: true, // unfortunately, there seems to be a bug with chart.js when animation is off it throws errors
    plugins: {
        legend: {
            position: 'top',
            display: true,
        },
        title: {
            text: 'Altitude',
            display: true,
        }
    },
    scales: {
        y: {
            display: true,
            title: {
                text: 'Altitude',
                display: true,
            }
        }, x: {
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
            label: 'Altitude',
            data: [],
            borderColor: 'aqua',
            backgroundColor: 'aqua',
        }
    ]
};

const canvas = document.getElementById('chart_altitude');
const chart = new Chart(canvas, {
    type: 'line',
    data: data,
    options: options,
});

/**
 * update_chart updates the chart
 * @param {*} data an array of the data we are looking at
 */
const update_chart = (data) => {
    return; // TODO: remove when we figure out the logging format
    chart.data.labels = data.map((elem) => elem.time);
    chart.data.datasets[0].data = data.map((elem) => elem.altitude);
    chart.update();
};

const subscribe = (subject) => {
    subject.subscribe({
        next: update_chart,
    });
}

export { subscribe };
