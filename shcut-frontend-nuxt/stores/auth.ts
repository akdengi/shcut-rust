import { defineStore } from 'pinia'
import type { User, AuthResponse } from '~/types/api'

interface AuthState {
  token: string | null
  user: User | null
  loading: boolean
}

export const useAuthStore = defineStore('auth', {
  state: (): AuthState => ({
    token: null,
    user: null,
    loading: false,
  }),

  getters: {
    isAuthenticated: (state) => !!state.token,
    isAdmin: (state) => state.user?.role === 'admin',
    isUser: (state) => state.user?.role === 'user',
    isView: (state) => state.user?.role === 'view',
    canEdit: (state) => state.user?.role === 'admin' || state.user?.role === 'user',
    canDelete: (state) => state.user?.role === 'admin',
    canManageSettings: (state) => state.user?.role === 'admin',
  },

  actions: {
    init() {
      if (import.meta.client) {
        const savedToken = localStorage.getItem('shcut_token')
        if (savedToken) {
          this.token = savedToken
        }
      }
    },

    setToken(token: string) {
      this.token = token
      if (import.meta.client) {
        localStorage.setItem('shcut_token', token)
      }
    },

    clearToken() {
      this.token = null
      this.user = null
      if (import.meta.client) {
        localStorage.removeItem('shcut_token')
      }
    },

    async fetchUser() {
      if (!this.token) return
      this.loading = true
      try {
        this.user = await $fetch<User>('/api/v1/auth/me', {
          headers: { Authorization: `Bearer ${this.token}` },
        })
      } catch {
        this.clearToken()
      } finally {
        this.loading = false
      }
    },

    async login(email: string, password: string) {
      const data = await $fetch<AuthResponse>('/api/v1/auth/login', {
        method: 'POST',
        body: { email, password },
      })
      this.setToken(data.token)
      this.user = data.user
      return data
    },

    async register(email: string, nickname: string, password: string) {
      const data = await $fetch<AuthResponse>('/api/v1/auth/register', {
        method: 'POST',
        body: { email, nickname, password },
      })
      this.setToken(data.token)
      this.user = data.user
      return data
    },

    async logout() {
      this.clearToken()
      await navigateTo('/auth/login')
    },
  },
})
