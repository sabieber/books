<template>
  <PageContainer title="Profile" ref="pageContainer">
    <template #title-button>
      <button @click="logout" class="btn btn-circle btn-error text-white">
        <PowerIcon class="size-6 text-white"/>
      </button>
    </template>
    <div class="mt-4">
      <h3 class="text-xl font-semibold text-white">Import GoodReads CSV</h3>
      <p class="text-sm text-gray-400 mb-2">Select a CSV file to import your GoodReads data.</p>
      <div class="flex items-center space-x-2">
        <input type="file" accept=".csv" @change="handleFileChange" class="file-input file-input-bordered w-full max-w-xs" />
        <button @click="uploadFile" class="btn btn-primary" :disabled="isUploading">
          <span v-if="isUploading" class="loading loading-spinner loading-sm"></span>
          <span v-else>Upload</span>
        </button>
      </div>
    </div>
  </PageContainer>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { useRouter } from 'vue-router';
import PageContainer from '@/components/PageContainer.vue';
import { PowerIcon } from "@heroicons/vue/16/solid";
import { apiFetch } from '@/api/client';
import { useAuthStore } from '@/stores/auth';

export default defineComponent({
  components: { PageContainer, PowerIcon },
  setup() {
    const router = useRouter();
    const pageContainer = ref<any>(null);
    const selectedFile = ref<File | null>(null);
    const isUploading = ref(false);
    const auth = useAuthStore();

    const logout = () => {
      auth.logout();
      router.push('/login');
    };

    const handleFileChange = (event: Event) => {
      const input = event.target as HTMLInputElement;
      if (input.files && input.files[0]) {
        selectedFile.value = input.files[0];
      }
    };

    const uploadFile = async () => {
      if (!selectedFile.value) {
        pageContainer.value.showToast({ message: 'Please select a file first.', type: 'alert-warning' });
        return;
      }

      isUploading.value = true;
      const formData = new FormData();
      formData.append('file', selectedFile.value);

      try {
        const response = await apiFetch('/api/user/import-good-reads', {
          method: 'POST',
          body: formData,
        });

        if (response.ok) {
          const data = await response.json();
          pageContainer.value.showToast({ message: data.message, type: 'alert-success' });
        } else {
          console.error('Failed to import file:', await response.json());
          pageContainer.value.showToast({ message: 'Failed to import file.', type: 'alert-error' });
        }
      } catch (error) {
        console.error('Failed to import file:', error);
        pageContainer.value.showToast({ message: 'Failed to import file.', type: 'alert-error' });
      } finally {
        isUploading.value = false;
      }
    };

    return { logout, pageContainer, handleFileChange, uploadFile, isUploading };
  }
});
</script>
