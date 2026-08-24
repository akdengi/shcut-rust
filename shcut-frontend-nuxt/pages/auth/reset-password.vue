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
        <template v-if="!success">
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-6">{{ $t('auth.resetPassword.heading') }}</h2>

          <form @submit.prevent="handleReset" class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{{ $t('auth.resetPassword.newPassword') }}</label>
              <input
                v-model="form.new_password"
                type="password"
                required
                minlength="6"
                autocomplete="new-password"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                :placeholder="$t('auth.resetPassword.minChars')"
              />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{{ $t('auth.resetPassword.confirmPassword') }}</label>
              <input
                v-model="form.confirm_password"
                type="password"
                required
                minlength="6"
                autocomplete="new-password"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                :placeholder="$t('auth.resetPassword.minChars')"
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
              {{ loading ? $t('auth.resetPassword.resetting') : $t('auth.resetPassword.resetPassword') }}
            </button>
          </form>
        </template>

        <div v-else class="text-center py-4">
          <div class="inline-flex items-center justify-center w-12 h-12 rounded-full bg-green-100 dark:bg-green-900/30 mb-4">
            <svg class="w-6 h-6 text-green-600 dark:text-green-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          </div>
          <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">{{ $t('auth.resetPassword.successMessage') }}</p>
          <NuxtLink to="/auth/login" class="text-blue-600 hover:text-blue-500 font-medium text-sm">{{ $t('auth.resetPassword.signInWithNewPassword') }}</NuxtLink>
        </div>

        <div class="mt-6 text-center text-sm text-gray-600 dark:text-gray-400">
          <NuxtLink to="/auth/login" class="text-blue-600 hover:text-blue-500 font-medium">{{ $t('auth.resetPassword.backToSignIn') }}</NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: false })

const { t } = useI18n()
const route = useRoute()
const { settings, fetchSettings } = useWorkspace()

onMounted(() => fetchSettings())

const token = computed(() => route.query.token as string || '')

const form = reactive({ new_password: '', confirm_password: '' })
const loading = ref(false)
const error = ref('')
const success = ref(false)

const handleReset = async () => {
  if (form.new_password !== form.confirm_password) {
    error.value = t('auth.resetPassword.passwordsNoMatch')
    return
  }
  if (!token.value) {
    error.value = t('auth.resetPassword.missingToken')
    return
  }
  loading.value = true
  error.value = ''
  try {
    await $fetch('/api/v1/auth/reset-password', {
      method: 'POST',
      body: { token: token.value, new_password: form.new_password },
    })
    success.value = true
  } catch (e: any) {
    if (e?.statusCode === 400) {
      error.value = t('auth.resetPassword.invalidOrExpiredToken')
    } else {
      error.value = t('auth.resetPassword.somethingWentWrong')
    }
  } finally {
    loading.value = false
  }
}
</script>
