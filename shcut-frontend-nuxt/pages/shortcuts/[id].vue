<template>
  <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <!-- Back -->
    <button @click="router.back()" class="inline-flex items-center gap-1 text-sm text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 mb-6 transition-colors">
      <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
      </svg>
      Back
    </button>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-16">
      <div class="w-8 h-8 border-2 border-indigo-600 border-t-transparent rounded-full animate-spin" />
    </div>

    <template v-else-if="shortcut">
      <!-- Header -->
      <div class="flex flex-col sm:flex-row sm:items-start justify-between gap-4 mb-8">
        <div>
          <div class="flex items-center gap-3">
            <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
              /{{ shortcut.name }}
            </h1>
            <span
              v-if="shortcut.visibility === 'public'"
              class="inline-flex items-center px-2 py-0.5 rounded-md text-xs font-medium bg-green-50 dark:bg-green-900/30 text-green-700 dark:text-green-400"
            >
              Public
            </span>
            <span v-else class="inline-flex items-center px-2 py-0.5 rounded-md text-xs font-medium bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400">
              Workspace
            </span>
          </div>

          <!-- Short link with copy -->
          <div class="mt-2 flex items-center gap-2">
            <span class="text-sm font-mono text-indigo-600 dark:text-indigo-400">
              {{ serverUrl }}/s/{{ shortcut.name }}
            </span>
            <button @click="copyLink" class="p-1 rounded text-gray-400 hover:text-indigo-500 transition-colors" title="Copy link">
              <svg v-if="!copied" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
              </svg>
              <svg v-else class="w-4 h-4 text-green-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
            </button>
          </div>

          <!-- Target -->
          <a :href="shortcut.link" target="_blank" rel="noopener noreferrer" class="mt-2 inline-flex items-center gap-1 text-sm text-gray-500 dark:text-gray-400 hover:text-indigo-500 break-all">
            {{ shortcut.link }}
            <svg class="w-3.5 h-3.5 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
            </svg>
          </a>

          <p v-if="shortcut.title" class="mt-2 text-sm font-medium text-gray-700 dark:text-gray-300">{{ shortcut.title }}</p>
          <p v-if="shortcut.description" class="mt-1 text-sm text-gray-500 dark:text-gray-400">{{ shortcut.description }}</p>
        </div>

        <div class="flex items-center gap-2 shrink-0">
          <button @click="showEditForm = true" class="inline-flex items-center gap-1.5 px-3 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-800 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors">
            Edit
          </button>
          <button @click="showDeleteConfirm = true" class="inline-flex items-center gap-1.5 px-3 py-2 text-sm font-medium text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 rounded-lg hover:bg-red-100 dark:hover:bg-red-900/30 transition-colors">
            Delete
          </button>
        </div>
      </div>

      <!-- Tags -->
      <div v-if="shortcut.tags?.length" class="flex flex-wrap gap-2 mb-8">
        <span v-for="tag in shortcut.tags" :key="tag" class="inline-flex items-center px-2.5 py-1 rounded-lg text-xs font-medium bg-indigo-50 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-400">
          {{ tag }}
        </span>
      </div>

      <!-- Stats Grid -->
      <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-8">
        <div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4">
          <p class="text-sm text-gray-500 dark:text-gray-400">Total Views</p>
          <p class="mt-1 text-2xl font-bold text-gray-900 dark:text-white">{{ shortcut.view_count }}</p>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4">
          <p class="text-sm text-gray-500 dark:text-gray-400">Countries</p>
          <p class="mt-1 text-2xl font-bold text-gray-900 dark:text-white">{{ analytics?.countries?.length || 0 }}</p>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4">
          <p class="text-sm text-gray-500 dark:text-gray-400">Browsers</p>
          <p class="mt-1 text-2xl font-bold text-gray-900 dark:text-white">{{ analytics?.browsers?.length || 0 }}</p>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4">
          <p class="text-sm text-gray-500 dark:text-gray-400">Referrers</p>
          <p class="mt-1 text-2xl font-bold text-gray-900 dark:text-white">{{ analytics?.references?.length || 0 }}</p>
        </div>
      </div>

      <!-- Views Chart (simple bar chart) -->
      <div v-if="analytics?.views_by_date?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5 mb-8">
        <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-4">Views by Date</h3>
        <div class="flex items-end gap-1 h-32">
          <div
            v-for="day in analytics.views_by_date.slice(-30)"
            :key="day.date"
            class="flex-1 flex flex-col items-center gap-1"
          >
            <div
              class="w-full bg-indigo-500 rounded-t"
              :style="{ height: `${(day.count / maxViews) * 100}%`, minHeight: '4px' }"
              :title="`${day.date}: ${day.count} views`"
            />
            <span class="text-[10px] text-gray-400 transform -rotate-45 origin-top-left">{{ day.date.slice(5) }}</span>
          </div>
        </div>
      </div>

      <!-- Analytics Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
        <!-- Devices -->
        <div v-if="analytics?.devices?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">Devices</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.devices" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ item.name }}</span>
              <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ item.count }}</span>
            </div>
          </div>
        </div>

        <!-- Browsers -->
        <div v-if="analytics?.browsers?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">Browsers</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.browsers" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ item.name }}</span>
              <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ item.count }}</span>
            </div>
          </div>
        </div>

        <!-- OS -->
        <div v-if="analytics?.os?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">Operating Systems</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.os" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ item.name }}</span>
              <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ item.count }}</span>
            </div>
          </div>
        </div>

        <!-- Countries -->
        <div v-if="analytics?.countries?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">Countries</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.countries" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ item.name }}</span>
              <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ item.count }}</span>
            </div>
          </div>
        </div>

        <!-- Referrer Domains -->
        <div v-if="analytics?.references?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">Referrers</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.references" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400 truncate max-w-[150px]">{{ item.name }}</span>
              <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ item.count }}</span>
            </div>
          </div>
        </div>

        <!-- UTM Sources -->
        <div v-if="analytics?.utm_sources?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">UTM Sources</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.utm_sources" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ item.name }}</span>
              <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ item.count }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Activity Log -->
      <div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 overflow-hidden">
        <div class="px-5 py-4 border-b border-gray-200 dark:border-gray-700">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white">Activity Log</h3>
        </div>
        <div class="overflow-x-auto">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900">
                <th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">Time</th>
                <th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">IP</th>
                <th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">Country</th>
                <th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">Device</th>
                <th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">OS</th>
                <th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">Browser</th>
                <th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">Referrer</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100 dark:divide-gray-700">
              <tr v-for="activity in activities" :key="activity.id" class="hover:bg-gray-50 dark:hover:bg-gray-700/50">
                <td class="px-4 py-3 text-gray-500 dark:text-gray-400 text-xs whitespace-nowrap">
                  {{ formatTime(activity.created_ts) }}
                </td>
                <td class="px-4 py-3 text-gray-500 dark:text-gray-400 text-xs font-mono">
                  {{ activity.ip || '—' }}
                </td>
                <td class="px-4 py-3 text-gray-500 dark:text-gray-400 text-xs">
                  {{ activity.country || '—' }}
                </td>
                <td class="px-4 py-3 text-gray-500 dark:text-gray-400 text-xs">
                  {{ activity.device || '—' }}
                </td>
                <td class="px-4 py-3 text-gray-500 dark:text-gray-400 text-xs">
                  {{ activity.os || '—' }}
                </td>
                <td class="px-4 py-3 text-gray-500 dark:text-gray-400 text-xs">
                  {{ activity.browser || '—' }}
                </td>
                <td class="px-4 py-3 text-gray-500 dark:text-gray-400 text-xs truncate max-w-[150px]">
                  {{ activity.referer_domain || 'Direct' }}
                </td>
              </tr>
              <tr v-if="!activities.length">
                <td colspan="7" class="px-4 py-8 text-center text-gray-500 dark:text-gray-400 text-sm">
                  No activity yet
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </template>

    <!-- Edit Drawer -->
    <Teleport to="body">
      <Transition enter-active-class="transition ease-out duration-300" enter-from-class="opacity-0" enter-to-class="opacity-100" leave-active-class="transition ease-in duration-200" leave-from-class="opacity-100" leave-to-class="opacity-0">
        <div v-if="showEditForm" class="fixed inset-0 z-50 flex justify-end">
          <div class="absolute inset-0 bg-black/40" @click="showEditForm = false" />
          <div class="relative w-full max-w-lg bg-white dark:bg-gray-900 shadow-xl overflow-y-auto">
            <div class="p-6">
              <div class="flex items-center justify-between mb-6">
                <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Edit Shortcut</h2>
                <button @click="showEditForm = false" class="p-1.5 rounded-lg text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800">
                  <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
              <ShortcutForm :shortcut="shortcut" @submit="handleEditSubmit" @cancel="showEditForm = false" />
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- Delete confirm -->
    <ConfirmDialog
      v-model="showDeleteConfirm"
      title="Delete Shortcut"
      :message="`Are you sure you want to delete '${shortcut?.name}'?`"
      confirm-text="Delete"
      :danger="true"
      @confirm="handleDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { useShortcutsStore } from '~/stores/shortcuts'
import { useAuthStore } from '~/stores/auth'
import { useToast } from '~/composables/useToast'
import type { ShortcutAnalytics } from '~/types/api'

definePageMeta({ middleware: 'auth' })

const route = useRoute()
const router = useRouter()
const shortcutsStore = useShortcutsStore()
const authStore = useAuthStore()
const toast = useToast()

const loading = ref(true)
const showEditForm = ref(false)
const showDeleteConfirm = ref(false)
const analytics = ref<ShortcutAnalytics | null>(null)
const activities = ref<any[]>([])
const copied = ref(false)

const serverUrl = computed(() => {
  if (import.meta.client) return window.location.origin
  return ''
})

const shortcut = computed(() => shortcutsStore.current)

const maxViews = computed(() => {
  if (!analytics.value?.views_by_date?.length) return 1
  return Math.max(...analytics.value.views_by_date.map(d => d.count), 1)
})

const formatTime = (ts: number) => {
  return new Date(ts * 1000).toLocaleString()
}

onMounted(async () => {
  const id = Number(route.params.id)
  try {
    await shortcutsStore.fetchShortcut(id)
    const data = await $fetch<any>(`/api/v1/shortcuts/${id}/analytics`, {
      headers: { Authorization: `Bearer ${authStore.token}` },
    })
    analytics.value = data
    activities.value = data.activities || []
  } catch {
    toast.error('Failed to load shortcut')
    await navigateTo('/')
  } finally {
    loading.value = false
  }
})

const copyLink = async () => {
  if (!shortcut.value) return
  const link = `${serverUrl.value}/s/${shortcut.value.name}`
  try {
    await navigator.clipboard.writeText(link)
    copied.value = true
    toast.success('Link copied')
    setTimeout(() => { copied.value = false }, 2000)
  } catch {
    const input = document.createElement('input')
    input.value = link
    document.body.appendChild(input)
    input.select()
    document.execCommand('copy')
    document.body.removeChild(input)
    copied.value = true
    toast.success('Link copied')
    setTimeout(() => { copied.value = false }, 2000)
  }
}

const handleEditSubmit = async (payload: any) => {
  if (!shortcut.value) return
  try {
    await shortcutsStore.updateShortcut(shortcut.value.id, payload)
    toast.success('Shortcut updated')
    showEditForm.value = false
  } catch (e: any) {
    toast.error(e?.data?.message || 'Failed to update')
  }
}

const handleDelete = async () => {
  if (!shortcut.value) return
  try {
    await shortcutsStore.deleteShortcut(shortcut.value.id)
    toast.success('Shortcut deleted')
    await navigateTo('/')
  } catch (e: any) {
    toast.error(e?.data?.message || 'Failed to delete')
  }
  showDeleteConfirm.value = false
}
</script>
