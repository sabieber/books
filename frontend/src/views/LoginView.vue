<template>
  <div class="dark min-h-screen flex items-center justify-center bg-gray-900">
    <div class="w-full max-w-md bg-gray-800 rounded-lg shadow-md p-6">
      <h2 class="text-2xl font-semibold text-white text-center mb-4">Login</h2>
      <div role="alert" class="alert alert-error mb-4" v-show="errorMessage">
        <ExclamationTriangleIcon class="size-6 text-white"/>
        <span v-text="errorMessage"></span>
      </div>
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
import { defineComponent, ref } from 'vue';
import { RouterLink } from "vue-router";
import { UserIcon, KeyIcon, ExclamationTriangleIcon } from "@heroicons/vue/16/solid";
import router from "@/router";

export default defineComponent({
  components: { RouterLink, UserIcon, KeyIcon, ExclamationTriangleIcon },
  setup() {
    const name = ref('');
    const password = ref('');
    const errorMessage = ref('');

    const login = () => {
      fetch('http://localhost:3000/api/user/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username: name.value, password: password.value })
      }).then(async response => {
        if (response.ok) {
          response.json().then(data => {
            localStorage.setItem('token', 'TODO');
            localStorage.setItem('user_id', data.user_id);
            router.push('/');
          }).catch(error => {
            errorMessage.value = 'Failed to connect to the server!';
            console.error('Login failed:', error);
          });
        } else {
          response.json().then(data => {
            errorMessage.value = data.error;
            console.error('Login failed:', data.error);
          }).catch(error => {
            errorMessage.value = 'Failed to connect to the server!';
            console.error('Login failed:', error);
          });
        }
      }).catch(error => {
        errorMessage.value = 'Failed to connect to the server!';
        console.error('Login failed:', error);
      });
    };

    return {
      name,
      password,
      errorMessage,
      login,
    };
  },
});
</script>
