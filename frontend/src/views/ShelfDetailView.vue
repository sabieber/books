<template>
  <div class="dark min-h-screen flex items-center justify-center bg-gray-900">
    <div class="w-full max-w-md bg-gray-800 rounded-lg shadow-md p-6">
      <h2 class="text-2xl font-semibold text-white mb-2">{{ shelf.name }}</h2>
      <p class="text-sm text-gray-400 mb-4">{{ shelf.description }}</p>
      <div v-if="loading" class="flex justify-center">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
      <ul v-else-if="books.length" class="space-y-4">
        <li v-for="book in books" :key="book.id" class="p-4 bg-gray-700 rounded-lg hover:bg-gray-600 transition">
          <div class="flex justify-between items-center">
            <h3 class="text-xl font-bold text-white">{{ book.title }}</h3>
            <button @click="removeBookFromShelf(book.id)" class="btn btn-circle btn-sm btn-error">
              <MinusIcon class="size-3 text-white"/>
            </button>
          </div>
          <p class="text-sm text-gray-400">{{ book.author }}</p>
        </li>
      </ul>
      <div v-else class="text-white text-center">No books found.</div>
    </div>
    <div v-if="toastMessage" class="toast toast-top toast-center">
      <div :class="`alert ${toastType}`">
        <span>{{ toastMessage }}</span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent, onMounted, ref} from 'vue';
import {useRoute} from 'vue-router';
import {MinusIcon} from "@heroicons/vue/16/solid";

export default defineComponent({
  components: {MinusIcon},
  setup() {
    const route = useRoute();
    const books = ref([]);
    const loading = ref(true);
    const shelf = ref({name: '', description: ''});
    const toastMessage = ref('');
    const toastType = ref('');

    const fetchShelfBooks = async (shelfId: string) => {
      try {
        const response = await fetch('http://localhost:3000/api/shelves/books', {
          method: 'POST',
          headers: {'Content-Type': 'application/json'},
          body: JSON.stringify({shelf_id: shelfId}),
        });
        if (response.ok) {
          const data = await response.json();
          books.value = data.books;
          shelf.value = data.shelf;
        } else {
          console.error('Failed to fetch books:', await response.json());
        }
      } catch (error) {
        console.error('Failed to fetch books:', error);
      } finally {
        loading.value = false;
      }
    };

    const removeBookFromShelf = async (bookId: string) => {
      try {
        const response = await fetch('http://localhost:3000/api/shelves/remove-book', {
          method: 'POST',
          headers: {'Content-Type': 'application/json'},
          body: JSON.stringify({book_id: bookId}),
        });
        if (response.ok) {
          toastMessage.value = 'Book removed from shelf successfully.';
          toastType.value = 'alert-success';
          books.value = books.value.filter(book => book.id !== bookId);
        } else {
          console.error('Failed to remove book from shelf:', await response.json());
          toastMessage.value = 'Failed to remove book from shelf.';
          toastType.value = 'alert-error';
        }
      } catch (error) {
        console.error('Failed to remove book from shelf:', error);
        toastMessage.value = 'Failed to remove book from shelf.';
        toastType.value = 'alert-error';
      } finally {
        setTimeout(() => {
          toastMessage.value = '';
        }, 3000);
      }
    };

    onMounted(() => {
      const shelfId = route.params.id as string;
      fetchShelfBooks(shelfId);
    });

    return {
      books,
      loading,
      shelf,
      toastMessage,
      toastType,
      removeBookFromShelf,
    };
  },
});
</script>
