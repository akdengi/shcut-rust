<template>
  <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="mb-8">
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Analytics Settings</h1>
      <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Configure what data to collect</p>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-16">
      <div class="w-8 h-8 border-2 border-indigo-600 border-t-transparent rounded-full animate-spin" />
    </div>

    <div v-else class="space-y-6">
      <!-- Main toggle -->
      <div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-sm font-semibold text-gray-900 dark:text-white">Analytics</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Enable or disable all analytics collection</p>
          </div>
          <button
            @click="toggleSetting('analytics_enabled')"
            :class="[
              'relative inline-flex h-6 w-11 items-center rounded-full transition-colors',
              settings.analytics_enabled === 'true' ? 'bg-indigo-600' : 'bg-gray-200 dark:bg-gray-700'
            ]"
          >
            <span
              :class="[
                'inline-block h-4 w-4 transform rounded-full bg-white transition-transform',
                settings.analytics_enabled === 'true' ? 'translate-x-6' : 'translate-x-1'
              ]"
            />
          </button>
        </div>
      </div>

      <!-- Individual settings -->
      <div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 divide-y divide-gray-200 dark:divide-gray-700">
        <!-- Geolocation -->
        <div class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <h3 class="text-sm font-semibold text-gray-900 dark:text-white">Geolocation</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Detect country and city from IP address</p>
            </div>
            <button
              @click="toggleSetting('analytics_geolocation')"
              :disabled="settings.analytics_enabled !== 'true'"
              :class="[
                'relative inline-flex h-6 w-11 items-center rounded-full transition-colors disabled:opacity-50',
                settings.analytics_geolocation === 'true' ? 'bg-indigo-600' : 'bg-gray-200 dark:bg-gray-700'
              ]"
            >
              <span
                :class="[
                  'inline-block h-4 w-4 transform rounded-full bg-white transition-transform',
                  settings.analytics_geolocation === 'true' ? 'translate-x-6' : 'translate-x-1'
                ]"
              />
            </button>
          </div>
          <p class="text-xs text-gray-400 dark:text-gray-500 mt-2">Uses ip-api.com (free, no API key needed)</p>
        </div>

        <!-- UTM Parameters -->
        <div class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <h3 class="text-sm font-semibold text-gray-900 dark:text-white">UTM Parameters</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Track utm_source, utm_medium, utm_campaign</p>
            </div>
            <button
              @click="toggleSetting('analytics_utm')"
              :disabled="settings.analytics_enabled !== 'true'"
              :class="[
                'relative inline-flex h-6 w-11 items-center rounded-full transition-colors disabled:opacity-50',
                settings.analytics_utm === 'true' ? 'bg-indigo-600' : 'bg-gray-200 dark:bg-gray-700'
              ]"
            >
              <span
                :class="[
                  'inline-block h-4 w-4 transform rounded-full bg-white transition-transform',
                  settings.analytics_utm === 'true' ? 'translate-x-6' : 'translate-x-1'
                ]"
              />
            </button>
          </div>
        </div>

        <!-- Referrer -->
        <div class="p-6">
          <div class="flex items-center justify-between">
            <div>
              <h3 class="text-sm font-semibold text-gray-900 dark:text-white">Referrer Tracking</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Track where visitors come from</p>
            </div>
            <button
              @click="toggleSetting('analytics_referrer')"
              :disabled="settings.analytics_enabled !== 'true'"
              :class="[
                'relative inline-flex h-6 w-11 items-center rounded-full transition-colors disabled:opacity-50',
                settings.analytics_referrer === 'true' ? 'bg-indigo-600' : 'bg-gray-200 dark:bg-gray-700'
              ]"
            >
              <span
                :class="[
                  'inline-block h-4 w-4 transform rounded-full bg-white transition-transform',
                  settings.analytics_referrer === 'true' ? 'translate-x-6' : 'translate-x-1'
                ]"
              />
            </button>
          </div>
        </div>
      </div>

      <!-- Info -->
      <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-xl p-4">
        <div class="flex">
          <svg class="w-5 h-5 text-blue-600 dark:text-blue-400 shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <div class="ml-3">
            <p class="text-sm text-blue-700 dark:text-blue-300">
              Disabling analytics speeds up redirects. Device and browser detection always works (local processing). Geolocation adds ~100ms latency.
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '~/stores/auth'
import { useToast } from '~/composables/useToast'

definePageMeta({ middleware: 'auth' })

const authStore = useAuthStore()
const toast = useToast()

const loading = ref(true)
const settings = ref<Record<string, string>>({})

const headers = computed(() => ({
  Authorization: `Bearer ${authStore.token}`,
}))

onMounted(async () => {
  try {
    const data = await $fetch<Record<string, string>>('/api/v1/settings', {
      headers: headers.value,
    })
    settings.value = data
  } catch {
    toast.error('Failed to load settings')
  } finally {
    loading.value = false
  }
})

const toggleSetting = async (key: string) => {
  const current = settings.value[key] === 'true'
  const newValue = !current

  try {
    await $fetch('/api/v1/settings', {
      method: 'PUT',
      body: { [key]: newValue },
      headers: headers.value,
    })
    settings.value[key] = newValue ? 'true' : 'false'
    toast.success('Setting updated')
  } catch {
    toast.error('Failed to update setting')
  }
}
</script>
