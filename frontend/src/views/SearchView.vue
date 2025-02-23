<template>
  <div class="dark min-h-screen flex items-center justify-center bg-gray-900">
    <div class="w-full max-w-md bg-gray-800 rounded-lg shadow-md p-6">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-2xl font-semibold text-white">Search</h2>
      </div>
      <div class="form-control mb-4 flex flex-row">
        <input type="text" v-model="query" class="input input-bordered flex-grow" placeholder="Search for books..." @keyup.enter="searchBooks" />
        <button class="btn btn-square btn-primary ml-2" @click="searchBooks">
          <MagnifyingGlassIcon class="size-6"/>
        </button>
      </div>
      <div v-if="loading" class="flex justify-center">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
      <div v-else-if="books.length" class="grid gap-4 text-white" style="grid-template-columns: subgrid;">
        <div v-for="book in books" :key="book.id" class="flex items-start gap-4 cursor-pointer" @click="viewBookDetail(book.id)">
          <img :src="book.volumeInfo.imageLinks?.thumbnail" alt="Book cover" class="w-24 object-cover" />
          <div>
            <h3 class="font-bold">{{ book.volumeInfo.title }}</h3>
            <p>{{ book.volumeInfo.authors?.join(', ') }}</p>
          </div>
        </div>
      </div>
      <div v-else class="text-white text-center">No books found.</div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { MagnifyingGlassIcon } from "@heroicons/vue/16/solid";

export default defineComponent({
  components: { MagnifyingGlassIcon },
  setup() {
    const query = ref('');
    const books = ref<Array<any>>([]);
    const loading = ref(false);
    const router = useRouter();
    const route = useRoute();

    const searchBooks = async () => {
      if (!query.value.trim()) {
        return;
      }
      loading.value = true;
      try {
        const response = await fetch(`https://www.googleapis.com/books/v1/volumes?q=${encodeURIComponent(query.value)}`);
        if (response.ok) {
          const data = await response.json();
          books.value = data.items || [];
        } else {
          console.error('Failed to fetch books:', await response.json());
        }
      } catch (error) {
        console.error('Failed to fetch books:', error);
      } finally {
        loading.value = false;
      }
    };

    const viewBookDetail = (id: string) => {
      router.push({ name: 'search-detail', params: { id } });
    };

    onMounted(() => {
      const savedQuery = route.query.q as string;
      if (savedQuery) {
        query.value = savedQuery;
        searchBooks();
      }
    });

    return {
      query,
      books,
      loading,
      searchBooks,
      viewBookDetail,
    };
  },
});
</script>

<style scoped>
.grid {
  display: grid;
  grid-template-columns: 1fr 3fr;
}
</style>
