<template>
  <div class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50">
    <div class="bg-white rounded-lg p-6 w-full max-w-md">
      <h2 class="text-xl font-bold mb-4">Track Progress</h2>
      <form @submit.prevent.stop="submitForm">
        <div class="mb-4">
          <label for="progress" class="block text-sm font-medium text-gray-700">Page Number</label>
          <input type="number" v-model="progress" id="progress" class="input input-bordered w-full" ref="progressInput" required />
        </div>
        <div class="mb-4">
          <label for="readAt" class="block text-sm font-medium text-gray-700">Date</label>
          <input type="date" v-model="readAt" id="readAt" class="input input-bordered w-full" required />
        </div>
        <div class="flex justify-end">
          <button type="button" @click="$emit('close')" class="btn btn-secondary mr-2">Cancel</button>
          <button type="submit" class="btn btn-primary">Submit</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, watch } from 'vue';

export default defineComponent({
  props: {
    initialProgress: {
      type: Number,
      required: true,
    },
  },
  setup(props, { emit }) {
    const progress = ref(props.initialProgress);
    const readAt = ref(new Date().toISOString().split('T')[0]); // Initialize with current date
    const progressInput = ref<HTMLInputElement | null>(null);

    const submitForm = () => {
      emit('submit', progress.value, readAt.value);
    };

    watch(() => props.initialProgress, (newVal) => {
      progress.value = newVal;
    });

    onMounted(() => {
      if (progressInput.value) {
        progressInput.value.focus();
        progressInput.value.select();
      }
    });

    return {
      progress,
      readAt,
      submitForm,
      progressInput,
    };
  },
});
</script>
