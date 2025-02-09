<template>
  <div class="navbar bg-gray-800">
    <div class="flex-1">
      <a class="btn btn-ghost text-xl"><RouterLink to="/">Books</RouterLink></a>
    </div>
    <div class="flex-none">
      <ul class="menu menu-horizontal px-1">
        <li><RouterLink to="/">Home</RouterLink></li>
        <li><RouterLink to="/library">Library</RouterLink></li>
        <li><a @click="logout">Logout</a></li>
      </ul>
    </div>
  </div>

  <RouterView />
</template>

<script lang="ts">
import router from "@/router";

export default {
  methods: {
    logout() {
      localStorage.removeItem('token')
      localStorage.removeItem('user_id')
      router.push('/')
    }
  }
};
</script>
