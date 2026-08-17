<template>
  <div>
    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <!-- Toolbar -->
      <div class="flex items-center justify-between mb-6 gap-4 flex-wrap">
        <div class="flex items-center gap-3">
          <!-- Tag filter -->
          <select
            v-model="tagFilter"
            @change="handleFilterChange"
            class="px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 focus:outline-none focus:ring-1 focus:ring-indigo-500"
          >
            <option value="">All tags</option>
            <option v-for="tag in availableTags" :key="tag" :value="tag">{{ tag }}</option>
          </select>

          <!-- Per page selector -->
          <select
            v-model="perPage"
            @change="handleFilterChange"
            class="px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-300 focus:outline-none focus:ring-1 focus:ring-indigo-500"
          >
            <option :value="0">All</option>
            <option :value="10">10</option>
            <option :value="20">20</option>
            <option :value="50">50</option>
            <option :value="100">100</option>
          </select>

          <!-- View toggle -->
          <div class="flex border border-gray-300 dark:border-gray-600 rounded-lg overflow-hidden">
            <button
              @click="viewMode = 'cards'"
              :class="[
                'px-3 py-1.5 text-sm font-medium transition-colors',
                viewMode === 'cards'
                  ? 'bg-indigo-600 text-white'
                  : 'bg-white dark:bg-gray-800 text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700'
              ]"
              title="Cards"
            >
              <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
              </svg>
            </button>
            <button
              @click="viewMode = 'table'"
              :class="[
                'px-3 py-1.5 text-sm font-medium transition-colors',
                viewMode === 'table'
                  ? 'bg-indigo-600 text-white'
                  : 'bg-white dark:bg-gray-800 text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700'
              ]"
              title="Table"
            >
              <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16" />
              </svg>
            </button>
          </div>
        </div>

        <!-- New Shortcut button (admin and user only) -->
        <button
          v-if="authStore.canEdit"
          @click="showCreateDrawer = true"
          class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
        >
          <svg class="-ml-1 mr-2 h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          New Shortcut
        </button>
      </div>

      <!-- Loading -->
      <div v-if="shortcutsStore.loading" class="text-center py-12 text-gray-500">Loading...</div>

      <!-- Empty state -->
      <EmptyState
        v-else-if="shortcutsStore.items.length === 0"
        title="No shortcuts"
        description="Create your first shortcut to get started."
        action-text="Create Shortcut"
        @action="showCreateDrawer = true"
      />

      <!-- Cards view -->
      <div v-else-if="viewMode === 'cards'" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <ShortcutCard
          v-for="shortcut in shortcutsStore.items"
          :key="shortcut.id"
          :shortcut="shortcut"
          @edit="editShortcut"
          @delete="confirmDelete"
          @stats="openStats"
          @filter-tag="handleTagClick"
        />
      </div>

      <!-- Table view -->
      <div v-else class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 overflow-hidden">
        <div class="overflow-x-auto">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-900">
                <th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">ID</th>
                <th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">Name</th>
                <th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">Short Link</th>
                <th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">Target URL</th>
                <th class="px-4 py-3 text-left font-medium text-gray-500 dark:text-gray-400">Tags</th>
                <th class="px-4 py-3 text-center font-medium text-gray-500 dark:text-gray-400">Views</th>
                <th class="px-4 py-3 text-center font-medium text-gray-500 dark:text-gray-400">Visibility</th>
                <th class="px-4 py-3 text-right font-medium text-gray-500 dark:text-gray-400">Actions</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100 dark:divide-gray-700">
              <tr
                v-for="shortcut in shortcutsStore.items"
                :key="shortcut.id"
                class="hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors"
              >
                <td class="px-4 py-3 text-gray-500 dark:text-gray-400 font-mono text-xs">{{ shortcut.id }}</td>
                <td class="px-4 py-3 font-semibold text-gray-900 dark:text-white">
                  /{{ shortcut.name }}
                </td>
                <td class="px-4 py-3">
                  <div class="flex items-center gap-1">
                    <a
                      :href="`/s/${shortcut.name}`"
                      target="_blank"
                      class="text-xs text-indigo-600 dark:text-indigo-400 hover:underline font-mono"
                    >
                      /s/{{ shortcut.name }}
                    </a>
                    <button
                      @click="copyShortcutLink(shortcut.name)"
                      class="text-gray-400 hover:text-indigo-500 transition-colors"
                      title="Copy link"
                    >
                      <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                      </svg>
                    </button>
                  </div>
                </td>
                <td class="px-4 py-3 text-gray-500 dark:text-gray-400 max-w-xs truncate">{{ shortcut.link }}</td>
                <td class="px-4 py-3">
                  <div class="flex flex-wrap gap-1">
                    <span
                      v-for="tag in shortcut.tags"
                      :key="tag"
                      class="inline-flex items-center px-1.5 py-0.5 rounded text-xs font-medium bg-indigo-50 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-400 cursor-pointer hover:bg-indigo-100 dark:hover:bg-indigo-900/50"
                      @click="handleTagClick(tag)"
                    >
                      {{ tag }}
                    </span>
                  </div>
                </td>
                <td class="px-4 py-3 text-center text-gray-500 dark:text-gray-400">{{ shortcut.view_count }}</td>
                <td class="px-4 py-3 text-center">
                  <span
                    v-if="shortcut.visibility === 'public'"
                    class="inline-flex items-center px-1.5 py-0.5 rounded text-xs font-medium bg-green-50 dark:bg-green-900/30 text-green-700 dark:text-green-400"
                  >
                    Public
                  </span>
                  <span v-else class="text-gray-400 dark:text-gray-500 text-xs">Workspace</span>
                </td>
                <td class="px-4 py-3 text-right">
                  <div class="flex items-center justify-end gap-1">
                    <a
                      :href="`/s/${shortcut.name}`"
                      target="_blank"
                      class="p-1.5 rounded-lg text-gray-400 hover:text-green-600 dark:hover:text-green-400 hover:bg-green-50 dark:hover:bg-green-900/20 transition-colors"
                      title="Open shortcut"
                    >
                      <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
                      </svg>
                    </a>
                    <button
                      @click="openStats(shortcut)"
                      class="p-1.5 rounded-lg text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                      title="Stats"
                    >
                      <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                      </svg>
                    </button>
                    <button
                      v-if="authStore.canEdit"
                      @click="editShortcut(shortcut)"
                      class="p-1.5 rounded-lg text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                      title="Edit"
                    >
                      <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                      </svg>
                    </button>
                    <button
                      v-if="authStore.canDelete"
                      @click="confirmDelete(shortcut)"
                      class="p-1.5 rounded-lg text-gray-400 hover:text-red-600 dark:hover:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
                      title="Delete"
                    >
                      <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                      </svg>
                    </button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- Pagination -->
      <div v-if="perPage > 0 && shortcutsStore.totalPages > 1" class="mt-6 flex items-center justify-between">
        <p class="text-sm text-gray-500 dark:text-gray-400">
          {{ shortcutsStore.total }} shortcuts
        </p>
        <nav class="flex items-center gap-2">
          <button
            @click="changePage(shortcutsStore.page - 1)"
            :disabled="shortcutsStore.page <= 1"
            class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-sm disabled:opacity-50 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
          >
            Previous
          </button>
          <span class="text-sm text-gray-600 dark:text-gray-400">
            {{ shortcutsStore.page }} / {{ shortcutsStore.totalPages }}
          </span>
          <button
            @click="changePage(shortcutsStore.page + 1)"
            :disabled="shortcutsStore.page >= shortcutsStore.totalPages"
            class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-sm disabled:opacity-50 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
          >
            Next
          </button>
        </nav>
      </div>
    </main>

    <!-- Create/Edit Drawer -->
    <div v-if="showCreateDrawer || editingShortcut" class="fixed inset-0 z-50 overflow-hidden">
      <div class="absolute inset-0 bg-black bg-opacity-50" @click="closeDrawer"></div>
      <div class="absolute inset-y-0 right-0 max-w-lg w-full bg-white dark:bg-gray-800 shadow-xl">
        <div class="h-full flex flex-col">
          <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
              {{ editingShortcut ? 'Edit Shortcut' : 'New Shortcut' }}
            </h3>
          </div>
          <div class="flex-1 overflow-y-auto px-6 py-4">
            <ShortcutForm
              :shortcut="editingShortcut"
              :loading="submitting"
              @submit="handleShortcutSubmit"
              @cancel="closeDrawer"
            />
          </div>
        </div>
      </div>
    </div>

    <!-- Delete Confirmation -->
    <ConfirmDialog
      v-model="showDeleteConfirm"
      title="Delete Shortcut"
      :message="`Are you sure you want to delete '${deletingShortcut?.name}'?`"
      confirm-text="Delete"
      :danger="true"
      @confirm="handleDelete"
    />
  </div>
</template>

<script setup lang="ts">
import type { ShortcutWithTags, ShortcutCreatePayload } from '~/types/api'

definePageMeta({ middleware: 'auth' })

const shortcutsStore = useShortcutsStore()
const authStore = useAuthStore()
const { success } = useToast()

const viewMode = ref<'cards' | 'table'>('cards')
const perPage = ref(0)
const tagFilter = ref('')
const availableTags = ref<string[]>([])
const showCreateDrawer = ref(false)
const editingShortcut = ref<ShortcutWithTags | null>(null)
const deletingShortcut = ref<ShortcutWithTags | null>(null)
const showDeleteConfirm = ref(false)
const submitting = ref(false)

const loadShortcuts = () => {
  const params: any = { per_page: perPage.value || 9999 }
  if (tagFilter.value) params.tag = tagFilter.value
  shortcutsStore.fetchShortcuts(params)
}

const loadTags = async () => {
  try {
    const tags = await $fetch<{ id: number; name: string }[]>('/api/v1/tags')
    availableTags.value = tags.map(t => t.name)
  } catch {
    // ignore
  }
}

onMounted(async () => {
  await Promise.all([
    loadShortcuts(),
    loadTags(),
  ])
})

const changePage = (page: number) => {
  const params: any = { page, per_page: perPage.value || 9999 }
  if (tagFilter.value) params.tag = tagFilter.value
  shortcutsStore.fetchShortcuts(params)
}

const handleFilterChange = () => {
  const params: any = { page: 1, per_page: perPage.value || 9999 }
  if (tagFilter.value) params.tag = tagFilter.value
  shortcutsStore.fetchShortcuts(params)
}

const handleTagClick = (tag: string) => {
  tagFilter.value = tag
  handleFilterChange()
}

const openStats = (shortcut: ShortcutWithTags) => {
  navigateTo(`/shortcuts/${shortcut.id}`)
}

const copyShortcutLink = async (name: string) => {
  const link = `${window.location.origin}/s/${name}`
  try {
    await navigator.clipboard.writeText(link)
    success('Link copied')
  } catch {
    // fallback
    const input = document.createElement('input')
    input.value = link
    document.body.appendChild(input)
    input.select()
    document.execCommand('copy')
    document.body.removeChild(input)
    success('Link copied')
  }
}

const editShortcut = (shortcut: ShortcutWithTags) => {
  editingShortcut.value = shortcut
  showCreateDrawer.value = false
}

const confirmDelete = (shortcut: ShortcutWithTags) => {
  deletingShortcut.value = shortcut
  showDeleteConfirm.value = true
}

const handleDelete = async () => {
  if (!deletingShortcut.value) return
  try {
    await shortcutsStore.deleteShortcut(deletingShortcut.value.id)
    success('Shortcut deleted')
    loadShortcuts()
  } catch {
    // error handled by toast
  }
  showDeleteConfirm.value = false
  deletingShortcut.value = null
}

const handleShortcutSubmit = async (payload: ShortcutCreatePayload) => {
  submitting.value = true
  try {
    if (editingShortcut.value) {
      await shortcutsStore.updateShortcut(editingShortcut.value.id, payload)
      success('Shortcut updated')
    } else {
      await shortcutsStore.createShortcut(payload)
      success('Shortcut created')
    }
    closeDrawer()
    loadTags()
  } catch (e: any) {
    // error shown in toast
  } finally {
    submitting.value = false
  }
}

const closeDrawer = () => {
  showCreateDrawer.value = false
  editingShortcut.value = null
}
</script>
