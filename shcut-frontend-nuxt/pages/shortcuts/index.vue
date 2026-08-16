<template>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
      <div>
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Shortcuts</h1>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
          {{ shortcutsStore.total }} total shortcuts
        </p>
      </div>
      <button
        @click="openCreate"
        class="inline-flex items-center gap-2 px-4 py-2.5 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors shrink-0"
      >
        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        New Shortcut
      </button>
    </div>

    <!-- Filters -->
    <div class="flex flex-col sm:flex-row gap-3 mb-6">
      <div class="flex-1">
        <SearchInput v-model="searchQuery" placeholder="Search shortcuts..." />
      </div>
      <select
        v-model="visibilityFilter"
        class="px-3 py-2 text-sm rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none"
      >
        <option value="">All visibility</option>
        <option value="workspace">Workspace</option>
        <option value="public">Public</option>
      </select>
    </div>

    <!-- Loading -->
    <div v-if="shortcutsStore.loading" class="flex justify-center py-16">
      <div class="w-8 h-8 border-2 border-indigo-600 border-t-transparent rounded-full animate-spin" />
    </div>

    <!-- List -->
    <div v-else-if="shortcutsStore.items.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <ShortcutCard
        v-for="shortcut in shortcutsStore.items"
        :key="shortcut.id"
        :shortcut="shortcut"
        @edit="openEdit"
        @delete="confirmDelete"
      />
    </div>

    <!-- Empty -->
    <EmptyState
      v-else
      title="No shortcuts found"
      :description="searchQuery ? 'Try adjusting your search or filters.' : 'Create your first shortcut to get started.'"
    >
      <template #action>
        <button
          v-if="!searchQuery"
          @click="openCreate"
          class="inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors"
        >
          Create shortcut
        </button>
      </template>
    </EmptyState>

    <!-- Pagination -->
    <div v-if="shortcutsStore.totalPages > 1" class="flex items-center justify-center gap-2 mt-8">
      <button
        @click="goToPage(shortcutsStore.page - 1)"
        :disabled="shortcutsStore.page <= 1"
        class="px-3 py-1.5 text-sm font-medium rounded-lg border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800 disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
      >
        Previous
      </button>
      <span class="text-sm text-gray-500 dark:text-gray-400 px-3">
        Page {{ shortcutsStore.page }} of {{ shortcutsStore.totalPages }}
      </span>
      <button
        @click="goToPage(shortcutsStore.page + 1)"
        :disabled="shortcutsStore.page >= shortcutsStore.totalPages"
        class="px-3 py-1.5 text-sm font-medium rounded-lg border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800 disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
      >
        Next
      </button>
    </div>

    <!-- Create/Edit Drawer -->
    <Teleport to="body">
      <Transition
        enter-active-class="transition ease-out duration-300"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition ease-in duration-200"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div v-if="showForm" class="fixed inset-0 z-50 flex justify-end">
          <div class="absolute inset-0 bg-black/40" @click="closeForm" />
          <div class="relative w-full max-w-lg bg-white dark:bg-gray-900 shadow-xl overflow-y-auto">
            <div class="p-6">
              <div class="flex items-center justify-between mb-6">
                <h2 class="text-lg font-semibold text-gray-900 dark:text-white">
                  {{ editingShortcut ? 'Edit Shortcut' : 'New Shortcut' }}
                </h2>
                <button @click="closeForm" class="p-1.5 rounded-lg text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800">
                  <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
              <ShortcutForm
                :shortcut="editingShortcut"
                @submit="handleFormSubmit"
                @cancel="closeForm"
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
import { ref, watch, onMounted } from '#imports'
import { useShortcutsStore } from '~/stores/shortcuts'
import { useToast } from '~/composables/useToast'
import type { ShortcutWithTags } from '~/types/api'

definePageMeta({
  middleware: 'auth',
})

const shortcutsStore = useShortcutsStore()
const toast = useToast()

const searchQuery = ref('')
const visibilityFilter = ref('')
const showForm = ref(false)
const editingShortcut = ref<ShortcutWithTags | null>(null)
const showDeleteConfirm = ref(false)
const deletingShortcut = ref<ShortcutWithTags | null>(null)

const loadShortcuts = () => {
  const params: any = { page: 1, per_page: 20 }
  if (searchQuery.value) params.search = searchQuery.value
  if (visibilityFilter.value) params.visibility = visibilityFilter.value
  shortcutsStore.fetchShortcuts(params)
}

onMounted(loadShortcuts)

watch([searchQuery, visibilityFilter], () => {
  loadShortcuts()
})

const goToPage = (page: number) => {
  const params: any = { page, per_page: 20 }
  if (searchQuery.value) params.search = searchQuery.value
  if (visibilityFilter.value) params.visibility = visibilityFilter.value
  shortcutsStore.fetchShortcuts(params)
}

const openCreate = () => {
  editingShortcut.value = null
  showForm.value = true
}

const openEdit = (shortcut: ShortcutWithTags) => {
  editingShortcut.value = shortcut
  showForm.value = true
}

const closeForm = () => {
  showForm.value = false
  editingShortcut.value = null
}

const handleFormSubmit = async (payload: any) => {
  try {
    if (editingShortcut.value) {
      await shortcutsStore.updateShortcut(editingShortcut.value.id, payload)
      toast.success('Shortcut updated')
    } else {
      await shortcutsStore.createShortcut(payload)
      toast.success('Shortcut created')
    }
    closeForm()
    loadShortcuts()
  } catch (e: any) {
    toast.error(e?.data?.message || 'Failed to save shortcut')
  }
}

const confirmDelete = (shortcut: ShortcutWithTags) => {
  deletingShortcut.value = shortcut
  showDeleteConfirm.value = true
}

const handleDelete = async () => {
  if (!deletingShortcut.value) return
  try {
    await shortcutsStore.deleteShortcut(deletingShortcut.value.id)
    toast.success('Shortcut deleted')
    loadShortcuts()
  } catch (e: any) {
    toast.error(e?.data?.message || 'Failed to delete shortcut')
  }
  showDeleteConfirm.value = false
  deletingShortcut.value = null
}
</script>
