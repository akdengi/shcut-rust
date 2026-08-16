import { useAuthStore } from '~/stores/auth'

interface ApiOptions {
  method?: string
  body?: any
  headers?: Record<string, string>
}

export const useApi = () => {
  const config = useRuntimeConfig()
  const authStore = useAuthStore()

  const baseURL = config.public.apiBase || ''

  const request = async <T>(endpoint: string, options: ApiOptions = {}): Promise<T> => {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...options.headers,
    }

    if (authStore.token) {
      headers['Authorization'] = `Bearer ${authStore.token}`
    }

    const response = await $fetch<T>(endpoint, {
      baseURL,
      method: (options.method || 'GET') as any,
      body: options.body,
      headers,
    })

    return response
  }

  const get = <T>(endpoint: string) => request<T>(endpoint)
  const post = <T>(endpoint: string, body?: any) => request<T>(endpoint, { method: 'POST', body })
  const put = <T>(endpoint: string, body?: any) => request<T>(endpoint, { method: 'PUT', body })
  const del = <T>(endpoint: string) => request<T>(endpoint, { method: 'DELETE' })

  return { get, post, put, del }
}
