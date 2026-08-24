<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 px-4">
    <div class="w-full max-w-md">
      <div class="text-center mb-8">
        <template v-if="settings.logo_url">
          <img :src="settings.logo_url" :alt="settings.company_name" class="w-16 h-16 mx-auto rounded-xl object-contain mb-4" />
        </template>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">{{ settings.company_name }}</h1>
      </div>

      <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-8">
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-2">{{ $t('auth.forgotPassword.heading') }}</h2>
        <p class="text-sm text-gray-600 dark:text-gray-400 mb-6">{{ $t('auth.forgotPassword.description') }}</p>

        <form v-if="!sent" @submit.prevent="handleForgot" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{{ $t('auth.forgotPassword.email') }}</label>
            <input
              v-model="form.email"
              type="email"
              required
              autocomplete="email"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
              :placeholder="$t('auth.forgotPassword.emailPlaceholder')"
            />
          </div>

          <div v-if="error" class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-600 dark:text-red-400 text-sm px-4 py-3 rounded-md">
            {{ error }}
          </div>

          <button
            type="submit"
            :disabled="loading"
            class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
          >
            {{ loading ? $t('auth.forgotPassword.sending') : $t('auth.forgotPassword.sendResetLink') }}
          </button>
        </form>

        <div v-else class="text-center py-4">
          <div class="inline-flex items-center justify-center w-12 h-12 rounded-full bg-green-100 dark:bg-green-900/30 mb-4">
            <svg class="w-6 h-6 text-green-600 dark:text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          </div>
          <p class="text-sm text-gray-600 dark:text-gray-400">{{ $t('auth.forgotPassword.successMessage') }}</p>
        </div>

        <div class="mt-6 text-center text-sm text-gray-600 dark:text-gray-400">
          <NuxtLink to="/auth/login" class="text-blue-600 hover:text-blue-500 font-medium">{{ $t('auth.forgotPassword.backToSignIn') }}</NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: false })

const { t } = useI18n()
const { settings, fetchSettings } = useWorkspace()

onMounted(() => fetchSettings())

const form = reactive({ email: '' })
const loading = ref(false)
const error = ref('')
const sent = ref(false)

const handleForgot = async () => {
  loading.value = true
  error.value = ''
  try {
    await $fetch('/api/v1/auth/forgot-password', {
      method: 'POST',
      body: { email: form.email },
    })
    sent.value = true
  } catch (e: any) {
    if (e?.statusCode === 501) {
      error.value = t('auth.forgotPassword.notConfigured')
    } else {
      error.value = t('auth.forgotPassword.somethingWentWrong')
    }
  } finally {
    loading.value = false
  }
}
</script>
