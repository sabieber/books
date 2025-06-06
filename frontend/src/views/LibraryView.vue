<template>
  <PageContainer title="Library" ref="pageContainer">
    <template #title-button>
      <CreateShelfModal @shelfCreated="fetchShelves"/>
    </template>
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
  </PageContainer>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { MinusIcon } from "@heroicons/vue/16/solid";
import CreateShelfModal from '@/components/CreateShelfModal.vue';
import PageContainer from '@/components/PageContainer.vue';

export default defineComponent({
  components: {
    CreateShelfModal,
    MinusIcon,
    PageContainer,
  },
  setup() {
    const shelves = ref<Array<{ id: string, name: string, description: string }>>([]);
    const loading = ref(true);
    const router = useRouter();
    const pageContainer = ref(null);

    const fetchShelves = async () => {
      const userId = localStorage.getItem('user_id');
      if (userId) {
        try {
          const response = await fetch('http://localhost:3000/api/shelves', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ user_id: userId }),
          });
          if (response.ok) {
            const data = await response.json();
            shelves.value = data.shelves;
          } else {
            console.error('Failed to fetch shelves:', await response.json());
          }
        } catch (error) {
          console.error('Failed to fetch shelves:', error);
        } finally {
          loading.value = false;
        }
      } else {
        loading.value = false;
      }
    };

    const goToShelf = (shelfId: string) => {
      router.push({ name: 'shelf-detail', params: { id: shelfId } });
    };

    const removeShelf = async (shelfId: string) => {
      try {
        const response = await fetch('http://localhost:3000/api/shelves/remove', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ shelf_id: shelfId }),
        });
        if (response.ok) {
          pageContainer.value.showToast({ message: 'Shelf removed successfully.', type: 'alert-success' });
          shelves.value = shelves.value.filter(shelf => shelf.id !== shelfId);
        } else {
          console.error('Failed to remove shelf:', await response.json());
          pageContainer.value.showToast({ message: 'Failed to remove shelf.', type: 'alert-error' });
        }
      } catch (error) {
        console.error('Failed to remove shelf:', error);
        pageContainer.value.showToast({ message: 'Failed to remove shelf.', type: 'alert-error' });
      }
    };

    onMounted(fetchShelves);

    return {
      shelves,
      loading,
      fetchShelves,
      goToShelf,
      removeShelf,
      pageContainer,
    };
  },
});
</script>
