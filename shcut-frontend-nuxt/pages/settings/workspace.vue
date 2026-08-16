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

      <!-- Logo URL -->
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
          Logo URL
        </label>
        <input
          v-model="form.logo_url"
          type="url"
          placeholder="https://example.com/logo.png"
          class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none"
        />
        <p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
          Recommended size: 64x64px or larger, square format
        </p>
      </div>

      <!-- Preview -->
      <div v-if="form.company_name || form.logo_url" class="border-t border-gray-200 dark:border-gray-700 pt-6">
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">Preview</label>
        <div class="flex items-center gap-3 p-4 bg-gray-50 dark:bg-gray-900 rounded-lg">
          <template v-if="form.logo_url">
            <img :src="form.logo_url" :alt="form.company_name" class="w-10 h-10 rounded-lg object-contain" />
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

const { settings, fetchSettings, updateSettings } = useWorkspace()
const { success } = useToast()

const form = reactive({
  company_name: '',
  logo_url: '',
})

const saving = ref(false)
const saved = ref(false)

onMounted(async () => {
  await fetchSettings()
  form.company_name = settings.value.company_name
  form.logo_url = settings.value.logo_url
})

const handleSave = async () => {
  saving.value = true
  saved.value = false
  try {
    await updateSettings({
      company_name: form.company_name,
      logo_url: form.logo_url,
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
