import { useAuthStore } from '~/stores/auth'

export default defineNuxtRouteMiddleware((to) => {
  const authStore = useAuthStore()

  // Initialize token from localStorage on first load
  if (import.meta.client && !authStore.token) {
    authStore.init()
  }

  const publicPages = ['/auth/login', '/auth/register', '/']
  const isPublic = publicPages.includes(to.path)

  if (!authStore.isAuthenticated && !isPublic) {
    return navigateTo('/auth/login')
  }
})
