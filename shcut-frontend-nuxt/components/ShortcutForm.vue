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
      <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
        Tags
      </label>
      <!-- Tag cloud -->
      <div class="flex flex-wrap gap-2">
        <button
          v-for="tag in existingTags"
          :key="tag"
          type="button"
          @click="toggleTag(tag)"
          :class="[
            'inline-flex items-center px-3 py-1.5 rounded-lg text-sm font-medium transition-colors',
            selectedTags.includes(tag)
              ? 'bg-indigo-600 text-white'
              : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-indigo-100 dark:hover:bg-indigo-900/30 hover:text-indigo-700 dark:hover:text-indigo-300'
          ]"
        >
          <svg v-if="selectedTags.includes(tag)" class="w-3.5 h-3.5 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
          {{ tag }}
        </button>
        <span v-if="!existingTags.length" class="text-sm text-gray-500 dark:text-gray-400">
          No tags available. Create tags in Settings → Tag Management.
        </span>
      </div>
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
          <label for="og-image" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">OG Image</label>
          <div class="flex gap-2 mb-2">
            <button type="button" @click="ogImageMode = 'url'" :class="['px-3 py-1.5 text-xs font-medium rounded-lg transition-colors', ogImageMode === 'url' ? 'bg-indigo-600 text-white' : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600']">URL</button>
            <button type="button" @click="ogImageMode = 'upload'" :class="['px-3 py-1.5 text-xs font-medium rounded-lg transition-colors', ogImageMode === 'upload' ? 'bg-indigo-600 text-white' : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600']">Upload File</button>
          </div>
          <div v-if="ogImageMode === 'url'">
            <input id="og-image" v-model="form.og_image" type="url" placeholder="https://example.com/image.png"
              class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors" />
          </div>
          <div v-else>
            <div v-if="form.og_image && form.og_image.startsWith('/uploads/')" class="mb-2">
              <img :src="form.og_image" alt="OG Image preview" class="h-20 rounded-lg object-cover border border-gray-200 dark:border-gray-700" />
            </div>
            <input type="file" accept="image/png,image/jpeg,image/gif,image/webp" @change="handleOgImageUpload" class="block w-full text-sm text-gray-500 dark:text-gray-400 file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:text-sm file:font-medium file:bg-indigo-50 dark:file:bg-indigo-900/30 file:text-indigo-700 dark:file:text-indigo-300 hover:file:bg-indigo-100 dark:hover:file:bg-indigo-900/50" />
            <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">PNG, JPG, GIF or WebP. Max 2 MB.</p>
            <p v-if="ogImageUploading" class="mt-1 text-xs text-indigo-600 dark:text-indigo-400">Uploading...</p>
          </div>
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
import { ref, watch, computed, onMounted } from '#imports'
import type { ShortcutWithTags, ShortcutCreatePayload } from '~/types/api'

const props = defineProps<{
  shortcut?: ShortcutWithTags | null
  loading?: boolean
}>()

const emit = defineEmits<{
  submit: [payload: ShortcutCreatePayload]
  cancel: []
}>()

const editMode = computed(() => !!props.shortcut)
const submitting = computed(() => props.loading ?? false)

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

const ogImageMode = ref<'url' | 'upload'>('url')
const ogImageUploading = ref(false)

const handleOgImageUpload = async (event: Event) => {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  if (!file) return

  if (file.size > 2 * 1024 * 1024) {
    alert('File size must be less than 2 MB')
    input.value = ''
    return
  }

  ogImageUploading.value = true
  try {
    const formData = new FormData()
    formData.append('file', file)
    const data = await $fetch<{ url: string }>('/api/v1/upload/og-image', {
      method: 'POST',
      body: formData,
      headers: { Authorization: `Bearer ${useAuthStore().token}` },
    })
    form.value.og_image = data.url
  } catch (e) {
    alert('Failed to upload image')
  } finally {
    ogImageUploading.value = false
    input.value = ''
  }
}

const selectedTags = ref<string[]>([])
const existingTags = ref<string[]>([])

// Fetch existing tags
onMounted(async () => {
  try {
    const tags = await $fetch<{ id: number; name: string }[]>('/api/v1/tags')
    existingTags.value = tags.map(t => t.name)
  } catch {
    // ignore
  }
})

// Toggle tag selection
const toggleTag = (tag: string) => {
  const idx = selectedTags.value.indexOf(tag)
  if (idx === -1) {
    selectedTags.value.push(tag)
  } else {
    selectedTags.value.splice(idx, 1)
  }
}

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
      selectedTags.value = [...(s.tags || [])]
    } else {
      form.value = {
        name: '',
        link: '',
        title: '',
        description: '',
        visibility: 'workspace',
        og_title: '',
        og_description: '',
        og_image: '',
      }
      selectedTags.value = []
    }
  },
  { immediate: true }
)

const handleSubmit = () => {
  const payload: ShortcutCreatePayload = {
    name: form.value.name,
    link: form.value.link,
    visibility: form.value.visibility,
    tags: [...selectedTags.value],
  }

  if (form.value.title) payload.title = form.value.title
  if (form.value.description) payload.description = form.value.description
  if (form.value.og_title) payload.og_title = form.value.og_title
  if (form.value.og_description) payload.og_description = form.value.og_description
  if (form.value.og_image) payload.og_image = form.value.og_image

  emit('submit', payload)
}
</script>
