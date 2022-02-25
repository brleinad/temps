<template>
  <Combobox as="div" v-model="selectedLocation" class="p-6 max-w-md">
    <ComboboxLabel class="block text-sm font-medium text-gray-100"
      ></ComboboxLabel
    >
    <div class="relative mt-1">
      <ComboboxInput
        class="w-full rounded-md border border-gray-300 bg-white py-2 pl-3 pr-10 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 sm:text-sm"
        @change="getLocations($event.target.value)"
      />
      <div
        class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none"
      >
        <SearchIcon class="h-5 w-5 text-gray-400" aria-hidden="true" />
      </div>

      <ComboboxOptions
        v-if="foundLocations.length > 0"
        class="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm"
      >
        <ComboboxOption
          v-for="location in foundLocations"
          :key="location.lat + location.lon"
          :value="location"
          as="template"
          v-slot="{ active, selected }"
          @click="addLocation(location)"
        >
          <li
            :class="[
              'relative cursor-default select-none py-2 pl-3 pr-9',
              active ? 'bg-indigo-600 text-white' : 'text-gray-900',
            ]"
          >
            <div class="flex">
              <span :class="['truncate', selected && 'font-semibold']">
                {{ location.name }}
              </span>
              <span
                :class="[
                  'ml-2 truncate text-gray-500',
                  active ? 'text-indigo-200' : 'text-gray-500',
                ]"
              >
                {{ location.country }}
              </span>
            </div>
          </li>
        </ComboboxOption>
      </ComboboxOptions>
    </div>
  </Combobox>
</template>

<script>
import { getSavedLocations } from '../helpers';
import SearchIcon from "../assets/SearchIcon.svg?component";
import {
  Combobox,
  ComboboxInput,
  ComboboxLabel,
  ComboboxOption,
  ComboboxOptions,
} from "@headlessui/vue";
export default {
  name: "SearchBar",
  components: {
    Combobox,
    ComboboxInput,
    ComboboxLabel,
    ComboboxOption,
    ComboboxOptions,
    SearchIcon,
  },
  props: {
    apiUrl: {
      required: true,
      type: String,
    },
  },
  data() {
    return {
      selectedLocation: null,
      foundLocations: [],
      query: "",
    };
  },
  mounted() {},
  methods: {
    async getLocations(locationName) {
      if (locationName.length < 3) {
        this.foundLocations = [];
        return;
      }
      const response = await fetch(`${this.apiUrl}/locations/${locationName}`);
      this.foundLocations = await response.json();
    },
    addLocation(location) {
      let savedLocations = getSavedLocations();
      savedLocations.push(location);
      localStorage.setItem('savedLocations', JSON.stringify(savedLocations));
      // TODO: avoid duplicates
      this.$emit('addedLocation');
    },
  },
};
</script>
