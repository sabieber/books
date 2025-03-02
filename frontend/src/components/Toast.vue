<template>
  <div v-if="message" class="toast toast-top toast-center">
    <div :class="`alert ${type}`">
      <span>{{ message }}</span>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';

export default defineComponent({
  setup(props, { expose }) {
    const message = ref('');
    const type = ref('');

    const showToast = ({ message, type }) => {
      this.message.value = message;
      this.type.value = type;
      setTimeout(() => {
        this.message.value = '';
      }, 3000);
    };

    expose({ showToast });

    return {
      message,
      type,
    };
  },
});
</script>
