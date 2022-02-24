<template>
  <div>
    <div class="flow-root p-6">
      <ul role="list" class="-my-5 divide-y divide-gray-200">
        <li v-for="locationForecast in forecasts" :key="locationForecast.name" class="py-4">
          <LocationForecast :location="locationForecast"/>
        </li>
      </ul>
    </div>
<!--    <div class="mt-6">-->
<!--      <a href="#" class="w-full flex justify-center items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"> View all </a>-->
<!--    </div>-->
  </div>
</template>

<script>
import LocationForecast from './LocationForecast.vue';

/*
const fakeForecasts = [
  {
    name: 'Mont Carmel',
    latitude: 12.1313,
    longitude: 12.12313,
    dayForecasts: [
      {
        dt: 1645527600,
        icon: '01d',
        aveTemp: 15,
        minTemp: 10,
        maxTemp: 22,
        humidity: 30,
        precipitation: 2,
      },
      {
        dt: 1645527600,
        icon: '02d',
        aveTemp: 15,
        minTemp: 10,
        maxTemp: 22,
        humidity: 30,
        precipitation: 2,
      },
      {
        dt: 1645527600,
        icon: '02d',
        aveTemp: 15,
        minTemp: 10,
        maxTemp: 22,
        humidity: 30,
        precipitation: 2,
      },
      {
        dt: 1645527600,
        icon: '02d',
        aveTemp: 15,
        minTemp: 10,
        maxTemp: 22,
        humidity: 30,
        precipitation: 2,
      },
    ],
  },
];
 */

export default {
  name: 'ForecastsDisplay',
  components: {LocationForecast},
  // setup() {
  //   return {
  //     forecasts: [],
  //   };
  // },
  data () {
    return {
      forecasts: [],
    }
  },
  async mounted() {
    const apiUrl = 'http://0.0.0.0:8088';
    const savedLocations = [
      {
        name: "Stoneham",
        lat: 46.999607,
        lon: -71.36948,
        country: "CA",
      },
      {
        name: "Chicoutimi",
        lat: 48.428635,
        lon: -71.06371,
        country: "CA",
      },
      {
        name: "Kamouraska",
        lat: 47.566795,
        lon: -69.86687,
        country: "CA",
      },
    ];

    for (const location of savedLocations) {
      const response = await fetch(`${apiUrl}/forecast/${location.lat},${location.lon}`);
      const forecast = await response.json();
      forecast.name = location.name;
      forecast.country = location.country;
      this.forecasts.push(forecast);
    };

  },
};
</script>
