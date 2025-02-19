<template>
  <div class="dark min-h-screen flex items-center justify-center bg-gray-900">
    <div class="w-full max-w-md bg-gray-800 rounded-lg shadow-md p-6">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-2xl font-semibold text-white">Library</h2>
        <CreateShelfComponent @shelfCreated="fetchShelves"/>
      </div>
      <div v-if="loading" class="flex justify-center">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
      <ul v-else-if="shelves.length" class="space-y-4">
        <li v-for="shelf in shelves" :key="shelf.id" class="p-4 bg-gray-700 rounded-lg hover:bg-gray-600 transition">
          <div class="flex justify-between items-center cursor-pointer" @click="goToShelf(shelf.id)">
            <div>
              <h3 class="text-xl font-bold text-white">{{ shelf.name }}</h3>
              <p class="text-sm text-gray-400">{{ shelf.description }}</p>
            </div>
            <button @click.stop="removeShelf(shelf.id)" class="btn btn-circle btn-sm btn-error">
              <MinusIcon class="size-3 text-white"/>
            </button>
          </div>
        </li>
      </ul>
      <div v-else class="text-white text-center">No shelves found.</div>
    </div>
    <div v-if="toastMessage" class="toast toast-top toast-center">
      <div :class="`alert ${toastType}`">
        <span>{{ toastMessage }}</span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue';
import {MinusIcon} from "@heroicons/vue/16/solid";
import CreateShelfComponent from '@/components/CreateShelfComponent.vue';

export default defineComponent({
  components: {
    CreateShelfComponent,
    MinusIcon,
  },
  data() {
    return {
      shelves: [] as Array<{ id: string, name: string, description: string }>,
      loading: true,
      toastMessage: '',
      toastType: '',
    };
  },
  async created() {
    await this.fetchShelves();
  },
  methods: {
    async fetchShelves() {
      const userId = localStorage.getItem('user_id');
      if (userId) {
        try {
          const response = await fetch('http://localhost:3000/api/shelves', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({user_id: userId}),
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
    },
    goToShelf(shelfId: string) {
      this.$router.push({ name: 'shelf-detail', params: { id: shelfId } });
    },
    async removeShelf(shelfId: string) {
      try {
        const response = await fetch('http://localhost:3000/api/shelves/remove', {
          method: 'POST',
          headers: {'Content-Type': 'application/json'},
          body: JSON.stringify({shelf_id: shelfId}),
        });
        if (response.ok) {
          this.toastMessage = 'Shelf removed successfully.';
          this.toastType = 'alert-success';
          this.shelves = this.shelves.filter(shelf => shelf.id !== shelfId);
        } else {
          console.error('Failed to remove shelf:', await response.json());
          this.toastMessage = 'Failed to remove shelf.';
          this.toastType = 'alert-error';
        }
      } catch (error) {
        console.error('Failed to remove shelf:', error);
        this.toastMessage = 'Failed to remove shelf.';
        this.toastType = 'alert-error';
      } finally {
        setTimeout(() => {
          this.toastMessage = '';
        }, 3000);
      }
    },
  },
});
</script>
