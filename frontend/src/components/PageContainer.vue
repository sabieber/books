<template>
  <div class="dark min-h-screen flex flex-col items-center justify-center bg-gray-900">
    <div class="w-full max-w-lg rounded-lg shadow-md p-6 flex flex-col flex-grow">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-2xl font-semibold text-white">{{ title }}</h2>
        <slot name="title-button"></slot>
      </div>
      <slot></slot>
    </div>
    <Toast ref="toast" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Toast from '@/components/Toast.vue';

export default defineComponent({
  components: { Toast },
  props: {
    title: {
      type: String,
      required: true
    }
  }
});
</script>
