<template>
  <div class="group bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4 hover:shadow-md hover:border-indigo-200 dark:hover:border-indigo-800 transition-all duration-200">
    <div class="flex items-start justify-between gap-3">
      <div class="min-w-0 flex-1">
        <!-- Name -->
        <div class="flex items-center gap-2">
          <span class="text-lg font-semibold text-indigo-600 dark:text-indigo-400 truncate">
            /{{ shortcut.name }}
          </span>
          <!-- Stats button -->
          <button
            @click="$emit('stats', shortcut)"
            class="opacity-0 group-hover:opacity-100 transition-opacity text-gray-400 hover:text-indigo-500"
            :title="$t('common.stats')"
          >
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
            </svg>
          </button>
        </div>

        <!-- Short link with copy -->
        <div class="mt-0.5 flex items-center gap-1.5">
          <span class="text-xs text-gray-400 dark:text-gray-500 font-mono">
            {{ serverUrl }}/s/{{ shortcut.name }}
          </span>
          <button
            @click="copyLink"
            class="text-gray-400 hover:text-indigo-500 transition-colors"
            :title="$t('common.copyLink')"
          >
            <svg v-if="!copied" class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
            </svg>
            <svg v-else class="w-3.5 h-3.5 text-green-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          </button>
        </div>

        <!-- Target URL -->
        <p class="mt-1 text-sm text-gray-500 dark:text-gray-400 truncate">
          {{ shortcut.link }}
        </p>

        <!-- Title / Description -->
        <p v-if="shortcut.title" class="mt-1 text-sm font-medium text-gray-700 dark:text-gray-300 truncate">
          {{ shortcut.title }}
        </p>
        <p v-if="shortcut.description" class="mt-0.5 text-xs text-gray-400 dark:text-gray-500 line-clamp-2">
          {{ shortcut.description }}
        </p>
      </div>

      <!-- Actions -->
      <div class="flex items-center gap-1 shrink-0">
        <a
          :href="`/s/${shortcut.name}`"
          target="_blank"
          class="p-1.5 rounded-lg text-gray-400 hover:text-green-600 dark:hover:text-green-400 hover:bg-green-50 dark:hover:bg-green-900/20 transition-colors opacity-0 group-hover:opacity-100"
          :title="$t('common.openShortcut')"
        >
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
          </svg>
        </a>
        <button
          v-if="authStore.canEdit"
          @click="$emit('edit', shortcut)"
          class="p-1.5 rounded-lg text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors opacity-0 group-hover:opacity-100"
          :title="$t('common.edit')"
        >
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
          </svg>
        </button>
        <button
          v-if="authStore.canDelete"
          @click="$emit('delete', shortcut)"
          class="p-1.5 rounded-lg text-gray-400 hover:text-red-600 dark:hover:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors opacity-0 group-hover:opacity-100"
          :title="$t('common.delete')"
        >
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
        </button>
      </div>
    </div>

    <!-- Tags & meta -->
    <div class="mt-3 flex items-center justify-between gap-2">
      <div class="flex flex-wrap gap-1.5">
        <span class="inline-flex items-center px-1.5 py-0.5 rounded text-xs font-mono text-gray-400 dark:text-gray-500 bg-gray-100 dark:bg-gray-700">
          #{{ shortcut.id }}
        </span>
        <span
          v-for="tag in shortcut.tags"
          :key="tag"
          class="inline-flex items-center px-2 py-0.5 rounded-md text-xs font-medium bg-indigo-50 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-400 cursor-pointer hover:bg-indigo-100 dark:hover:bg-indigo-900/50"
          @click="$emit('filterTag', tag)"
        >
          {{ tag }}
        </span>
      </div>
      <div class="flex items-center gap-3 text-xs text-gray-400 dark:text-gray-500 shrink-0">
        <span class="flex items-center gap-1">
          <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
          </svg>
          {{ shortcut.view_count }}
        </span>
        <span
          v-if="shortcut.visibility === 'public'"
          class="inline-flex items-center px-1.5 py-0.5 rounded text-xs font-medium bg-green-50 dark:bg-green-900/30 text-green-700 dark:text-green-400"
        >
          {{ $t('common.public') }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ShortcutWithTags } from '~/types/api'

const { t } = useI18n()
const authStore = useAuthStore()

const serverUrl = computed(() => {
  if (import.meta.client) {
    return window.location.origin
  }
  return ''
})

const copied = ref(false)

const copyLink = async () => {
  const link = `${serverUrl.value}/s/${props.shortcut.name}`
  try {
    await navigator.clipboard.writeText(link)
    copied.value = true
    setTimeout(() => { copied.value = false }, 2000)
  } catch {
    // fallback
    const input = document.createElement('input')
    input.value = link
    document.body.appendChild(input)
    input.select()
    document.execCommand('copy')
    document.body.removeChild(input)
    copied.value = true
    setTimeout(() => { copied.value = false }, 2000)
  }
}

const props = defineProps<{
  shortcut: ShortcutWithTags
}>()

defineEmits<{
  edit: [shortcut: ShortcutWithTags]
  delete: [shortcut: ShortcutWithTags]
  stats: [shortcut: ShortcutWithTags]
  filterTag: [tag: string]
}>()
</script>
