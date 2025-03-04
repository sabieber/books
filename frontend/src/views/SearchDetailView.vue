<template>
  <PageContainer :title="book?.volumeInfo?.title ?? 'Book'" ref="pageContainer">
    <template #title-button>
      <button @click="showPopup = true" class="btn btn-circle btn-primary text-white">
        <PlusIcon class="size-6 text-white"/>
      </button>
    </template>
    <div v-if="loading" class="flex justify-center">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
    <div v-else-if="book" class="text-white">
      <img :src="book.volumeInfo.imageLinks?.thumbnail" alt="Book cover" class="w-24 h-32 object-cover mb-4" />
      <p class="mb-2">{{ book.volumeInfo.authors?.join(', ') }}</p>
      <p class="mb-2">{{ formatDate(book.volumeInfo.publishedDate) }}</p>
      <p class="mb-2" v-html="book.volumeInfo.description"></p>
    </div>
    <div v-else class="text-white text-center">Book not found.</div>
    <AddToShelfPopup v-if="showPopup" @close="showPopup = false" @toast="showToast" :book="book" />
  </PageContainer>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { PlusIcon } from "@heroicons/vue/16/solid";
import AddToShelfPopup from '@/components/AddToShelfPopup.vue';
import { fetchBookDetails } from '@/api/googleBooksApi';
import PageContainer from '@/components/PageContainer.vue';
import moment from 'moment';

export default defineComponent({
  components: { PlusIcon, AddToShelfPopup, PageContainer },
  setup() {
    const route = useRoute();
    const book = ref(null);
    const loading = ref(true);
    const showPopup = ref(false);
    const pageContainer = ref(null);

    const fetchBookDetailsWrapper = async (bookId: string) => {
      book.value = await fetchBookDetails(bookId);
      loading.value = false;
    };

    const showToast = ({ message, type }) => {
      pageContainer.value.showToast({ message, type });
    };

    const formatDate = (date: string) => {
      return moment(date).format('LL');
    };

    onMounted(() => {
      const bookId = route.params.id as string;
      fetchBookDetailsWrapper(bookId);
    });

    return {
      book,
      loading,
      showPopup,
      showToast,
      formatDate,
      pageContainer,
    };
  },
});
</script>
