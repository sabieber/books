<template>
  <div class="dark min-h-screen flex items-center justify-center bg-gray-900">
    <div class="w-full max-w-md bg-gray-800 rounded-lg shadow-md p-6">
      <h2 class="text-2xl font-semibold text-white text-center mb-4">Register</h2>
      <label class="input input-bordered flex items-center gap-2 mb-4">
        <UserIcon class="size-6 text-blue-500" />
        <input type="text" class="grow" placeholder="Name" v-model="name"/>
      </label>
      <label class="input input-bordered flex items-center gap-2 mb-4">
        <KeyIcon class="size-6 text-blue-500" />
        <input type="password" class="grow" placeholder="Password" v-model="password"/>
      </label>
      <button class="btn btn-block btn-primary mb-4" @click="register">Register</button>
    </div>
  </div>
</template>

<script lang="ts">
import router from "@/router";
import {KeyIcon, UserIcon} from "@heroicons/vue/16/solid";

export default {
  components: {KeyIcon, UserIcon},
  data() {
    return {
      name: '',
      password: ''
    };
  },
  methods: {
    register() {
      // Send register request and handle response
      router.push('/login')
    }
  }
};
</script>
