<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
    <div class="bg-gray-800 p-6 rounded-lg shadow-md w-full max-w-md">
      <h3 class="text-xl font-semibold text-white mb-4">Select a Shelf</h3>
      <div v-if="loadingShelves" class="flex justify-center">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
      <ul v-else-if="shelves.length" class="list-disc list-inside text-white">
        <li v-for="shelf in shelves" :key="shelf.id" @click="addBookToShelf(shelf.id)" class="cursor-pointer">
          {{ shelf.name }} - {{ shelf.description }}
        </li>
      </ul>
      <div v-else class="text-white text-center">No shelves found.</div>
      <button @click="$emit('close')" class="mt-4 text-white">Close</button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import type { PropType } from 'vue';
import { apiFetch } from '@/api/client';

export default defineComponent({
  props: {
    book: {
      type: Object as PropType<any>,
      required: true,
    },
  },
  setup(props, { emit }) {
    const shelves = ref<Array<{ id: string, name: string, description: string }>>([]);
    const loadingShelves = ref(false);

    const fetchShelves = async () => {
      loadingShelves.value = true;
      try {
        const response = await apiFetch('/api/shelves', { method: 'POST' });
        if (response.ok) {
          const data = await response.json();
          shelves.value = data.shelves;
        } else {
          console.error('Failed to fetch shelves:', await response.json());
        }
      } catch (error) {
        console.error('Failed to fetch shelves:', error);
      } finally {
        loadingShelves.value = false;
      }
    };

    const addBookToShelf = async (shelfId: string) => {
      try {
        const response = await apiFetch('/api/shelves/add-book', {
          method: 'POST',
          body: JSON.stringify({
            shelf_id: shelfId,
            title: props.book.volumeInfo.title,
            author: props.book.volumeInfo.authors?.join(', '),
            isbn13: props.book.volumeInfo.industryIdentifiers?.find((id: any) => id.type === 'ISBN_13')?.identifier,
            isbn10: props.book.volumeInfo.industryIdentifiers?.find((id: any) => id.type === 'ISBN_10')?.identifier,
            google_books_id: props.book.id,
          }),
        });
        if (response.ok) {
          emit('toast', { message: 'Book added to shelf successfully.', type: 'alert-success' });
          emit('close');
        } else {
          console.error('Failed to add book to shelf:', await response.json());
          emit('toast', { message: 'Failed to add book to shelf.', type: 'alert-error' });
        }
      } catch (error) {
        console.error('Failed to add book to shelf:', error);
        emit('toast', { message: 'Failed to add book to shelf.', type: 'alert-error' });
      }
    };

    onMounted(fetchShelves);

    return { shelves, loadingShelves, addBookToShelf };
  },
});
</script>
