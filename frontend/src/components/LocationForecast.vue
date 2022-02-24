<template>
  <div class="flex items-center space-x-4">
    <div class="flex-1 min-w-0">
      <p class="text-sm font-medium text-white truncate pb-3 text-lg">
        {{ location.name }}
      </p>
      <ul role="list" class="grid grid-cols-4 gap-3 sm:grid-cols-7">
        <li v-for="dayForecast in location.daily.slice(0, 4)" :key="dayForecast.dt" class="col-span-1 flex flex-col text-center rounded-lg shadow divide-y divide-gray-200 bg-blue-900">
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
