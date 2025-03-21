<template>
  <PageContainer title="Search">
    <div class="form-control mb-4 flex flex-row">
      <input type="text" v-model="query" class="input input-bordered flex-grow" placeholder="Search for books..." @keyup.enter="searchBooks" />
      <button class="btn btn-square btn-primary ml-2" @click="searchBooks">
        <MagnifyingGlassIcon class="size-6"/>
      </button>
    </div>
    <div v-if="loading" class="flex justify-center flex-grow">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
    <div v-else-if="books.length" class="grid gap-4 text-white overflow-y-auto flex-grow" style="grid-template-columns: subgrid;">
      <div v-for="book in books" :key="book.id" class="flex items-start gap-4 cursor-pointer" @click="viewBookDetail(book.id)">
        <img :src="book.volumeInfo.imageLinks?.thumbnail" alt="Book cover" class="w-24 object-cover" />
        <div>
          <h3 class="font-bold">{{ book.volumeInfo.title }}</h3>
          <p>{{ book.volumeInfo.authors?.join(', ') }}</p>
        </div>
      </div>
    </div>
    <div v-else class="text-white text-center flex-grow">No books found.</div>
  </PageContainer>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { MagnifyingGlassIcon } from "@heroicons/vue/16/solid";
import { searchBooks } from '@/api/googleBooksApi';
import PageContainer from '@/components/PageContainer.vue';

export default defineComponent({
  components: { MagnifyingGlassIcon, PageContainer },
  setup() {
    const query = ref('');
    const books = ref<Array<any>>([]);
    const loading = ref(false);
    const router = useRouter();
    const route = useRoute();

    const searchBooksWrapper = async () => {
      if (!query.value.trim()) {
        return;
      }
      loading.value = true;
      books.value = await searchBooks(query.value);
      loading.value = false;
    };

    const viewBookDetail = (id: string) => {
      router.push({ name: 'search-detail', params: { id } });
    };

    onMounted(() => {
      const savedQuery = route.query.q as string;
      if (savedQuery) {
        query.value = savedQuery;
        searchBooksWrapper();
      }
    });

    return {
      query,
      books,
      loading,
      searchBooks: searchBooksWrapper,
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
