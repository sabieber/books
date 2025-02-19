<template>
  <div>
    <button class="btn btn-circle btn-primary" @click="show = true">
      <PlusIcon class="size-6 text-white"/>
    </button>
    <div v-if="show" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Create Shelf</h3>
        <div role="alert" class="alert alert-error mb-4" v-show="errorMessage">
          <ExclamationTriangleIcon class="size-6 text-white"/>
          <span v-text="errorMessage"></span>
        </div>
        <div class="form-control">
          <label class="label">
            <span class="label-text">Name</span>
          </label>
          <input type="text" v-model="name" class="input input-bordered" required/>
        </div>
        <div class="form-control">
          <label class="label">
            <span class="label-text">Description</span>
          </label>
          <input type="text" v-model="description" class="input input-bordered"/>
        </div>
        <div class="modal-action">
          <button class="btn" @click="createShelf" :disabled="!name">Create</button>
          <button class="btn" @click="cancel">Cancel</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { ExclamationTriangleIcon, PlusIcon } from "@heroicons/vue/16/solid";

export default defineComponent({
  components: { ExclamationTriangleIcon, PlusIcon },
  setup(_, { emit }) {
    const show = ref(false);
    const name = ref('');
    const description = ref('');
    const errorMessage = ref('');

    const createShelf = async () => {
      const userId = localStorage.getItem('user_id');
      if (userId) {
        try {
          const response = await fetch('http://localhost:3000/api/shelves/create', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name: name.value, description: description.value, user_id: userId }),
          });
          if (response.ok) {
            emit('shelfCreated');
            name.value = '';
            description.value = '';
            show.value = false;
            errorMessage.value = '';
          } else {
            const data = await response.json();
            errorMessage.value = data.error;
            console.error('Failed to create shelf:', data.error);
          }
        } catch (error) {
          errorMessage.value = 'Failed to connect to the server!';
          console.error('Failed to create shelf:', error);
        }
      }
    };

    const cancel = () => {
      name.value = '';
      description.value = '';
      show.value = false;
      errorMessage.value = '';
    };

    return {
      show,
      name,
      description,
      errorMessage,
      createShelf,
      cancel,
    };
  },
});
</script>
