<template>
  <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Tag Management</h1>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Manage tags for shortcuts</p>
      </div>
      <button
        @click="showCreateForm = true"
        class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors"
      >
        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        New Tag
      </button>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-16">
      <div class="w-8 h-8 border-2 border-indigo-600 border-t-transparent rounded-full animate-spin" />
    </div>

    <!-- Tags list -->
    <div v-else class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 overflow-hidden">
      <div class="divide-y divide-gray-200 dark:divide-gray-700">
        <div v-for="tag in tags" :key="tag.id" class="flex items-center justify-between px-5 py-4 hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors">
          <div class="flex items-center gap-3">
            <span class="inline-flex items-center px-2.5 py-1 rounded-lg text-sm font-medium bg-indigo-50 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-400">
              {{ tag.name }}
            </span>
            <span class="text-xs text-gray-500 dark:text-gray-400">
              {{ tag.shortcut_count || 0 }} shortcuts
            </span>
          </div>
          <div class="flex items-center gap-2">
            <button
              @click="startRename(tag)"
              class="p-1.5 rounded-lg text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
              title="Rename"
            >
              <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
              </svg>
            </button>
            <button
              @click="confirmDelete(tag)"
              class="p-1.5 rounded-lg text-gray-400 hover:text-red-600 dark:hover:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
              title="Delete"
            >
              <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </button>
          </div>
        </div>
        <div v-if="!tags.length" class="px-5 py-8 text-center text-gray-500 dark:text-gray-400">
          No tags yet. Create your first tag.
        </div>
      </div>
    </div>

    <!-- Create Tag Modal -->
    <Teleport to="body">
      <Transition enter-active-class="transition ease-out duration-200" enter-from-class="opacity-0" enter-to-class="opacity-100" leave-active-class="transition ease-in duration-150" leave-from-class="opacity-100" leave-to-class="opacity-0">
        <div v-if="showCreateForm" class="fixed inset-0 z-50 flex items-center justify-center p-4">
          <div class="absolute inset-0 bg-black/50" @click="showCreateForm = false" />
          <div class="relative bg-white dark:bg-gray-800 rounded-xl shadow-xl max-w-md w-full p-6">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Create Tag</h3>
            <form @submit.prevent="handleCreate">
              <input
                v-model="newTagName"
                type="text"
                placeholder="Tag name"
                autofocus
                class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2.5 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
              />
              <div class="flex items-center justify-end gap-3 mt-4">
                <button type="button" @click="showCreateForm = false" class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors">
                  Cancel
                </button>
                <button type="submit" :disabled="!newTagName.trim()" class="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 disabled:opacity-50 transition-colors">
                  Create
                </button>
              </div>
            </form>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- Rename Tag Modal -->
    <Teleport to="body">
      <Transition enter-active-class="transition ease-out duration-200" enter-from-class="opacity-0" enter-to-class="opacity-100" leave-active-class="transition ease-in duration-150" leave-from-class="opacity-100" leave-to-class="opacity-0">
        <div v-if="renamingTag" class="fixed inset-0 z-50 flex items-center justify-center p-4">
          <div class="absolute inset-0 bg-black/50" @click="renamingTag = null" />
          <div class="relative bg-white dark:bg-gray-800 rounded-xl shadow-xl max-w-md w-full p-6">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Rename Tag</h3>
            <form @submit.prevent="handleRename">
              <input
                v-model="renameName"
                type="text"
                placeholder="New tag name"
                autofocus
                class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2.5 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
              />
              <div class="flex items-center justify-end gap-3 mt-4">
                <button type="button" @click="renamingTag = null" class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors">
                  Cancel
                </button>
                <button type="submit" :disabled="!renameName.trim()" class="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 disabled:opacity-50 transition-colors">
                  Rename
                </button>
              </div>
            </form>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- Delete Confirmation -->
    <ConfirmDialog
      v-model="showDeleteConfirm"
      title="Delete Tag"
      :message="`Are you sure you want to delete tag '${deletingTag?.name}'? It will be removed from all shortcuts.`"
      confirm-text="Delete"
      :danger="true"
      @confirm="handleDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from '~/stores/auth'
import { useToast } from '~/composables/useToast'

interface TagWithCount {
  id: number
  name: string
  shortcut_count?: number
}

definePageMeta({ middleware: 'auth' })

const authStore = useAuthStore()
const toast = useToast()

const loading = ref(true)
const tags = ref<TagWithCount[]>([])
const showCreateForm = ref(false)
const newTagName = ref('')
const renamingTag = ref<TagWithCount | null>(null)
const renameName = ref('')
const deletingTag = ref<TagWithCount | null>(null)
const showDeleteConfirm = ref(false)

const headers = computed(() => ({
  Authorization: `Bearer ${authStore.token}`,
}))

const loadTags = async () => {
  try {
    const data = await $fetch<TagWithCount[]>('/api/v1/tags', {
      headers: headers.value,
    })
    tags.value = data
  } catch {
    toast.error('Failed to load tags')
  } finally {
    loading.value = false
  }
}

onMounted(loadTags)

const handleCreate = async () => {
  const name = newTagName.value.trim().toLowerCase()
  if (!name) return

  try {
    const tag = await $fetch<TagWithCount>('/api/v1/tags', {
      method: 'POST',
      body: { name },
      headers: headers.value,
    })
    tags.value.push(tag)
    newTagName.value = ''
    showCreateForm.value = false
    toast.success('Tag created')
  } catch (e: any) {
    if (e?.statusCode === 409) {
      toast.error('Tag already exists')
    } else {
      toast.error('Failed to create tag')
    }
  }
}

const startRename = (tag: TagWithCount) => {
  renamingTag.value = tag
  renameName.value = tag.name
}

const handleRename = async () => {
  if (!renamingTag.value) return
  const name = renameName.value.trim().toLowerCase()
  if (!name || name === renamingTag.value.name) {
    renamingTag.value = null
    return
  }

  try {
    const updated = await $fetch<TagWithCount>(`/api/v1/tags/${renamingTag.value.id}`, {
      method: 'PUT',
      body: { name },
      headers: headers.value,
    })
    const idx = tags.value.findIndex(t => t.id === updated.id)
    if (idx !== -1) tags.value[idx] = updated
    renamingTag.value = null
    toast.success('Tag renamed')
  } catch (e: any) {
    if (e?.statusCode === 409) {
      toast.error('Tag name already exists')
    } else {
      toast.error('Failed to rename tag')
    }
  }
}

const confirmDelete = (tag: TagWithCount) => {
  deletingTag.value = tag
  showDeleteConfirm.value = true
}

const handleDelete = async () => {
  if (!deletingTag.value) return
  try {
    await $fetch(`/api/v1/tags/${deletingTag.value.id}`, {
      method: 'DELETE',
      headers: headers.value,
    })
    tags.value = tags.value.filter(t => t.id !== deletingTag.value!.id)
    toast.success('Tag deleted')
  } catch {
    toast.error('Failed to delete tag')
  }
  showDeleteConfirm.value = false
  deletingTag.value = null
}
</script>
