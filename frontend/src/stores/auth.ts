import { defineStore } from 'pinia'

export const useAuthStore = defineStore('auth', {
  state: () => ({
    token: localStorage.getItem('token') as string | null,
    userId: localStorage.getItem('user_id') as string | null,
  }),
  getters: {
    isAuthenticated: (state): boolean => state.token !== null && state.token !== 'TODO',
  },
  actions: {
    setAuth(token: string, userId: string) {
      this.token = token
      this.userId = userId
      localStorage.setItem('token', token)
      localStorage.setItem('user_id', userId)
    },
    logout() {
      this.token = null
      this.userId = null
      localStorage.removeItem('token')
      localStorage.removeItem('user_id')
    },
  },
})
