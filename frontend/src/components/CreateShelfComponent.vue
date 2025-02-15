<template>
  <div>
    <button class="btn btn-circle btn-primary" @click="show = true">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" class="w-6 h-6">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
      </svg>
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
import {defineComponent} from 'vue';
import {ExclamationTriangleIcon} from "@heroicons/vue/16/solid";

export default defineComponent({
  components: {ExclamationTriangleIcon},
  data() {
    return {
      show: false,
      name: '',
      description: '',
      errorMessage: '',
    };
  },
  methods: {
    async createShelf() {
      const userId = localStorage.getItem('user_id');
      if (userId) {
        try {
          const response = await fetch('http://localhost:3000/api/shelves/create', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({name: this.name, description: this.description, user_id: userId}),
          });
          if (response.ok) {
            this.$emit('shelfCreated');
            this.name = '';
            this.description = '';
            this.show = false;
            this.errorMessage = '';
          } else {
            const data = await response.json();
            this.errorMessage = data.error;
            console.error('Failed to create shelf:', data.error);
          }
        } catch (error) {
          this.errorMessage = 'Failed to connect to the server!';
          console.error('Failed to create shelf:', error);
        }
      }
    },
    cancel() {
      this.name = '';
      this.description = '';
      this.show = false;
      this.errorMessage = '';
    },
  },
});
</script>
