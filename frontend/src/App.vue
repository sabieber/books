<template>
  <div class="navbar bg-gray-800">
    <div class="flex-1">
      <a class="btn btn-ghost text-xl">
        <RouterLink to="/">Books</RouterLink>
      </a>
    </div>
    <div class="flex-none hidden sm:flex">
      <ul class="menu menu-horizontal px-1">
        <li class="tooltip tooltip-bottom" data-tip="Home">
          <RouterLink to="/" :class="{ 'menu-active': $route.path === '/' }">
            <HomeIcon class="size-6" />
          </RouterLink>
        </li>
        <li class="tooltip tooltip-bottom" data-tip="Search">
          <RouterLink to="/search" :class="{ 'menu-active': $route.path === '/search' }">
            <MagnifyingGlassIcon class="size-6" />
          </RouterLink>
        </li>
        <li class="tooltip tooltip-bottom" data-tip="Library">
          <RouterLink to="/library" :class="{ 'menu-active': $route.path === '/library' }">
            <BookOpenIcon class="size-6" />
          </RouterLink>
        </li>
        <li class="tooltip tooltip-bottom" data-tip="Logout">
          <a @click="logout">
            <UserIcon class="size-6" />
          </a>
        </li>
      </ul>
    </div>
  </div>

  <RouterView />

  <div class="dock dock-sm sm:hidden">
    <button :class="{ 'dock-active': $route.path === '/' }">
      <RouterLink to="/">
        <HomeIcon class="size-6" />
      </RouterLink>
    </button>
    <button :class="{ 'dock-active': $route.path === '/search' }">
      <RouterLink to="/search">
        <MagnifyingGlassIcon class="size-6" />
      </RouterLink>
    </button>
    <button :class="{ 'dock-active': $route.path === '/library' }">
      <RouterLink to="/library">
        <BookOpenIcon class="size-6" />
      </RouterLink>
    </button>
    <button @click="logout">
      <UserIcon class="size-6" />
    </button>
  </div>
</template>

<script lang="ts">
import router from "@/router";
import { MagnifyingGlassIcon, HomeIcon, BookOpenIcon, UserIcon } from "@heroicons/vue/16/solid";

export default {
  components: { MagnifyingGlassIcon, HomeIcon, BookOpenIcon, UserIcon },
  methods: {
    logout() {
      localStorage.removeItem('token')
      localStorage.removeItem('user_id')
      router.push('/')
    }
  }
};
</script>
