<script setup>
import ForecastDisplays from "./components/ForecastsDisplay.vue";
import SearchBar from './components/SearchBar.vue';
</script>

<template>
  <main class="bg-gray-900 h-vfull">
    <SearchBar :apiUrl="apiUrl" @addedLocation="handleNewLocation"/>
    <ForecastDisplays :apiUrl="apiUrl" :savedLocations="savedLocations" @deleteLocation="deleteLocation"/>
  </main>
</template>


<script>

import { getSavedLocations } from './helpers';

let apiUrl = 'http://0.0.0.0:8088';
if (window.location.href.indexOf('temps') > -1) {
  apiUrl = 'https://temps-backend.onrender.com';
}

export default {
  data() {
    return {
      apiUrl,
      savedLocations: [],
    }
  },
  mounted() {
    this.savedLocations = getSavedLocations();
    fetch(`${this.apiUrl}/`); // wake up the backend
  },
  methods: {
    handleNewLocation() {
      this.savedLocations = getSavedLocations();
      console.log(this.savedLocations)
    },
    deleteLocation(deletedLocation) {
      // TODO: avoid reloading everything on delete
      this.savedLocations = this.savedLocations.filter(
        // TODO: make this more robust
        (location) => !(deletedLocation.name === location.name && deletedLocation.country === location.country));
      localStorage.setItem('savedLocations', JSON.stringify(this.savedLocations));
    },
  },
}
</script>