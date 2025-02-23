<template>
  <div class="dark min-h-screen flex items-center justify-center bg-gray-900">
    <div class="w-full max-w-md bg-gray-800 rounded-lg shadow-md p-6 relative">
      <button @click="showPopup = true" class="btn btn-circle btn-primary absolute top-4 right-4 text-white">
        <PlusIcon class="size-6 text-white"/>
      </button>
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
    <AddToShelfPopup v-if="showPopup" @close="showPopup = false" @toast="showToast" :book="book" />
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

export default defineComponent({
  components: { PlusIcon, AddToShelfPopup },
  setup() {
    const route = useRoute();
    const book = ref(null);
    const loading = ref(true);
    const showPopup = ref(false);
    const toastMessage = ref('');
    const toastType = ref('');

    const fetchBookDetails = async (bookId: string) => {
      try {
        const response = await fetch(`https://www.googleapis.com/books/v1/volumes/${bookId}`);
        if (response.ok) {
          book.value = await response.json();
        } else {
          console.error('Failed to fetch book details:', await response.json());
        }
      } catch (error) {
        console.error('Failed to fetch book details:', error);
      } finally {
        loading.value = false;
      }
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
      fetchBookDetails(bookId);
    });

    return {
      book,
      loading,
      showPopup,
      toastMessage,
      toastType,
      showToast,
    };
  },
});
</script>
