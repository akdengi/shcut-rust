<template>
  <header class="sticky top-0 z-40 border-b border-gray-200 dark:border-gray-700 bg-white/80 dark:bg-gray-900/80 backdrop-blur-md">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between h-16">
        <!-- Logo -->
        <NuxtLink to="/" class="flex items-center gap-2 shrink-0">
          <template v-if="settings.logo_url">
            <img :src="settings.logo_url" :alt="settings.company_name" class="w-8 h-8 rounded-lg object-contain" />
          </template>
          <template v-else>
            <div class="w-8 h-8 rounded-lg bg-indigo-600 flex items-center justify-center">
              <span class="text-white font-bold text-lg">/</span>
            </div>
          </template>
          <span class="text-xl font-bold text-gray-900 dark:text-white">{{ settings.company_name }}</span>
        </NuxtLink>

        <!-- Right side -->
        <div class="flex items-center gap-3">
          <!-- Dark mode toggle -->
          <button
            @click="toggleDark()"
            class="p-2 rounded-lg text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
            title="Toggle dark mode"
          >
            <svg v-if="isDark" class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
            </svg>
            <svg v-else class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
            </svg>
          </button>

          <template v-if="authStore.isAuthenticated">
            <!-- User menu -->
            <div class="relative" ref="menuRef">
              <button
                @click="showMenu = !showMenu"
                class="flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm font-medium
                       text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
              >
                <div class="w-7 h-7 rounded-full bg-indigo-100 dark:bg-indigo-900 flex items-center justify-center">
                  <span class="text-indigo-600 dark:text-indigo-400 text-xs font-bold">
                    {{ authStore.user?.nickname?.charAt(0).toUpperCase() || '?' }}
                  </span>
                </div>
                <span class="hidden sm:inline">{{ authStore.user?.nickname }}</span>
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
              </button>

              <Transition
                enter-active-class="transition ease-out duration-100"
                enter-from-class="transform opacity-0 scale-95"
                enter-to-class="transform opacity-100 scale-100"
                leave-active-class="transition ease-in duration-75"
                leave-from-class="transform opacity-100 scale-100"
                leave-to-class="transform opacity-0 scale-95"
              >
                <div
                  v-if="showMenu"
                  class="absolute right-0 mt-2 w-48 rounded-xl shadow-lg bg-white dark:bg-gray-800
                         ring-1 ring-black ring-opacity-5 py-1 border border-gray-100 dark:border-gray-700"
                >
                  <NuxtLink
                    to="/settings/profile"
                    class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700"
                    @click="showMenu = false"
                  >
                    Profile Settings
                  </NuxtLink>
                  <NuxtLink
                    v-if="authStore.isAdmin"
                    to="/settings/users"
                    class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700"
                    @click="showMenu = false"
                  >
                    User Management
                  </NuxtLink>
                  <NuxtLink
                    v-if="authStore.isAdmin"
                    to="/settings/workspace"
                    class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700"
                    @click="showMenu = false"
                  >
                    Workspace Settings
                  </NuxtLink>
                  <NuxtLink
                    v-if="authStore.isAdmin"
                    to="/settings/tags"
                    class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700"
                    @click="showMenu = false"
                  >
                    Tag Management
                  </NuxtLink>
                  <hr class="my-1 border-gray-100 dark:border-gray-700" />
                  <button
                    @click="handleLogout"
                    class="w-full text-left px-4 py-2 text-sm text-red-600 dark:text-red-400 hover:bg-gray-50 dark:hover:bg-gray-700"
                  >
                    Sign out
                  </button>
                </div>
              </Transition>
            </div>
          </template>

          <template v-else>
            <NuxtLink
              to="/auth/login"
              class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors"
            >
              Log in
            </NuxtLink>
            <NuxtLink
              to="/auth/signup"
              class="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors"
            >
              Sign up
            </NuxtLink>
          </template>
        </div>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { useAuthStore } from '~/stores/auth'

const authStore = useAuthStore()
const { settings, fetchSettings } = useWorkspace()

onMounted(() => {
  fetchSettings()
})
const colorMode = useColorMode()

const isDark = computed(() => colorMode.value === 'dark')
const toggleDark = () => {
  colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
}

const showMenu = ref(false)
const menuRef = ref<HTMLElement | null>(null)

const handleClickOutside = (e: MouseEvent) => {
  if (menuRef.value && !menuRef.value.contains(e.target as Node)) {
    showMenu.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})

const handleLogout = () => {
  showMenu.value = false
  authStore.logout()
}
</script>
