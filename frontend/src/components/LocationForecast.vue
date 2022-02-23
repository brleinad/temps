<template>
  <div class="flex items-center space-x-4">
    <div class="flex-1 min-w-0">
      <p class="text-sm font-medium text-gray-900 truncate">
        {{ location.name }}
      </p>
      <ul role="list" class="grid grid-cols-3 gap-6 sm:grid-cols-7">
        <li v-for="dayForecast in location.dayForecasts.slice(0, 3)" :key="dayForecast.dt" class="col-span-1 flex flex-col text-center bg-white rounded-lg shadow divide-y divide-gray-200">
          <DayForecast :dayForecast="dayForecast"/>
        </li>
      </ul>
    </div>
  </div>
</template>

<script>
import DayForecast from './DayForecast.vue';
export default {
  name: 'LocationForecast',
  components: {DayForecast},
  props: {
    location: {
      required: true,
    },
  },
};
</script>
