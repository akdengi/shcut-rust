<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
    <AppHeader />
    <main>
      <slot />
    </main>
    <ToastContainer />
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '~/stores/auth'

const authStore = useAuthStore()

// Initialize auth on layout mount
onMounted(() => {
  authStore.init()
  if (authStore.token) {
    authStore.fetchUser()
  }
})
</script>
