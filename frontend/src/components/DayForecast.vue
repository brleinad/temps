<template>
        <div class="flex-1 flex flex-col p-4">
        <h3 class="mt-3 text-gray-900 text-sm font-medium">{{ day }}</h3>
          <!--        <img class="w-32 h-32 flex-shrink-0 mx-auto rounded-full" :src="dayForecast.icon" alt="" />-->
          <span class="text-gray-500 text-sm">{{  dayForecast.aveTemp }}</span>
      </div>
<!--      <div>-->
<!--        <div class="-mt-px flex divide-x divide-gray-200">-->
<!--          <div class="w-0 flex-1 flex">-->
<!--            <a :href="`mailto:${person.email}`" class="relative -mr-px w-0 flex-1 inline-flex items-center justify-center py-4 text-sm text-gray-700 font-medium border border-transparent rounded-bl-lg hover:text-gray-500">-->
<!--              <MailIcon class="w-5 h-5 text-gray-400" aria-hidden="true" />-->
<!--              <span class="ml-3">Email</span>-->
<!--            </a>-->
<!--          </div>-->
<!--          <div class="-ml-px w-0 flex-1 flex">-->
<!--            <a :href="`tel:${person.telephone}`" class="relative w-0 flex-1 inline-flex items-center justify-center py-4 text-sm text-gray-700 font-medium border border-transparent rounded-br-lg hover:text-gray-500">-->
<!--              <PhoneIcon class="w-5 h-5 text-gray-400" aria-hidden="true" />-->
<!--              <span class="ml-3">Call</span>-->
<!--            </a>-->
<!--          </div>-->
<!--        </div>-->
<!--      </div>-->
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
    }
  }
};
</script>
