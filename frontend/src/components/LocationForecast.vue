<template>
  <div class="flex items-center space-x-4">
    <div class="flex-1 min-w-0">
      <div class="flex items-center justify-between">
        <p class="text-sm font-medium text-white truncate pb-3 text-lg">
          {{ location.name }}
        </p>
        <span class="text-sm" @click="$emit('deleteLocation', location)">
        <DeleteIcon class="h-5 w-5 text-slate-50"/>
      </span>
      </div>
      <ul role="list" class="grid grid-cols-4 gap-3 sm:grid-cols-7">
        <li v-for="dayForecast in location.daily.slice(0, 4)" :key="dayForecast.dt"
          class="col-span-1 flex flex-col text-center rounded-lg shadow divide-y divide-gray-200 bg-blue-400"
        >
          <DayForecast :dayForecast="dayForecast"/>
        </li>
      </ul>
    </div>
  </div>
</template>

<script>
import DayForecast from './DayForecast.vue';
import DeleteIcon from "../assets/DeleteIcon.svg?component";

export default {
  name: 'LocationForecast',
  components: {DayForecast, DeleteIcon},
  props: {
    location: {
      required: true,
    },
  },
};
</script>
