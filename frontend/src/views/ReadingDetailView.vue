<template>
  <PageContainer title="Reading Entries">
    <div v-if="loading" class="flex justify-center">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
    <div v-else-if="entries.length" class="text-white">
      <ul class="space-y-4">
        <li v-for="entry in entries" :key="entry.id" class="p-4 bg-gray-700 rounded-lg">
          <div class="flex justify-between items-center">
            <span>{{ entry.read_at }}</span>
            <span>{{ entry.progress }} pages</span>
          </div>
          <p class="text-sm text-gray-400">{{ entry.mode }}</p>
        </li>
      </ul>
    </div>
    <div v-else class="text-white text-center">No entries found.</div>
    <button @click="showModal = true" class="btn btn-primary mt-4">Track Progress</button>
    <TrackProgressModal v-if="showModal" @close="showModal = false" @submit="trackProgress" :initialProgress="latestProgress" />
  </PageContainer>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import TrackProgressModal from '@/components/TrackProgressModal.vue';
import PageContainer from '@/components/PageContainer.vue';
import Toast from '@/components/Toast.vue';

export default defineComponent({
  components: { TrackProgressModal, PageContainer, Toast },
  setup() {
    const route = useRoute();
    const bookId = ref('');
    const entries = ref([]);
    const loading = ref(true);
    const showModal = ref(false);
    const latestProgress = ref(0);
    const toast = ref(null);

    const fetchReadingEntries = async (readingId: string) => {
      try {
        const response = await fetch('http://localhost:3000/api/books/reading', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ reading_id: readingId }),
        });
        if (response.ok) {
          const data = await response.json();
          entries.value = data.entries;
          bookId.value = data.book_id;
          if (entries.value.length > 0) {
            latestProgress.value = entries.value[entries.value.length - 1].progress;
          }
        } else {
          console.error('Failed to fetch reading entries:', await response.json());
        }
      } catch (error) {
        console.error('Failed to fetch reading entries:', error);
      } finally {
        loading.value = false;
      }
    };

    const trackProgress = async (progress: number, readAt: string) => {
      try {
        const userId = localStorage.getItem('user_id');
        const response = await fetch('http://localhost:3000/api/books/track-progress', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ reading_id: route.params.id, book_id: bookId.value, user_id: userId, progress, read_at: readAt }),
        });
        if (response.ok) {
          fetchReadingEntries(route.params.id as string);
          showModal.value = false;
        } else {
          const errorData = await response.json();
          toast.value.showToast({ message: errorData.error, type: 'alert-error' });
        }
      } catch (error) {
        toast.value.showToast({ message: 'Failed to track progress.', type: 'alert-error' });
      }
    };

    onMounted(() => {
      const readingId = route.params.id as string;
      fetchReadingEntries(readingId);
    });

    return {
      entries,
      loading,
      showModal,
      trackProgress,
      latestProgress,
      toast,
    };
  },
});
</script>
