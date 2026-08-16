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

          <!-- Short link -->
          <div class="mt-2 flex items-center gap-2">
            <span class="text-sm font-mono text-indigo-600 dark:text-indigo-400">
              {{ serverUrl }}/s/{{ shortcut.name }}
            </span>
            <button @click="copyLink" class="text-gray-400 hover:text-indigo-500 transition-colors" title="Copy">
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
          <button
            @click="showEditForm = true"
            class="inline-flex items-center gap-1.5 px-3 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-800 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
          >
            Edit
          </button>
          <button
            @click="showDeleteConfirm = true"
            class="inline-flex items-center gap-1.5 px-3 py-2 text-sm font-medium text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 rounded-lg hover:bg-red-100 dark:hover:bg-red-900/30 transition-colors"
          >
            Delete
          </button>
        </div>
      </div>

      <!-- Tags -->
      <div v-if="shortcut.tags?.length" class="flex flex-wrap gap-2 mb-8">
        <span
          v-for="tag in shortcut.tags"
          :key="tag"
          class="inline-flex items-center px-2.5 py-1 rounded-lg text-xs font-medium bg-indigo-50 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-400"
        >
          {{ tag }}
        </span>
      </div>

      <!-- Stats -->
      <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-8">
        <div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4">
          <p class="text-sm text-gray-500 dark:text-gray-400">Total Views</p>
          <p class="mt-1 text-2xl font-bold text-gray-900 dark:text-white">{{ shortcut.view_count }}</p>
        </div>
        <div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4">
          <p class="text-sm text-gray-500 dark:text-gray-400">Devices</p>
          <p class="mt-1 text-2xl font-bold text-gray-900 dark:text-white">{{ analytics?.devices?.length || 0 }}</p>
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

      <!-- Analytics tables -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div v-if="analytics?.devices?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">Devices</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.devices" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ item.name }}</span>
              <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ item.count }}</span>
            </div>
          </div>
        </div>

        <div v-if="analytics?.browsers?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">Browsers</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.browsers" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ item.name }}</span>
              <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ item.count }}</span>
            </div>
          </div>
        </div>

        <div v-if="analytics?.references?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">Referrers</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.references" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400 truncate max-w-[200px]">{{ item.name }}</span>
              <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ item.count }}</span>
            </div>
          </div>
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
      v-if="showDeleteConfirm"
      title="Delete Shortcut"
      :message="`Are you sure you want to delete '${shortcut?.name}'?`"
      @confirm="handleDelete"
      @cancel="showDeleteConfirm = false"
    />
  </div>
</template>

<script setup lang="ts">
import { useShortcutsStore } from '~/stores/shortcuts'
import { useToast } from '~/composables/useToast'
import type { ShortcutAnalytics } from '~/types/api'

definePageMeta({ middleware: 'auth' })

const route = useRoute()
const router = useRouter()
const shortcutsStore = useShortcutsStore()
const toast = useToast()

const loading = ref(true)
const showEditForm = ref(false)
const showDeleteConfirm = ref(false)
const analytics = ref<ShortcutAnalytics | null>(null)
const copied = ref(false)

const serverUrl = computed(() => {
  if (import.meta.client) return window.location.origin
  return ''
})

const shortcut = computed(() => shortcutsStore.current)

onMounted(async () => {
  const id = Number(route.params.id)
  try {
    await shortcutsStore.fetchShortcut(id)
    await shortcutsStore.fetchAnalytics(id)
    analytics.value = shortcutsStore.analytics
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
    setTimeout(() => { copied.value = false }, 2000)
  } catch {
    // fallback
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
