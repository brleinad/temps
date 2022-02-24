<template>
  <div>
    <div class="flow-root p-6">
      <ul role="list" class="-my-5 divide-y divide-gray-200">
        <li v-for="locationForecast in forecasts" :key="locationForecast.name" class="py-4">
          <LocationForecast :location="locationForecast"/>
        </li>
      </ul>
    </div>
  </div>
</template>

<script>
import LocationForecast from './LocationForecast.vue';
import {useLocationsStore} from '@/stores/locations';

export default {
  name: 'ForecastsDisplay',
  components: {LocationForecast},
  setup() {
  },
  data () {
    return {
      forecasts: [],
    }
  },
  props: {
    apiUrl: {
      required: true,
      type: String,
    },
  },
  async mounted() {
    // const locationsStore = useLocationsStore();

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
      {
        name: "Kamouraska",
        lat: 47.566795,
        lon: -69.86687,
        country: "CA",
      },
    ];

    for (const location of savedLocations) {
      const response = await fetch(`${this.apiUrl}/forecast/${location.lat},${location.lon}`);
      const forecast = await response.json();
      forecast.name = location.name;
      forecast.country = location.country;
      this.forecasts.push(forecast);
    };

  },
};
</script>
