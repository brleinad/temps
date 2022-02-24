<template>
  <div class="flex-1 flex flex-col p-4">
    <h3 class="mt-3 text-white text-sm font-medium">{{ day }}</h3>
    <img class="flex-shrink-0 mx-auto rounded-full" :src="iconUrl" alt="" />
    <span class="text-slate-50 text-sm">{{ kelvin2Celsius(dayForecast.temp.day) }} &deg;</span>
  </div>
</template>

<script>
export default {
  name: 'DayForecast',
  props: {
    dayForecast: {
      required: true,
    },
  },
  computed: {
    day() {
      const day = new Date(this.dayForecast.dt * 1000);
      return day.toDateString().split(' ')[0];
    },
    iconUrl() {
      return `http://openweathermap.org/img/wn/${this.dayForecast.weather[0].icon}@2x.png`;
    }
  },
  methods: {
    kelvin2Celsius(kelvin) {
      return Math.round(kelvin - 273.15);
    },
  },
};
</script>
