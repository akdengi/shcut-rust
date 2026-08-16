<template>
  <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <!-- Back -->
    <NuxtLink to="/shortcuts" class="inline-flex items-center gap-1 text-sm text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 mb-6 transition-colors">
      <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
      </svg>
      Back to shortcuts
    </NuxtLink>

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
          </div>
          <a
            :href="shortcut.link"
            target="_blank"
            rel="noopener noreferrer"
            class="mt-1 inline-flex items-center gap-1 text-sm text-indigo-600 dark:text-indigo-400 hover:underline break-all"
          >
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
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
            </svg>
            Edit
          </button>
          <button
            @click="showDeleteConfirm = true"
            class="inline-flex items-center gap-1.5 px-3 py-2 text-sm font-medium text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-900/20 rounded-lg hover:bg-red-100 dark:hover:bg-red-900/30 transition-colors"
          >
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
            Delete
          </button>
        </div>
      </div>

      <!-- Tags -->
      <div v-if="shortcut.tags?.length" class="flex flex-wrap gap-2 mb-6">
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
        <StatsCard label="Total Views" :value="shortcut.view_count" color="indigo">
          <template #icon>
            <svg class="w-5 h-5 text-indigo-600 dark:text-indigo-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
            </svg>
          </template>
        </StatsCard>
        <StatsCard
          label="Devices"
          :value="analytics?.devices?.length || 0"
          color="blue"
        >
          <template #icon>
            <svg class="w-5 h-5 text-blue-600 dark:text-blue-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z" />
            </svg>
          </template>
        </StatsCard>
        <StatsCard
          label="Countries"
          :value="analytics?.countries?.length || 0"
          color="green"
        >
          <template #icon>
            <svg class="w-5 h-5 text-green-600 dark:text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </template>
        </StatsCard>
        <StatsCard
          label="Browsers"
          :value="analytics?.browsers?.length || 0"
          color="amber"
        >
          <template #icon>
            <svg class="w-5 h-5 text-amber-600 dark:text-amber-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
            </svg>
          </template>
        </StatsCard>
      </div>

      <!-- Analytics tables -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <!-- Devices -->
        <div v-if="analytics?.devices?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">Devices</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.devices" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ item.name }}</span>
              <div class="flex items-center gap-2">
                <div class="w-24 h-1.5 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
                  <div
                    class="h-full bg-indigo-500 rounded-full"
                    :style="{ width: `${(item.count / analytics.view_count) * 100}%` }"
                  />
                </div>
                <span class="text-xs font-medium text-gray-500 dark:text-gray-400 w-8 text-right">{{ item.count }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Browsers -->
        <div v-if="analytics?.browsers?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">Browsers</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.browsers" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ item.name }}</span>
              <div class="flex items-center gap-2">
                <div class="w-24 h-1.5 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
                  <div
                    class="h-full bg-green-500 rounded-full"
                    :style="{ width: `${(item.count / analytics.view_count) * 100}%` }"
                  />
                </div>
                <span class="text-xs font-medium text-gray-500 dark:text-gray-400 w-8 text-right">{{ item.count }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Countries -->
        <div v-if="analytics?.countries?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">Countries</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.countries" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ item.name }}</span>
              <div class="flex items-center gap-2">
                <div class="w-24 h-1.5 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
                  <div
                    class="h-full bg-amber-500 rounded-full"
                    :style="{ width: `${(item.count / analytics.view_count) * 100}%` }"
                  />
                </div>
                <span class="text-xs font-medium text-gray-500 dark:text-gray-400 w-8 text-right">{{ item.count }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- References -->
        <div v-if="analytics?.references?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">Referrers</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.references" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400 truncate max-w-[200px]">{{ item.name }}</span>
              <div class="flex items-center gap-2">
                <div class="w-24 h-1.5 bg-gray-100 dark:bg-gray-700 rounded-full overflow-hidden">
                  <div
                    class="h-full bg-blue-500 rounded-full"
                    :style="{ width: `${(item.count / analytics.view_count) * 100}%` }"
                  />
                </div>
                <span class="text-xs font-medium text-gray-500 dark:text-gray-400 w-8 text-right">{{ item.count }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- UTM breakdown -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div v-if="analytics?.utm_sources?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">UTM Sources</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.utm_sources" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ item.name }}</span>
              <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ item.count }}</span>
            </div>
          </div>
        </div>
        <div v-if="analytics?.utm_mediums?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">UTM Mediums</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.utm_mediums" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ item.name }}</span>
              <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ item.count }}</span>
            </div>
          </div>
        </div>
        <div v-if="analytics?.utm_campaigns?.length" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5">
          <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-3">UTM Campaigns</h3>
          <div class="space-y-2">
            <div v-for="item in analytics.utm_campaigns" :key="item.name" class="flex items-center justify-between">
              <span class="text-sm text-gray-600 dark:text-gray-400">{{ item.name }}</span>
              <span class="text-xs font-medium text-gray-500 dark:text-gray-400">{{ item.count }}</span>
            </div>
          </div>
        </div>
      </div>
    </template>

    <!-- Edit Drawer -->
    <Teleport to="body">
      <Transition
        enter-active-class="transition ease-out duration-300"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition ease-in duration-200"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
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
              <ShortcutForm
                :shortcut="shortcut"
                @submit="handleEditSubmit"
                @cancel="showEditForm = false"
              />
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- Delete confirm -->
    <ConfirmDialog
      v-model="showDeleteConfirm"
      title="Delete Shortcut"
      message="Are you sure you want to delete this shortcut? This action cannot be undone."
      confirm-text="Delete"
      :danger="true"
      @confirm="handleDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from '#imports'
import { useShortcutsStore } from '~/stores/shortcuts'
import { useToast } from '~/composables/useToast'
import type { ShortcutAnalytics } from '~/types/api'

definePageMeta({
  middleware: 'auth',
})

const route = useRoute()
const shortcutsStore = useShortcutsStore()
const toast = useToast()

const loading = ref(true)
const showEditForm = ref(false)
const showDeleteConfirm = ref(false)
const analytics = ref<ShortcutAnalytics | null>(null)

const shortcut = computed(() => shortcutsStore.current)

onMounted(async () => {
  const id = Number(route.params.id)
  try {
    await shortcutsStore.fetchShortcut(id)
    await shortcutsStore.fetchAnalytics(id)
    analytics.value = shortcutsStore.analytics
  } catch {
    toast.error('Failed to load shortcut')
    await navigateTo('/shortcuts')
  } finally {
    loading.value = false
  }
})

const handleEditSubmit = async (payload: any) => {
  if (!shortcut.value) return
  try {
    await shortcutsStore.updateShortcut(shortcut.value.id, payload)
    toast.success('Shortcut updated')
    showEditForm.value = false
  } catch (e: any) {
    toast.error(e?.data?.message || 'Failed to update shortcut')
  }
}

const handleDelete = async () => {
  if (!shortcut.value) return
  try {
    await shortcutsStore.deleteShortcut(shortcut.value.id)
    toast.success('Shortcut deleted')
    await navigateTo('/shortcuts')
  } catch (e: any) {
    toast.error(e?.data?.message || 'Failed to delete shortcut')
  }
}
</script>
