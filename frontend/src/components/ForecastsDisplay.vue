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
    savedLocations: {
      type: Array,
    }
  },
  async mounted() {
    // this.getForecasts(); not needed?
  },
  watch: {
    savedLocations() {
      this.getForecasts();
    },
  },
  methods: {
    async getForecasts() {
      this.forecasts = []; //  TODO: avoid refreshing every time
      for (const location of this.savedLocations) {
        const response = await fetch(`${this.apiUrl}/forecast/${location.lat},${location.lon}`);
        const forecast = await response.json();
        forecast.name = location.name;
        forecast.country = location.country;
        this.forecasts.push(forecast);
      };
    },
  },
};
</script>
