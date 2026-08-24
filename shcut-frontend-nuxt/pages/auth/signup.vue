<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 px-4">
    <div class="w-full max-w-md">
      <div class="text-center mb-8">
        <template v-if="settings.logo_url">
          <img :src="settings.logo_url" :alt="settings.company_name" class="w-16 h-16 mx-auto rounded-xl object-contain mb-4" />
        </template>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">{{ settings.company_name }}</h1>
        <p class="mt-2 text-gray-600 dark:text-gray-400">{{ $t('auth.signup.subtitle') }}</p>
      </div>

      <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-8">
        <!-- Registration disabled -->
        <div v-if="!registrationAllowed" class="text-center">
          <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 text-yellow-700 dark:text-yellow-400 px-4 py-3 rounded-md mb-4">
            {{ $t('auth.signup.registrationDisabled') }}
          </div>
          <NuxtLink to="/auth/login" class="text-blue-600 hover:text-blue-500 font-medium">
            {{ $t('auth.signup.goToSignIn') }}
          </NuxtLink>
        </div>

        <!-- Registration form -->
        <template v-else>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-6">{{ $t('auth.signup.heading') }}</h2>

          <form @submit.prevent="handleRegister" class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{{ $t('auth.signup.nickname') }}</label>
              <input
                v-model="form.nickname"
                type="text"
                required
                autocomplete="nickname"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                :placeholder="$t('auth.signup.nicknamePlaceholder')"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{{ $t('auth.signup.email') }}</label>
              <input
                v-model="form.email"
                type="email"
                required
                autocomplete="email"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                :placeholder="$t('auth.signup.emailPlaceholder')"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{{ $t('auth.signup.password') }}</label>
              <div class="relative">
                <input
                  v-model="form.password"
                  :type="showPassword ? 'text' : 'password'"
                  required
                  minlength="6"
                  autocomplete="new-password"
                  class="w-full px-3 py-2 pr-10 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                  :placeholder="$t('auth.signup.passwordPlaceholder')"
                />
                <button type="button" @click="showPassword = !showPassword" class="absolute inset-y-0 right-0 flex items-center pr-3 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                  <svg v-if="!showPassword" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                  </svg>
                  <svg v-else class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
                  </svg>
                </button>
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{{ $t('auth.signup.confirmPassword') }}</label>
              <div class="relative">
                <input
                  v-model="form.confirmPassword"
                  :type="showConfirmPassword ? 'text' : 'password'"
                  required
                  minlength="6"
                  autocomplete="new-password"
                  class="w-full px-3 py-2 pr-10 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                  placeholder="••••••"
                />
                <button type="button" @click="showConfirmPassword = !showConfirmPassword" class="absolute inset-y-0 right-0 flex items-center pr-3 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300">
                  <svg v-if="!showConfirmPassword" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                  </svg>
                  <svg v-else class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
                  </svg>
                </button>
              </div>
            </div>

            <div v-if="error" class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 text-red-600 dark:text-red-400 text-sm px-4 py-3 rounded-md">
              {{ error }}
            </div>

            <button
              type="submit"
              :disabled="loading"
              class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
            >
              {{ loading ? $t('auth.signup.creatingAccount') : $t('auth.signup.signUp') }}
            </button>
          </form>

          <div class="mt-6 text-center text-sm text-gray-600 dark:text-gray-400">
            {{ $t('auth.signup.hasAccount') }}
            <NuxtLink to="/auth/login" class="text-blue-600 hover:text-blue-500 font-medium">{{ $t('auth.signup.signIn') }}</NuxtLink>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: false })

const { t } = useI18n()
const authStore = useAuthStore()
const { settings, fetchSettings } = useWorkspace()
const { success } = useToast()

const registrationAllowed = ref(true)

onMounted(async () => {
  await fetchSettings()
  try {
    const data = await $fetch<{ allowed: boolean }>('/api/v1/auth/register-allowed')
    registrationAllowed.value = data.allowed
  } catch {
    registrationAllowed.value = true
  }
})

const form = reactive({ nickname: '', email: '', password: '', confirmPassword: '' })
const showPassword = ref(false)
const showConfirmPassword = ref(false)
const loading = ref(false)
const error = ref('')

const handleRegister = async () => {
  loading.value = true
  error.value = ''
  if (form.password !== form.confirmPassword) {
    error.value = t('auth.signup.passwordsNoMatch')
    loading.value = false
    return
  }
  try {
    await authStore.register(form.email, form.nickname, form.password)
    success(t('auth.signup.accountCreated'))
    navigateTo('/')
  } catch (e: any) {
    if (e?.statusCode === 403) {
      error.value = t('auth.signup.registrationDisabled')
    } else if (e?.statusCode === 409) {
      error.value = t('auth.signup.emailExists')
    } else if (e?.statusCode === 400) {
      error.value = t('auth.signup.invalidInput')
    } else {
      error.value = t('auth.signup.registrationFailed')
    }
  } finally {
    loading.value = false
  }
}
</script>
