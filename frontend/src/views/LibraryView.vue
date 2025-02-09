<template>
  <div class="dark min-h-screen flex items-center justify-center bg-gray-900">
    <div class="w-full max-w-md bg-gray-800 rounded-lg shadow-md p-6">
      <h2 class="text-2xl font-semibold text-white text-center mb-4">Library</h2>
      <div v-if="loading" class="flex justify-center">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
      <ul v-else-if="shelves.length" class="list-disc list-inside text-white">
        <li v-for="shelf in shelves" :key="shelf.id">
          {{ shelf.name }} - {{ shelf.description }}
        </li>
      </ul>
      <div v-else class="text-white text-center">No shelves found.</div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  data() {
    return {
      shelves: [] as Array<{ id: string, name: string, description: string }>,
      loading: true,
    };
  },
  async created() {
    const userId = localStorage.getItem('user_id');
    if (userId) {
      try {
        const response = await fetch('http://localhost:3000/api/shelves', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ user_id: userId })
        });
        if (response.ok) {
          const data = await response.json();
          this.shelves = data.shelves;
        } else {
          console.error('Failed to fetch shelves:', await response.json());
        }
      } catch (error) {
        console.error('Failed to fetch shelves:', error);
      } finally {
        this.loading = false;
      }
    } else {
      this.loading = false;
    }
  }
});
</script>
