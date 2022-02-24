<script setup>
import ForecastDisplays from "./components/ForecastsDisplay.vue";
import SearchBar from './components/SearchBar.vue';
</script>

<template>
  <main class="bg-gray-900 h-vfull">
    <SearchBar/>
    <ForecastDisplays/>
  </main>
</template>
