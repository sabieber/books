<template>
  <div class="dark min-h-screen flex items-center justify-center bg-gray-900">
    <div class="w-full max-w-md bg-gray-800 rounded-lg shadow-md p-6">
      <h2 class="text-2xl font-semibold text-white text-center mb-4">Login</h2>
      <label class="input input-bordered flex items-center gap-2 mb-4">
        <UserIcon class="size-6 text-blue-500" />
        <input type="text" class="grow" placeholder="Name" v-model="name"/>
      </label>
      <label class="input input-bordered flex items-center gap-2 mb-4">
        <KeyIcon class="size-6 text-blue-500" />
        <input type="password" class="grow" placeholder="Password" v-model="password"/>
      </label>
      <button class="btn btn-block btn-primary mb-4" @click="login">Login</button>
      <RouterLink to="/register">
        <button class="btn btn-block">Register</button>
      </RouterLink>
    </div>
  </div>
</template>

<script lang="ts">
import {RouterLink} from "vue-router";
import {UserIcon, KeyIcon} from "@heroicons/vue/16/solid";
import router from "@/router";

export default {
  components: {RouterLink, UserIcon, KeyIcon},
  data() {
    return {
      name: '',
      password: ''
    };
  },
  methods: {
    login() {
      // TODO: Send login request and handle authentication token
      localStorage.setItem('token', 'TODO')
      router.push('/')
    }
  }
};
</script>
