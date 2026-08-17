<template>
  <div class="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex items-center justify-between mb-6">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Profile Settings</h1>
      <button @click="navigateTo('/')" class="p-2 rounded-lg text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors" title="Close">
        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6">
      <form @submit.prevent="handleSave" class="space-y-5">
        <!-- Avatar -->
        <div class="flex items-center gap-4">
          <div class="w-16 h-16 rounded-full bg-indigo-100 dark:bg-indigo-900 flex items-center justify-center">
            <span class="text-2xl font-bold text-indigo-600 dark:text-indigo-400">
              {{ authStore.user?.nickname?.charAt(0).toUpperCase() || '?' }}
            </span>
          </div>
          <div>
            <p class="text-sm font-medium text-gray-900 dark:text-white">{{ authStore.user?.nickname }}</p>
            <p class="text-xs text-gray-500 dark:text-gray-400">{{ authStore.user?.email }}</p>
          </div>
        </div>

        <hr class="border-gray-200 dark:border-gray-700" />

        <!-- Nickname -->
        <div>
          <label for="nickname" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nickname</label>
          <input
            id="nickname"
            v-model="form.nickname"
            type="text"
            class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-900 px-3 py-2.5 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
          />
        </div>

        <!-- Email -->
        <div>
          <label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Email</label>
          <input
            id="email"
            v-model="form.email"
            type="email"
            class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-900 px-3 py-2.5 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
          />
        </div>

        <!-- Role (read-only) -->
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Role</label>
          <input
            :value="authStore.user?.role"
            type="text"
            disabled
            class="block w-full rounded-lg border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 px-3 py-2.5 text-sm text-gray-500 dark:text-gray-400 cursor-not-allowed"
          />
        </div>

        <div v-if="error" class="p-3 rounded-lg bg-red-50 dark:bg-red-900/20 text-sm text-red-600 dark:text-red-400">
          {{ error }}
        </div>

        <div v-if="success" class="p-3 rounded-lg bg-green-50 dark:bg-green-900/20 text-sm text-green-600 dark:text-green-400">
          {{ success }}
        </div>

        <div class="flex justify-end">
          <button
            type="submit"
            :disabled="saving"
            class="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {{ saving ? 'Saving...' : 'Save changes' }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from '#imports'
import { useAuthStore } from '~/stores/auth'
import { useApi } from '~/composables/useApi'
import { useToast } from '~/composables/useToast'

definePageMeta({
  middleware: 'auth',
})

const authStore = useAuthStore()
const api = useApi()
const toast = useToast()

const form = ref({ nickname: '', email: '' })
const saving = ref(false)
const error = ref('')
const success = ref('')

onMounted(async () => {
  if (!authStore.user) {
    await authStore.fetchUser()
  }
  form.value = {
    nickname: authStore.user?.nickname || '',
    email: authStore.user?.email || '',
  }
})

const handleSave = async () => {
  saving.value = true
  error.value = ''
  success.value = ''
  try {
    const updated = await api.put<any>(`/api/v1/users/${authStore.user!.id}`, {
      nickname: form.value.nickname,
      email: form.value.email,
    })
    authStore.user = updated
    success.value = 'Profile updated successfully'
    toast.success('Profile updated')
  } catch (e: any) {
    error.value = e?.data?.message || 'Failed to update profile'
  } finally {
    saving.value = false
  }
}
</script>
