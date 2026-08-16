<template>
  <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <!-- Back -->
    <NuxtLink to="/collections" class="inline-flex items-center gap-1 text-sm text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 mb-6 transition-colors">
      <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
      </svg>
      Back to collections
    </NuxtLink>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-16">
      <div class="w-8 h-8 border-2 border-indigo-600 border-t-transparent rounded-full animate-spin" />
    </div>

    <template v-else-if="collection">
      <!-- Header -->
      <div class="flex flex-col sm:flex-row sm:items-start justify-between gap-4 mb-8">
        <div>
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
            {{ collection.title || collection.name }}
          </h1>
          <p v-if="collection.description" class="mt-2 text-sm text-gray-500 dark:text-gray-400">
            {{ collection.description }}
          </p>
          <div class="flex items-center gap-3 mt-2 text-sm text-gray-400 dark:text-gray-500">
            <span>{{ collection.shortcut_ids?.length || 0 }} shortcuts</span>
            <span
              v-if="collection.visibility === 'public'"
              class="inline-flex items-center px-1.5 py-0.5 rounded text-xs font-medium bg-green-50 dark:bg-green-900/30 text-green-700 dark:text-green-400"
            >
              Public
            </span>
          </div>
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

      <!-- Shortcuts in collection -->
      <div v-if="shortcuts.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <ShortcutCard
          v-for="shortcut in shortcuts"
          :key="shortcut.id"
          :shortcut="shortcut"
          @edit="openEditShortcut"
          @delete="confirmDeleteShortcut"
        />
      </div>

      <EmptyState
        v-else
        title="No shortcuts in this collection"
        description="Add shortcuts to this collection from the shortcuts page."
      />

      <!-- Edit Collection Drawer -->
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
                  <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Edit Collection</h2>
                  <button @click="showEditForm = false" class="p-1.5 rounded-lg text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800">
                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
                <CollectionForm
                  :collection="collection"
                  @submit="handleEditSubmit"
                  @cancel="showEditForm = false"
                />
              </div>
            </div>
          </div>
        </Transition>
      </Teleport>
    </template>

    <!-- Delete confirm -->
    <ConfirmDialog
      v-model="showDeleteConfirm"
      title="Delete Collection"
      message="Are you sure you want to delete this collection? The shortcuts inside will not be deleted."
      confirm-text="Delete"
      :danger="true"
      @confirm="handleDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from '#imports'
import { useCollectionsStore } from '~/stores/collections'
import { useShortcutsStore } from '~/stores/shortcuts'
import { useApi } from '~/composables/useApi'
import { useToast } from '~/composables/useToast'
import type { ShortcutWithTags } from '~/types/api'

definePageMeta({
  middleware: 'auth',
})

const route = useRoute()
const collectionsStore = useCollectionsStore()
const shortcutsStore = useShortcutsStore()
const api = useApi()
const toast = useToast()

const loading = ref(true)
const showEditForm = ref(false)
const showDeleteConfirm = ref(false)
const collectionShortcuts = ref<ShortcutWithTags[]>([])

const collection = computed(() => collectionsStore.current)

const shortcuts = computed(() => collectionShortcuts.value)

onMounted(async () => {
  const id = Number(route.params.id)
  try {
    await collectionsStore.fetchCollection(id)
    // Fetch each shortcut in the collection
    if (collection.value?.shortcut_ids?.length) {
      const results = await Promise.all(
        collection.value.shortcut_ids.map((sid) =>
          api.get<ShortcutWithTags>(`/api/v1/shortcuts/${sid}`).catch(() => null)
        )
      )
      collectionShortcuts.value = results.filter(Boolean) as ShortcutWithTags[]
    }
  } catch {
    toast.error('Failed to load collection')
    await navigateTo('/collections')
  } finally {
    loading.value = false
  }
})

const openEditShortcut = (shortcut: ShortcutWithTags) => {
  // Navigate to shortcut detail for editing
  navigateTo(`/shortcuts/${shortcut.id}`)
}

const confirmDeleteShortcut = async (shortcut: ShortcutWithTags) => {
  // Remove from collection by updating
  if (!collection.value) return
  const remainingIds = collection.value.shortcut_ids.filter((id) => id !== shortcut.id)
  try {
    await collectionsStore.updateCollection(collection.value.id, { shortcut_ids: remainingIds })
    collectionShortcuts.value = collectionShortcuts.value.filter((s) => s.id !== shortcut.id)
    toast.success('Shortcut removed from collection')
  } catch {
    toast.error('Failed to remove shortcut')
  }
}

const handleEditSubmit = async (payload: any) => {
  if (!collection.value) return
  try {
    await collectionsStore.updateCollection(collection.value.id, payload)
    toast.success('Collection updated')
    showEditForm.value = false
  } catch (e: any) {
    toast.error(e?.data?.message || 'Failed to update collection')
  }
}

const handleDelete = async () => {
  if (!collection.value) return
  try {
    await collectionsStore.deleteCollection(collection.value.id)
    toast.success('Collection deleted')
    await navigateTo('/collections')
  } catch (e: any) {
    toast.error(e?.data?.message || 'Failed to delete collection')
  }
}
</script>
