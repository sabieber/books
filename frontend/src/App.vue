<template>
  <div class="navbar bg-gray-800">
    <div class="flex-1">
      <a class="btn btn-ghost text-xl">
        <RouterLink to="/">Books</RouterLink>
      </a>
    </div>
    <div class="flex-none">
      <ul class="menu menu-horizontal px-1">
        <li class="tooltip tooltip-bottom" data-tip="Home">
          <RouterLink to="/">
            <HomeIcon class="size-6"/>
          </RouterLink>
        </li>
        <li class="tooltip tooltip-bottom" data-tip="Search">
          <RouterLink to="/search">
            <MagnifyingGlassIcon class="size-6"/>
          </RouterLink>
        </li>
        <li class="tooltip tooltip-bottom" data-tip="Library">
          <RouterLink to="/library">
            <BookOpenIcon class="size-6"/>
          </RouterLink>
        </li>
        <li class="tooltip tooltip-bottom" data-tip="Logout">
          <a @click="logout">
            <UserIcon class="size-6"/>
          </a>
        </li>
      </ul>
    </div>
  </div>

  <RouterView/>
</template>

<script lang="ts">
import router from "@/router";
import {MagnifyingGlassIcon, HomeIcon, BookOpenIcon, UserIcon} from "@heroicons/vue/16/solid";

export default {
  components: {MagnifyingGlassIcon, HomeIcon, BookOpenIcon, UserIcon},
  methods: {
    logout() {
      localStorage.removeItem('token')
      localStorage.removeItem('user_id')
      router.push('/')
    }
  }
};
</script>
