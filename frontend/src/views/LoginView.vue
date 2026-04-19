<template>
  <PageContainer title="Login">
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
  </PageContainer>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { RouterLink } from "vue-router";
import { UserIcon, KeyIcon, ExclamationTriangleIcon } from "@heroicons/vue/16/solid";
import PageContainer from '@/components/PageContainer.vue';
import router from "@/router";
import { useAuthStore } from '@/stores/auth';
import { API_BASE_URL } from '@/api/config';

interface LoginSuccess {
  token: string
  user_id: string
}

interface LoginError {
  error: string
}

export default defineComponent({
  components: { RouterLink, UserIcon, KeyIcon, ExclamationTriangleIcon, PageContainer },
  setup() {
    const name = ref('');
    const password = ref('');
    const errorMessage = ref('');
    const auth = useAuthStore();

    const login = () => {
      fetch(`${API_BASE_URL}/api/user/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username: name.value, password: password.value })
      }).then(async response => {
        if (response.ok) {
          response.json().then((data: LoginSuccess) => {
            auth.setAuth(data.token, data.user_id);
            router.push('/');
          }).catch(() => {
            errorMessage.value = 'Failed to connect to the server!';
          });
        } else {
          response.json().then((data: LoginError) => {
            errorMessage.value = data.error;
          }).catch(() => {
            errorMessage.value = 'Failed to connect to the server!';
          });
        }
      }).catch(() => {
        errorMessage.value = 'Failed to connect to the server!';
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
