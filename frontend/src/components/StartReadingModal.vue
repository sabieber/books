<template>
  <div class="fixed inset-0 flex items-center justify-center bg-black bg-opacity-50">
    <div class="bg-white rounded-lg p-6 w-full max-w-md">
      <h2 class="text-xl font-bold mb-4">Start Reading</h2>
      <form @submit.prevent="submitForm">
        <div class="mb-4">
          <label for="totalPages" class="block text-sm font-medium text-gray-700">Total Pages</label>
          <input type="number" v-model="totalPages" id="totalPages" class="input input-bordered w-full" required />
        </div>
        <div class="flex justify-end">
          <button type="button" @click="$emit('close')" class="btn btn-secondary mr-2">Cancel</button>
          <button type="submit" class="btn btn-primary">Start</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';

export default defineComponent({
  props: {
    initialPages: {
      type: Number,
      required: true,
    },
  },
  setup(props, { emit }) {
    const totalPages = ref(props.initialPages);

    const submitForm = () => {
      emit('submit', totalPages.value);
    };

    return {
      totalPages,
      submitForm,
    };
  },
});
</script>
