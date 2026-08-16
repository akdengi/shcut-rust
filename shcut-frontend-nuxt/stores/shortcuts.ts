import { defineStore } from 'pinia'
import type {
  ShortcutWithTags,
  ShortcutCreatePayload,
  ShortcutUpdatePayload,
  ShortcutAnalytics,
  PaginatedResponse,
  ShortcutListParams,
} from '~/types/api'

interface ShortcutsState {
  items: ShortcutWithTags[]
  total: number
  page: number
  perPage: number
  totalPages: number
  loading: boolean
  current: ShortcutWithTags | null
  analytics: ShortcutAnalytics | null
}

function authHeaders(): Record<string, string> {
  if (import.meta.client) {
    const token = localStorage.getItem('shcut_token')
    if (token) return { Authorization: `Bearer ${token}` }
  }
  return {}
}

export const useShortcutsStore = defineStore('shortcuts', {
  state: (): ShortcutsState => ({
    items: [],
    total: 0,
    page: 1,
    perPage: 20,
    totalPages: 0,
    loading: false,
    current: null,
    analytics: null,
  }),

  actions: {
    async fetchShortcuts(params: ShortcutListParams = {}) {
      this.loading = true
      try {
        const query = new URLSearchParams()
        if (params.page) query.set('page', String(params.page))
        if (params.per_page) query.set('per_page', String(params.per_page))
        if (params.tag) query.set('tag', params.tag)
        if (params.search) query.set('search', params.search)
        if (params.visibility) query.set('visibility', params.visibility)

        const qs = query.toString()
        const url = `/api/v1/shortcuts${qs ? `?${qs}` : ''}`
        const data = await $fetch<PaginatedResponse<ShortcutWithTags>>(url, {
          headers: authHeaders(),
        })

        this.items = data.items
        this.total = data.total
        this.page = data.page
        this.perPage = data.per_page
        this.totalPages = data.total_pages
      } finally {
        this.loading = false
      }
    },

    async fetchShortcut(id: number) {
      this.loading = true
      try {
        this.current = await $fetch<ShortcutWithTags>(`/api/v1/shortcuts/${id}`, {
          headers: authHeaders(),
        })
      } finally {
        this.loading = false
      }
    },

    async fetchAnalytics(id: number) {
      this.analytics = await $fetch<ShortcutAnalytics>(`/api/v1/shortcuts/${id}/analytics`, {
        headers: authHeaders(),
      })
    },

    async createShortcut(payload: ShortcutCreatePayload) {
      const shortcut = await $fetch<ShortcutWithTags>('/api/v1/shortcuts', {
        method: 'POST',
        body: payload,
        headers: authHeaders(),
      })
      this.items.unshift(shortcut)
      this.total++
      return shortcut
    },

    async updateShortcut(id: number, payload: ShortcutUpdatePayload) {
      const shortcut = await $fetch<ShortcutWithTags>(`/api/v1/shortcuts/${id}`, {
        method: 'PUT',
        body: payload,
        headers: authHeaders(),
      })
      const idx = this.items.findIndex((s) => s.id === id)
      if (idx !== -1) this.items[idx] = shortcut
      if (this.current?.id === id) this.current = shortcut
      return shortcut
    },

    async deleteShortcut(id: number) {
      await $fetch(`/api/v1/shortcuts/${id}`, {
        method: 'DELETE',
        headers: authHeaders(),
      })
      this.items = this.items.filter((s) => s.id !== id)
      if (this.current?.id === id) this.current = null
      this.total--
    },
  },
})
