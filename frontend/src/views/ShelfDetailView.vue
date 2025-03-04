<template>
  <PageContainer :title="shelf.name" ref="pageContainer">
    <p class="text-sm text-gray-400 mb-4">{{ shelf.description }}</p>
    <div v-if="loading" class="flex justify-center">
      <span class="loading loading-spinner loading-lg"></span>
    </div>
    <ul v-else-if="books.length" class="space-y-4">
      <li v-for="book in books" :key="book.id" class="p-4 bg-gray-700 rounded-lg hover:bg-gray-600 transition cursor-pointer" @click="viewBookDetail(book.id)">
        <div class="flex justify-between items-center">
          <h3 class="text-xl font-bold text-white">{{ book.title }}</h3>
          <button @click.stop="removeBookFromShelf(book.id)" class="btn btn-circle btn-sm btn-error">
            <MinusIcon class="size-3 text-white"/>
          </button>
        </div>
        <p class="text-sm text-gray-400">{{ book.author }}</p>
      </li>
    </ul>
    <div v-else class="text-white text-center">No books found.</div>
  </PageContainer>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { MinusIcon } from "@heroicons/vue/16/solid";
import PageContainer from '@/components/PageContainer.vue';

export default defineComponent({
  components: { MinusIcon, PageContainer },
  setup() {
    const route = useRoute();
    const router = useRouter();
    const books = ref([]);
    const loading = ref(true);
    const shelf = ref({ name: '', description: '' });
    const pageContainer = ref(null);

    const fetchShelfBooks = async (shelfId: string) => {
      try {
        const response = await fetch('http://localhost:3000/api/shelves/books', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ shelf_id: shelfId }),
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
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ book_id: bookId }),
        });
        if (response.ok) {
          pageContainer.value.showToast({ message: 'Book removed from shelf successfully.', type: 'alert-success' });
          books.value = books.value.filter(book => book.id !== bookId);
        } else {
          console.error('Failed to remove book from shelf:', await response.json());
          pageContainer.value.showToast({ message: 'Failed to remove book from shelf.', type: 'alert-error' });
        }
      } catch (error) {
        console.error('Failed to remove book from shelf:', error);
        pageContainer.value.showToast({ message: 'Failed to remove book from shelf.', type: 'alert-error' });
      }
    };

    const viewBookDetail = (id: string) => {
      router.push({ name: 'book-detail', params: { id } });
    };

    onMounted(() => {
      const shelfId = route.params.id as string;
      fetchShelfBooks(shelfId);
    });

    return {
      books,
      loading,
      shelf,
      removeBookFromShelf,
      viewBookDetail,
      pageContainer,
    };
  },
});
</script>
