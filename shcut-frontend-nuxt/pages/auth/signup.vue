<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 px-4">
    <div class="w-full max-w-md">
      <div class="text-center mb-8">
        <template v-if="settings.logo_url">
          <img :src="settings.logo_url" :alt="settings.company_name" class="w-16 h-16 mx-auto rounded-xl object-contain mb-4" />
        </template>
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white">{{ settings.company_name }}</h1>
        <p class="mt-2 text-gray-600 dark:text-gray-400">Create your account</p>
      </div>

      <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-8">
        <!-- Registration disabled -->
        <div v-if="!registrationAllowed" class="text-center">
          <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 text-yellow-700 dark:text-yellow-400 px-4 py-3 rounded-md mb-4">
            Registration is currently disabled. Please contact an administrator.
          </div>
          <NuxtLink to="/auth/login" class="text-blue-600 hover:text-blue-500 font-medium">
            Go to Sign in
          </NuxtLink>
        </div>

        <!-- Registration form -->
        <template v-else>
          <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-6">Sign up</h2>

          <form @submit.prevent="handleRegister" class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nickname</label>
              <input
                v-model="form.nickname"
                type="text"
                required
                autocomplete="nickname"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                placeholder="admin"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Email</label>
              <input
                v-model="form.email"
                type="email"
                required
                autocomplete="email"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                placeholder="admin@example.com"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Password</label>
              <input
                v-model="form.password"
                type="password"
                required
                minlength="6"
                autocomplete="new-password"
                class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
                placeholder="••••••"
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
              {{ loading ? 'Creating account...' : 'Sign up' }}
            </button>
          </form>

          <div class="mt-6 text-center text-sm text-gray-600 dark:text-gray-400">
            Already have an account?
            <NuxtLink to="/auth/login" class="text-blue-600 hover:text-blue-500 font-medium">Sign in</NuxtLink>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: false })

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

const form = reactive({ nickname: '', email: '', password: '' })
const loading = ref(false)
const error = ref('')

const handleRegister = async () => {
  loading.value = true
  error.value = ''
  try {
    await authStore.register(form.email, form.nickname, form.password)
    success('Account created!')
    navigateTo('/')
  } catch (e: any) {
    if (e?.statusCode === 403) {
      error.value = 'Registration is disabled'
    } else if (e?.statusCode === 409) {
      error.value = 'User with this email already exists'
    } else if (e?.statusCode === 400) {
      error.value = 'Invalid input. Password must be at least 6 characters.'
    } else {
      error.value = 'Registration failed. Please try again.'
    }
  } finally {
    loading.value = false
  }
}
</script>
