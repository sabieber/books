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
import {defineComponent, onMounted, ref, PropType} from 'vue';

export default defineComponent({
  props: {
    book: {
      type: Object as PropType<any>,
      required: true,
    },
  },
  setup(props, {emit}) {
    const shelves = ref([]);
    const loadingShelves = ref(false);

    const fetchShelves = async () => {
      loadingShelves.value = true;
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
            shelves.value = data.shelves;
          } else {
            console.error('Failed to fetch shelves:', await response.json());
          }
        } catch (error) {
          console.error('Failed to fetch shelves:', error);
        } finally {
          loadingShelves.value = false;
        }
      } else {
        loadingShelves.value = false;
      }
    };

    const addBookToShelf = async (shelfId: string) => {
      const userId = localStorage.getItem('user_id');
      if (userId && props.book) {
        try {
          const response = await fetch('http://localhost:3000/api/shelves/add-book', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({
              user_id: userId,
              shelf_id: shelfId,
              title: props.book.volumeInfo.title,
              author: props.book.volumeInfo.authors?.join(', '),
              isbn13: props.book.volumeInfo.industryIdentifiers?.find(id => id.type === 'ISBN_13')?.identifier,
              isbn10: props.book.volumeInfo.industryIdentifiers?.find(id => id.type === 'ISBN_10')?.identifier,
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
      }
    };

    onMounted(() => {
      fetchShelves();
    });

    return {
      shelves,
      loadingShelves,
      addBookToShelf,
    };
  },
});
</script>
