import ApexCharts from 'apexcharts'

/**
 * Display the online-users chart
 * Needs a div with id="chart-online" in the DOM
 */
export default function onlineChart(): void {
  const options = {
    series: [
      {
        name: 'Online users',
        data: [
          10,
          41,
          35,
          26,
          49,
          62,
          69,
          0,
          0,
          101,
          10,
          41,
          35,
          26,
          49,
          62,
          69,
          73,
          90,
          101,
          null,
          null,
          null,
          null
        ]
      }
    ],
    chart: {
      height: 350,
      type: 'line',
      zoom: {
        enabled: false
      }
    },
    dataLabels: {
      enabled: false
    },
    stroke: {
      curve: 'smooth'
    },
    grid: {
      row: {
        colors: ['#FFFFFF', '#F3F3F3'], // takes an array which will be repeated on columns
        opacity: 1
      }
    },
    xaxis: {
      labels: {
        style: {
          colors: '#000000'
        }
      },
      categories: [
        '12 am',
        '1 am',
        '2 am',
        '3 am',
        '4 am',
        '5 am',
        '6 am',
        '7 am',
        '8 am',
        '9 am',
        '10 am',
        '11 am',
        '12 pm',
        '1 pm',
        '2 pm',
        '3 pm',
        '4 pm',
        '5 pm',
        '6 pm',
        '7 pm',
        '8 pm',
        '9 pm',
        '10 pm',
        '11 pm'
      ]
    },
    yaxis: {
      labels: {
        style: {
          colors: '#000000'
        }
      }
    }
  }

  const chart = new ApexCharts(document.querySelector('#chart-online'), options)
  chart.render()
}
