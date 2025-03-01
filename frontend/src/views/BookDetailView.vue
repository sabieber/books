<template>
  <div class="dark min-h-screen flex items-center justify-center bg-gray-900">
    <div class="w-full max-w-md bg-gray-800 rounded-lg shadow-md p-6 relative">
      <div v-if="loading" class="flex justify-center">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
      <div v-else-if="book" class="text-white">
        <img :src="book.volumeInfo.imageLinks?.thumbnail" alt="Book cover" class="w-24 h-32 object-cover mb-4" />
        <h2 class="text-2xl font-bold mb-2">{{ book.volumeInfo.title }}</h2>
        <p class="mb-2">{{ book.volumeInfo.authors?.join(', ') }}</p>
        <p class="mb-2">{{ book.volumeInfo.publishedDate }}</p>
        <p class="mb-2" v-html="book.volumeInfo.description"></p>
      </div>
      <div v-else class="text-white text-center">Book not found.</div>
    </div>
    <div v-if="toastMessage" class="toast toast-top toast-center">
      <div :class="`alert ${toastType}`">
        <span>{{ toastMessage }}</span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { PlusIcon } from "@heroicons/vue/16/solid";
import AddToShelfPopup from '@/components/AddToShelfPopup.vue';
import { fetchBookDetails } from '@/api/googleBooksApi';

export default defineComponent({
  components: { PlusIcon },
  setup() {
    const route = useRoute();
    const book = ref(null);
    const loading = ref(true);
    const toastMessage = ref('');
    const toastType = ref('');

    const fetchBookInfo = async (bookId: string) => {
      try {
        const response = await fetch('http://localhost:3000/api/books/info', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ book_id: bookId }),
        });
        if (response.ok) {
          const data = await response.json();
          return data.google_books_id;
        } else {
          console.error('Failed to fetch book info:', await response.json());
          return null;
        }
      } catch (error) {
        console.error('Failed to fetch book info:', error);
        return null;
      }
    };

    const fetchBookDetailsWrapper = async (bookId: string) => {
      const googleBooksId = await fetchBookInfo(bookId);
      if (googleBooksId) {
        book.value = await fetchBookDetails(googleBooksId);
      }
      loading.value = false;
    };

    const showToast = ({ message, type }) => {
      toastMessage.value = message;
      toastType.value = type;
      setTimeout(() => {
        toastMessage.value = '';
      }, 3000);
    };

    onMounted(() => {
      const bookId = route.params.id as string;
      fetchBookDetailsWrapper(bookId);
    });

    return {
      book,
      loading,
      toastMessage,
      toastType,
      showToast,
    };
  },
});
</script>
