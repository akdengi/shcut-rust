<template>
  <div class="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-8">Workspace Settings</h1>

    <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-6 space-y-6">
      <!-- Company Name -->
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Company Name
        </label>
        <input
          v-model="form.company_name"
          type="text"
          placeholder="My Company"
          class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none"
        />
      </div>

      <!-- Logo Upload -->
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Logo
        </label>
        
        <!-- Current logo preview -->
        <div v-if="settings.logo_url" class="mb-3 flex items-center gap-3">
          <img :src="settings.logo_url" alt="Current logo" class="w-16 h-16 rounded-lg object-contain border border-gray-200 dark:border-gray-600" />
          <span class="text-sm text-gray-500 dark:text-gray-400">Current logo</span>
        </div>

        <!-- File input -->
        <div class="flex items-center gap-3">
          <label class="cursor-pointer inline-flex items-center px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg shadow-sm text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 transition-colors">
            <svg class="w-5 h-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
            Choose file
            <input
              type="file"
              accept="image/png,image/jpeg,image/gif,image/svg+xml,image/webp"
              class="hidden"
              @change="handleFileSelect"
            />
          </label>
          <span v-if="selectedFile" class="text-sm text-gray-600 dark:text-gray-400">
            {{ selectedFile.name }}
          </span>
          <span v-else class="text-sm text-gray-400 dark:text-gray-500">
            PNG, JPG, GIF, SVG, WebP (max 2MB)
          </span>
        </div>

        <!-- Upload progress -->
        <div v-if="uploading" class="mt-2">
          <div class="flex items-center gap-2 text-sm text-indigo-600 dark:text-indigo-400">
            <svg class="animate-spin h-4 w-4" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Uploading...
          </div>
        </div>

        <!-- Upload error -->
        <div v-if="uploadError" class="mt-2 text-sm text-red-600 dark:text-red-400">
          {{ uploadError }}
        </div>
      </div>

      <!-- Preview -->
      <div v-if="form.company_name || settings.logo_url" class="border-t border-gray-200 dark:border-gray-700 pt-6">
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">Preview</label>
        <div class="flex items-center gap-3 p-4 bg-gray-50 dark:bg-gray-900 rounded-lg">
          <template v-if="settings.logo_url">
            <img :src="settings.logo_url" :alt="form.company_name" class="w-10 h-10 rounded-lg object-contain" />
          </template>
          <template v-else>
            <div class="w-10 h-10 rounded-lg bg-indigo-600 flex items-center justify-center">
              <span class="text-white font-bold text-xl">/</span>
            </div>
          </template>
          <span class="text-xl font-bold text-gray-900 dark:text-white">{{ form.company_name || 'ShCut' }}</span>
        </div>
      </div>

      <!-- Save -->
      <div class="flex justify-end pt-4 border-t border-gray-200 dark:border-gray-700">
        <button
          @click="handleSave"
          :disabled="saving"
          class="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 disabled:opacity-50 transition-colors"
        >
          {{ saving ? 'Saving...' : 'Save Settings' }}
        </button>
      </div>

      <div v-if="saved" class="text-green-600 dark:text-green-400 text-sm">Settings saved successfully</div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ middleware: 'auth' })

const { settings, fetchSettings, updateSettings, uploadLogo } = useWorkspace()
const { success } = useToast()

const form = reactive({
  company_name: '',
})

const saving = ref(false)
const saved = ref(false)
const selectedFile = ref<File | null>(null)
const uploading = ref(false)
const uploadError = ref('')

onMounted(async () => {
  await fetchSettings()
  form.company_name = settings.value.company_name
})

const handleFileSelect = async (event: Event) => {
  const input = event.target as HTMLInputElement
  if (!input.files?.length) return

  const file = input.files[0]
  selectedFile.value = file
  uploadError.value = ''

  // Auto-upload on select
  uploading.value = true
  try {
    await uploadLogo(file)
    success('Logo uploaded')
  } catch (e: any) {
    uploadError.value = e?.data?.message || 'Upload failed. Max size is 2MB.'
    selectedFile.value = null
  } finally {
    uploading.value = false
  }
}

const handleSave = async () => {
  saving.value = true
  saved.value = false
  try {
    await updateSettings({
      company_name: form.company_name,
    })
    saved.value = true
    success('Settings saved')
    setTimeout(() => { saved.value = false }, 3000)
  } catch (e) {
    console.error(e)
  } finally {
    saving.value = false
  }
}
</script>
