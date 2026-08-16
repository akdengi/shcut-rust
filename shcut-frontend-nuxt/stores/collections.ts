import { defineStore } from 'pinia'
import type {
  CollectionWithShortcuts,
  CollectionCreatePayload,
  CollectionUpdatePayload,
} from '~/types/api'

interface CollectionsState {
  items: CollectionWithShortcuts[]
  loading: boolean
  current: CollectionWithShortcuts | null
}

function authHeaders(): Record<string, string> {
  if (import.meta.client) {
    const token = localStorage.getItem('shcut_token')
    if (token) return { Authorization: `Bearer ${token}` }
  }
  return {}
}

export const useCollectionsStore = defineStore('collections', {
  state: (): CollectionsState => ({
    items: [],
    loading: false,
    current: null,
  }),

  actions: {
    async fetchCollections() {
      this.loading = true
      try {
        this.items = await $fetch<CollectionWithShortcuts[]>('/api/v1/collections', {
          headers: authHeaders(),
        })
      } finally {
        this.loading = false
      }
    },

    async fetchCollection(id: number) {
      this.loading = true
      try {
        this.current = await $fetch<CollectionWithShortcuts>(`/api/v1/collections/${id}`, {
          headers: authHeaders(),
        })
      } finally {
        this.loading = false
      }
    },

    async createCollection(payload: CollectionCreatePayload) {
      const collection = await $fetch<CollectionWithShortcuts>('/api/v1/collections', {
        method: 'POST',
        body: payload,
        headers: authHeaders(),
      })
      this.items.unshift(collection)
      return collection
    },

    async updateCollection(id: number, payload: CollectionUpdatePayload) {
      const collection = await $fetch<CollectionWithShortcuts>(`/api/v1/collections/${id}`, {
        method: 'PUT',
        body: payload,
        headers: authHeaders(),
      })
      const idx = this.items.findIndex((c) => c.id === id)
      if (idx !== -1) this.items[idx] = collection
      if (this.current?.id === id) this.current = collection
      return collection
    },

    async deleteCollection(id: number) {
      await $fetch(`/api/v1/collections/${id}`, {
        method: 'DELETE',
        headers: authHeaders(),
      })
      this.items = this.items.filter((c) => c.id !== id)
      if (this.current?.id === id) this.current = null
    },
  },
})
