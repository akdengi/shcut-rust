<template>
  <form @submit.prevent="handleSubmit" class="space-y-5">
    <!-- Name -->
    <div>
      <label for="shortcut-name" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Short name <span class="text-red-500">*</span>
      </label>
      <div class="flex items-center">
        <span class="text-sm text-gray-500 dark:text-gray-400 mr-1">/</span>
        <input
          id="shortcut-name"
          v-model="form.name"
          type="text"
          required
          placeholder="my-link"
          class="flex-1 block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
        />
      </div>
    </div>

    <!-- Target URL -->
    <div>
      <label for="shortcut-link" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Target URL <span class="text-red-500">*</span>
      </label>
      <input
        id="shortcut-link"
        v-model="form.link"
        type="url"
        required
        placeholder="https://example.com/very/long/url"
        class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
      />
    </div>

    <!-- Title -->
    <div>
      <label for="shortcut-title" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Title
      </label>
      <input
        id="shortcut-title"
        v-model="form.title"
        type="text"
        placeholder="Optional display title"
        class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
      />
    </div>

    <!-- Description -->
    <div>
      <label for="shortcut-desc" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Description
      </label>
      <textarea
        id="shortcut-desc"
        v-model="form.description"
        rows="2"
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
              : 'border-gray-300 dark:border-gray-600 text-gray-600 dark:text-gray-400 hover:border-gray-400 dark:hover:border-gray-500'
          ]"
        >
          <input v-model="form.visibility" type="radio" value="workspace" class="sr-only" />
          <svg class="w-4 h-4 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
          </svg>
          <span class="text-sm font-medium">Workspace</span>
        </label>
        <label
          :class="[
            'flex-1 flex items-center gap-2 px-3 py-2 rounded-lg border cursor-pointer transition-colors',
            form.visibility === 'public'
              ? 'border-indigo-500 bg-indigo-50 dark:bg-indigo-900/20 text-indigo-700 dark:text-indigo-400'
              : 'border-gray-300 dark:border-gray-600 text-gray-600 dark:text-gray-400 hover:border-gray-400 dark:hover:border-gray-500'
          ]"
        >
          <input v-model="form.visibility" type="radio" value="public" class="sr-only" />
          <svg class="w-4 h-4 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <span class="text-sm font-medium">Public</span>
        </label>
      </div>
    </div>

    <!-- Tags -->
    <div>
      <label for="shortcut-tags" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Tags
      </label>
      <input
        id="shortcut-tags"
        v-model="tagsInput"
        type="text"
        placeholder="Comma separated: tag1, tag2"
        class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
      />
    </div>

    <!-- OG fields (collapsible) -->
    <details class="group">
      <summary class="text-sm font-medium text-gray-500 dark:text-gray-400 cursor-pointer hover:text-gray-700 dark:hover:text-gray-300 select-none">
        Open Graph / Social Preview
        <svg class="w-4 h-4 inline-block ml-1 transition-transform group-open:rotate-180" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
        </svg>
      </summary>
      <div class="mt-3 space-y-4">
        <div>
          <label for="og-title" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">OG Title</label>
          <input id="og-title" v-model="form.og_title" type="text" placeholder="Custom social preview title"
            class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors" />
        </div>
        <div>
          <label for="og-desc" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">OG Description</label>
          <textarea id="og-desc" v-model="form.og_description" rows="2" placeholder="Custom social preview description"
            class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors resize-none" />
        </div>
        <div>
          <label for="og-image" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">OG Image URL</label>
          <input id="og-image" v-model="form.og_image" type="url" placeholder="https://example.com/image.png"
            class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors" />
        </div>
      </div>
    </details>

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
import type { ShortcutWithTags, ShortcutCreatePayload } from '~/types/api'

const props = defineProps<{
  shortcut?: ShortcutWithTags | null
}>()

const emit = defineEmits<{
  submit: [payload: ShortcutCreatePayload]
  cancel: []
}>()

const editMode = computed(() => !!props.shortcut)
const submitting = ref(false)

const form = ref({
  name: '',
  link: '',
  title: '',
  description: '',
  visibility: 'workspace' as 'workspace' | 'public',
  og_title: '',
  og_description: '',
  og_image: '',
})

const tagsInput = ref('')

watch(
  () => props.shortcut,
  (s) => {
    if (s) {
      form.value = {
        name: s.name,
        link: s.link,
        title: s.title || '',
        description: s.description || '',
        visibility: s.visibility,
        og_title: s.og_title || '',
        og_description: s.og_description || '',
        og_image: s.og_image || '',
      }
      tagsInput.value = s.tags?.join(', ') || ''
    }
  },
  { immediate: true }
)

const handleSubmit = async () => {
  submitting.value = true
  try {
    const tags = tagsInput.value
      .split(',')
      .map((t) => t.trim())
      .filter(Boolean)

    const payload: ShortcutCreatePayload = {
      name: form.value.name,
      link: form.value.link,
      visibility: form.value.visibility,
    }

    if (form.value.title) payload.title = form.value.title
    if (form.value.description) payload.description = form.value.description
    if (tags.length) payload.tags = tags
    if (form.value.og_title) payload.og_title = form.value.og_title
    if (form.value.og_description) payload.og_description = form.value.og_description
    if (form.value.og_image) payload.og_image = form.value.og_image

    emit('submit', payload)
  } finally {
    submitting.value = false
  }
}
</script>
