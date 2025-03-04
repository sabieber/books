<template>
  <PageContainer title="Profile" ref="pageContainer">
    <template #title-button>
      <button @click="logout" class="btn btn-circle btn-error text-white">
        <PowerIcon class="size-6 text-white"/>
      </button>
    </template>
    <h2 class="text-2xl font-semibold text-white text-center">This will become the profile page!</h2>
  </PageContainer>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { useRouter } from 'vue-router';
import PageContainer from '@/components/PageContainer.vue';
import { PowerIcon } from "@heroicons/vue/16/solid";

export default defineComponent({
  components: { PageContainer, PowerIcon },
  setup() {
    const router = useRouter();
    const pageContainer = ref(null);

    const logout = () => {
      localStorage.removeItem('token');
      localStorage.removeItem('user_id');
      router.push('/');
      pageContainer.value.showToast({ message: 'Logged out successfully.', type: 'alert-success' });
    };

    return {
      logout,
      pageContainer,
    };
  }
});
</script>