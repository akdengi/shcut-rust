<template>
  <div class="group bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-5 hover:shadow-md hover:border-indigo-200 dark:hover:border-indigo-800 transition-all duration-200">
    <NuxtLink :to="`/collections/${collection.id}`" class="block">
      <div class="flex items-start justify-between gap-3">
        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-2">
            <div class="w-8 h-8 rounded-lg bg-amber-100 dark:bg-amber-900/30 flex items-center justify-center shrink-0">
              <svg class="w-4 h-4 text-amber-600 dark:text-amber-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
              </svg>
            </div>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white truncate">
              {{ collection.title || collection.name }}
            </h3>
          </div>
          <p v-if="collection.description" class="mt-1 text-sm text-gray-500 dark:text-gray-400 line-clamp-2">
            {{ collection.description }}
          </p>
        </div>
      </div>

      <!-- Meta -->
      <div class="mt-3 flex items-center gap-3 text-xs text-gray-400 dark:text-gray-500">
        <span class="flex items-center gap-1">
          <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" />
          </svg>
          {{ collection.shortcut_ids?.length || 0 }} shortcuts
        </span>
        <span
          v-if="collection.visibility === 'public'"
          class="inline-flex items-center px-1.5 py-0.5 rounded text-xs font-medium bg-green-50 dark:bg-green-900/30 text-green-700 dark:text-green-400"
        >
          Public
        </span>
      </div>
    </NuxtLink>
  </div>
</template>

<script setup lang="ts">
import type { CollectionWithShortcuts } from '~/types/api'

defineProps<{
  collection: CollectionWithShortcuts
}>()
</script>
