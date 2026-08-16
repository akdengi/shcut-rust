<template>
  <form @submit.prevent="handleSubmit" class="space-y-5">
    <!-- Name -->
    <div>
      <label for="collection-name" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Name <span class="text-red-500">*</span>
      </label>
      <input
        id="collection-name"
        v-model="form.name"
        type="text"
        required
        placeholder="my-collection"
        class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
      />
    </div>

    <!-- Title -->
    <div>
      <label for="collection-title" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Display Title
      </label>
      <input
        id="collection-title"
        v-model="form.title"
        type="text"
        placeholder="Optional display title"
        class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
      />
    </div>

    <!-- Description -->
    <div>
      <label for="collection-desc" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Description
      </label>
      <textarea
        id="collection-desc"
        v-model="form.description"
        rows="3"
        placeholder="Optional description"
        class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors resize-none"
      />
    </div>

    <!-- Visibility -->
    <div>
      <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Visibility</label>
      <div class="flex gap-3">
        <label
          :class="[
            'flex-1 flex items-center gap-2 px-3 py-2 rounded-lg border cursor-pointer transition-colors',
            form.visibility === 'workspace'
              ? 'border-indigo-500 bg-indigo-50 dark:bg-indigo-900/20 text-indigo-700 dark:text-indigo-400'
              : 'border-gray-300 dark:border-gray-600 text-gray-600 dark:text-gray-400 hover:border-gray-400'
          ]"
        >
          <input v-model="form.visibility" type="radio" value="workspace" class="sr-only" />
          <span class="text-sm font-medium">Workspace</span>
        </label>
        <label
          :class="[
            'flex-1 flex items-center gap-2 px-3 py-2 rounded-lg border cursor-pointer transition-colors',
            form.visibility === 'public'
              ? 'border-indigo-500 bg-indigo-50 dark:bg-indigo-900/20 text-indigo-700 dark:text-indigo-400'
              : 'border-gray-300 dark:border-gray-600 text-gray-600 dark:text-gray-400 hover:border-gray-400'
          ]"
        >
          <input v-model="form.visibility" type="radio" value="public" class="sr-only" />
          <span class="text-sm font-medium">Public</span>
        </label>
      </div>
    </div>

    <!-- Submit -->
    <div class="flex items-center justify-end gap-3 pt-2">
      <button
        type="button"
        @click="$emit('cancel')"
        class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
      >
        Cancel
      </button>
      <button
        type="submit"
        :disabled="submitting"
        class="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
      >
        {{ submitting ? 'Saving...' : (editMode ? 'Update' : 'Create') }}
      </button>
    </div>
  </form>
</template>

<script setup lang="ts">
import { ref, watch } from '#imports'
import type { CollectionWithShortcuts, CollectionCreatePayload } from '~/types/api'

const props = defineProps<{
  collection?: CollectionWithShortcuts | null
}>()

const emit = defineEmits<{
  submit: [payload: CollectionCreatePayload]
  cancel: []
}>()

const editMode = computed(() => !!props.collection)
const submitting = ref(false)

const form = ref({
  name: '',
  title: '',
  description: '',
  visibility: 'workspace' as 'workspace' | 'public',
})

watch(
  () => props.collection,
  (c) => {
    if (c) {
      form.value = {
        name: c.name,
        title: c.title || '',
        description: c.description || '',
        visibility: c.visibility,
      }
    }
  },
  { immediate: true }
)

const handleSubmit = async () => {
  submitting.value = true
  try {
    const payload: CollectionCreatePayload = {
      name: form.value.name,
      visibility: form.value.visibility,
    }
    if (form.value.title) payload.title = form.value.title
    if (form.value.description) payload.description = form.value.description
    emit('submit', payload)
  } finally {
    submitting.value = false
  }
}
</script>
