import { createRouter, createWebHistory } from 'vue-router';
import HomeView from '@/views/HomeView.vue';
import LibraryView from "@/views/LibraryView.vue";
import SearchView from "@/views/SearchView.vue";
import SearchDetailView from "@/views/SearchDetailView.vue";
import LoginView from "@/views/LoginView.vue";
import RegisterView from "@/views/RegisterView.vue";
import ShelfDetailView from "@/views/ShelfDetailView.vue";
import BookDetailView from "@/views/BookDetailView.vue";
import ReadingDetailView from "@/views/ReadingDetailView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/library',
      name: 'library',
      component: LibraryView,
      meta: {
        requiresAuth: true
      }
    },
    {
      path: '/search',
      name: 'search',
      component: SearchView,
      meta: {
        requiresAuth: true
      }
    },
    {
      path: '/search/:id',
      name: 'search-detail',
      component: SearchDetailView,
      meta: {
        requiresAuth: true
      }
    },
    {
      path: '/shelf/:id',
      name: 'shelf-detail',
      component: ShelfDetailView,
      meta: {
        requiresAuth: true
      }
    },
    {
      path: '/book/:id',
      name: 'book-detail',
      component: BookDetailView,
      meta: {
        requiresAuth: true
      }
    },
    {
      path: '/reading/:id',
      name: 'reading-detail',
      component: ReadingDetailView,
      meta: {
        requiresAuth: true
      }
    },
    {
      path: '/profile',
      name: 'profile',
      component: () => import('@/views/ProfileView.vue'),
      meta: {
        requiresAuth: true
      }
    },
    {
      path: '/login',
      name: 'login',
      component: LoginView,
    },
    {
      path: '/register',
      name: 'register',
      component: RegisterView,
    }
  ],
})

router.beforeEach((to, from, next) => {
  if (to.meta.requiresAuth) {
    const token = localStorage.getItem('token');
    if (token) {
      // User is authenticated, proceed to the route
      next();
    } else {
      // User is not authenticated, redirect to login
      next('/login');
    }
  } else {
    // Non-protected route, allow access
    next();
  }
});

export default router
