<template>
  <div class="p-6">
    <label for="search" class="block text-sm font-medium text-gray-100">Search</label>
    <div class="mt-1 relative rounded-md shadow-sm">
      <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
        <SearchIcon class="h-5 w-5 text-gray-400" aria-hidden="true" />
      </div>
      <input type="text" name="search" id="search"
             class="focus:ring-indigo-500 focus:border-indigo-500 block w-full pl-10 sm:text-sm border-gray-300 rounded-md"
             placeholder="Squamish" />
    </div>
  </div>
</template>

<script>
import SearchIcon from '../assets/SearchIcon.svg?component';
export default {
  name: 'SearchBar',
  components: {SearchIcon}
};
</script>
