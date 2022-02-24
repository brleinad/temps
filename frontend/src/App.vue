<script setup>
import ForecastDisplays from "./components/ForecastsDisplay.vue";
import SearchBar from './components/SearchBar.vue';
</script>

<template>
  <main class="bg-gray-900 h-vfull">
    <SearchBar :apiUrl="apiUrl"/>
    <ForecastDisplays :apiUrl="apiUrl"/>
  </main>
</template>


<script>

let apiUrl = 'http://0.0.0.0:8088';
if (window.location.href.indexOf('temps') > -1) {
  apiUrl = 'https://temps-backend.onrender.com';
}

export default {
  data() {
    return {
      apiUrl,
    }
  },
}
</script>