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

const forecasts = [
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
  {
    name: 'Kamou',
    latitude: 12.1313,
    longitude: 12.12313,
    dayForecasts: [
      {
        dt: 1645527600,
        icon: '03d',
        aveTemp: 15,
        minTemp: 10,
        maxTemp: 22,
        humidity: 30,
        precipitation: 2,
      },
      {
        dt: 1645527600,
        icon: '03d',
        aveTemp: 15,
        minTemp: 10,
        maxTemp: 22,
        humidity: 30,
        precipitation: 2,
      },
      {
        dt: 1645527600,
        icon: '04d',
        aveTemp: 15,
        minTemp: 10,
        maxTemp: 22,
        humidity: 30,
        precipitation: 2,
      },
      {
        dt: 1645527600,
        icon: '03d',
        aveTemp: 15,
        minTemp: 10,
        maxTemp: 22,
        humidity: 30,
        precipitation: 2,
      },
    ],
  },
];

export default {
  name: 'ForecastsDisplay',
  components: {LocationForecast},
  setup() {
    return {
      forecasts,
    }
  },
};
</script>
