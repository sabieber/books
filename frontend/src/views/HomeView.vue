<template>
  <PageContainer title="Home">
    <h2 class="text-2xl font-semibold text-white text-center">This will become the home page!</h2>
  </PageContainer>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import PageContainer from '@/components/PageContainer.vue';

export default defineComponent({
  components: { PageContainer },
});
</script>