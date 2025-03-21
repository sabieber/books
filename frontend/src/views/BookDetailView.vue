<template>
  <PageContainer :title="book?.volumeInfo?.title ?? 'Book'" ref="pageContainer">
    <div v-if="loading" class="flex justify-center">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
    <div v-else-if="book" class="text-white">
      <img :src="book.volumeInfo.imageLinks?.thumbnail" alt="Book cover" class="w-24 h-32 object-cover mb-4" />
      <p class="mb-2">{{ book.volumeInfo.authors?.join(', ') }}</p>
      <p class="mb-2">{{ formatDate(book.volumeInfo.publishedDate) }}</p>
      <p class="mb-2" v-html="book.volumeInfo.description"></p>
      <button @click="showStartReadingModal = true" class="btn btn-primary mt-4">Start Reading</button>
      <div v-if="readings.length" class="mt-4">
        <h3 class="text-xl font-semibold mb-2">Readings</h3>
        <ul class="space-y-2">
          <li v-for="reading in readings" :key="reading.id" class="p-2 bg-gray-700 rounded-lg hover:bg-gray-600 transition cursor-pointer" @click="viewReadingDetail(reading.id)">
            <div class="flex justify-between items-center">
              <span>{{ reading.started_at }} - {{ reading.finished_at || 'Ongoing' }}</span>
              <span>{{ reading.progress }} / {{ reading.total_pages }} pages</span>
            </div>
          </li>
        </ul>
      </div>
    </div>
    <div v-else class="text-white text-center">Book not found.</div>
    <StartReadingModal v-if="showStartReadingModal" @close="showStartReadingModal = false" @submit="startReadingSession" :initialPages="book?.volumeInfo.pageCount || 0" />
  </PageContainer>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { PlusIcon } from "@heroicons/vue/16/solid";
import { fetchBookDetails } from '@/api/googleBooksApi';
import StartReadingModal from '@/components/StartReadingModal.vue';
import PageContainer from '@/components/PageContainer.vue';
import moment from 'moment';

export default defineComponent({
  components: { PlusIcon, StartReadingModal, PageContainer },
  setup() {
    const route = useRoute();
    const router = useRouter();
    const book = ref(null);
    const readings = ref([]);
    const loading = ref(true);
    const showStartReadingModal = ref(false);
    const pageContainer = ref(null);

    const fetchBookInfo = async (bookId: string) => {
      try {
        const response = await fetch('http://localhost:3000/api/books/info', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ book_id: bookId }),
        });
        if (response.ok) {
          const data = await response.json();
          readings.value = data.readings;
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

    const viewReadingDetail = (readingId: string) => {
      router.push({ name: 'reading-detail', params: { id: readingId } });
    };

    const startReadingSession = async (totalPages: number) => {
      try {
        const userId = localStorage.getItem('user_id');
        const response = await fetch('http://localhost:3000/api/books/start-reading', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ book_id: route.params.id, user_id: userId, total_pages: totalPages }),
        });
        if (response.ok) {
          fetchBookDetailsWrapper(route.params.id as string);
          showStartReadingModal.value = false;
        } else {
          const errorData = await response.json();
          pageContainer.value.showToast({ message: errorData.error, type: 'alert-error' });
        }
      } catch (error) {
        pageContainer.value.showToast({ message: 'Failed to start reading session.', type: 'alert-error' });
      }
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
      readings,
      loading,
      viewReadingDetail,
      startReadingSession,
      showStartReadingModal,
      formatDate,
      pageContainer,
    };
  },
});
</script>
