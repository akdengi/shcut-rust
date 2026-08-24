<template>
  <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">{{ $t('settings.apiKeys.heading') }}</h1>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">{{ $t('settings.apiKeys.subtitle') }}</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="navigateTo('/')" class="p-2 rounded-lg text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors" :title="$t('common.close')">
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
        <button
          @click="showCreateForm = true"
          class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors"
        >
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          {{ $t('settings.apiKeys.newKey') }}
        </button>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-16">
      <div class="w-8 h-8 border-2 border-indigo-600 border-t-transparent rounded-full animate-spin" />
    </div>

    <!-- Empty state -->
    <div
      v-else-if="keys.length === 0 && !createdKey"
      class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 overflow-hidden"
    >
      <EmptyState
        :title="$t('settings.apiKeys.emptyState')"
        description=""
      >
        <template #icon>
          <svg class="w-8 h-8 text-gray-400 dark:text-gray-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z" />
          </svg>
        </template>
        <template #action>
          <button
            @click="showCreateForm = true"
            class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors"
          >
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            {{ $t('settings.apiKeys.newKey') }}
          </button>
        </template>
      </EmptyState>
    </div>

    <!-- Newly created key banner -->
    <div
      v-if="createdKey"
      class="mb-6 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-xl p-4"
    >
      <div class="flex items-start gap-3">
        <svg class="w-5 h-5 text-green-600 dark:text-green-400 mt-0.5 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-green-800 dark:text-green-300 mb-2">{{ $t('settings.apiKeys.keyWarning') }}</p>
          <div class="flex items-center gap-2">
            <code class="flex-1 bg-white dark:bg-gray-900 border border-green-200 dark:border-green-800 rounded-lg px-3 py-2 text-sm font-mono text-gray-900 dark:text-white select-all break-all">
              {{ createdKey.key }}
            </code>
            <button
              @click="copyKey(createdKey.key)"
              class="shrink-0 inline-flex items-center gap-1.5 px-3 py-2 text-sm font-medium text-green-700 dark:text-green-300 bg-white dark:bg-gray-900 border border-green-200 dark:border-green-800 rounded-lg hover:bg-green-50 dark:hover:bg-green-900/40 transition-colors"
            >
              <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
              </svg>
              {{ $t('settings.apiKeys.copyKey') }}
            </button>
          </div>
        </div>
        <button
          @click="createdKey = null"
          class="shrink-0 p-1 rounded-lg text-green-600 dark:text-green-400 hover:bg-green-100 dark:hover:bg-green-900/40"
        >
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
    </div>

    <!-- Keys table -->
    <div v-if="keys.length > 0" class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 overflow-hidden">
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
          <thead class="bg-gray-50 dark:bg-gray-800/50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                {{ $t('settings.apiKeys.name') }}
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                {{ $t('settings.apiKeys.prefix') }}
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                {{ $t('settings.apiKeys.created') }}
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                {{ $t('settings.apiKeys.lastUsed') }}
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                {{ $t('settings.apiKeys.expires') }}
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                {{ $t('settings.apiKeys.status') }}
              </th>
              <th class="px-6 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                {{ $t('settings.apiKeys.actions') }}
              </th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
            <tr v-for="key in keys" :key="key.id" class="hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors">
              <td class="px-6 py-4">
                <span class="text-sm font-medium text-gray-900 dark:text-white">{{ key.name }}</span>
              </td>
              <td class="px-6 py-4">
                <div class="flex items-center gap-2">
                  <code class="text-sm font-mono text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 px-2 py-0.5 rounded">{{ key.key_prefix }}...</code>
                  <button
                    @click="copyPrefix(key.key_prefix)"
                    class="p-1 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                    :title="$t('settings.apiKeys.copyKey')"
                  >
                    <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                    </svg>
                  </button>
                </div>
              </td>
              <td class="px-6 py-4 text-sm text-gray-500 dark:text-gray-400">
                {{ new Date(key.created_ts).toLocaleDateString() }}
              </td>
              <td class="px-6 py-4 text-sm text-gray-500 dark:text-gray-400">
                {{ key.last_used_ts ? new Date(key.last_used_ts).toLocaleDateString() : $t('settings.apiKeys.neverUsed') }}
              </td>
              <td class="px-6 py-4 text-sm text-gray-500 dark:text-gray-400">
                {{ key.expires_at ? new Date(key.expires_at).toLocaleDateString() : $t('settings.apiKeys.noExpiry') }}
              </td>
              <td class="px-6 py-4">
                <span
                  :class="[
                    'inline-flex items-center px-2 py-0.5 rounded-md text-xs font-medium',
                    key.is_active
                      ? 'bg-green-50 dark:bg-green-900/30 text-green-700 dark:text-green-400'
                      : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
                  ]"
                >
                  {{ key.is_active ? $t('settings.apiKeys.active') : $t('settings.apiKeys.inactive') }}
                </span>
              </td>
              <td class="px-6 py-4 text-right">
                <div class="flex items-center justify-end gap-2">
                  <button
                    @click="toggleKey(key)"
                    class="text-sm font-medium text-indigo-600 dark:text-indigo-400 hover:underline"
                  >
                    {{ key.is_active ? $t('settings.apiKeys.inactive') : $t('settings.apiKeys.active') }}
                  </button>
                  <button
                    @click="confirmDelete(key)"
                    class="text-sm font-medium text-red-600 dark:text-red-400 hover:underline"
                  >
                    {{ $t('common.delete') }}
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Delete Confirmation -->
    <ConfirmDialog
      v-model="showDeleteConfirm"
      :title="$t('settings.apiKeys.deleteTitle')"
      :message="t('settings.apiKeys.deleteMessage', { name: deletingKey?.name })"
      danger
      :loading="deleteLoading"
      @confirm="handleDelete"
      @cancel="deletingKey = null"
    />

    <!-- Create Key Modal -->
    <Teleport to="body">
      <Transition
        enter-active-class="transition ease-out duration-200"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition ease-in duration-150"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div v-if="showCreateForm" class="fixed inset-0 z-50 flex items-center justify-center p-4">
          <div class="absolute inset-0 bg-black/50" @click="showCreateForm = false" />
          <div class="relative bg-white dark:bg-gray-800 rounded-xl shadow-xl max-w-md w-full p-6">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">{{ $t('settings.apiKeys.createKey') }}</h3>
            <form @submit.prevent="handleCreate" class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{{ $t('settings.apiKeys.keyName') }}</label>
                <input
                  v-model="createForm.name"
                  type="text"
                  required
                  class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2.5 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
                  :placeholder="$t('settings.apiKeys.keyNamePlaceholder')"
                />
              </div>
              <div v-if="authStore.isAdmin">
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{{ $t('settings.apiKeys.forUser') }}</label>
                <select
                  v-model="createForm.user_id"
                  class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2.5 text-sm text-gray-900 dark:text-white focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
                >
                  <option :value="null">{{ $t('settings.apiKeys.myself') }}</option>
                  <option v-for="u in users" :key="u.id" :value="u.id">
                    {{ u.nickname }} ({{ u.email }})
                  </option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">{{ $t('settings.apiKeys.expiresInDays') }}</label>
                <input
                  v-model.number="createForm.expires_in_days"
                  type="number"
                  min="1"
                  class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2.5 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
                  :placeholder="$t('settings.apiKeys.expiresInDaysPlaceholder')"
                />
              </div>
              <div class="flex items-center justify-end gap-3 pt-2">
                <button
                  type="button"
                  @click="showCreateForm = false"
                  class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
                >
                  {{ $t('common.cancel') }}
                </button>
                <button
                  type="submit"
                  :disabled="createSaving"
                  class="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 disabled:opacity-50 transition-colors"
                >
                  {{ createSaving ? $t('settings.apiKeys.creating') : $t('settings.apiKeys.create') }}
                </button>
              </div>
            </form>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from '#imports'
import { useApi } from '~/composables/useApi'
import { useAuthStore } from '~/stores/auth'
import { useToast } from '~/composables/useToast'
import type { ApiKey, ApiKeyCreateResponse, User } from '~/types/api'

definePageMeta({
  middleware: 'auth',
})

const { t } = useI18n()
const api = useApi()
const authStore = useAuthStore()
const toast = useToast()

const loading = ref(true)
const keys = ref<ApiKey[]>([])
const users = ref<User[]>([])
const showCreateForm = ref(false)
const createForm = ref({ name: '', expires_in_days: null as number | null, user_id: null as number | null })
const createSaving = ref(false)
const createdKey = ref<ApiKeyCreateResponse | null>(null)
const deletingKey = ref<ApiKey | null>(null)
const deleteLoading = ref(false)
const showDeleteConfirm = computed({
  get: () => deletingKey.value !== null,
  set: (val: boolean) => { if (!val) deletingKey.value = null },
})

onMounted(async () => {
  try {
    keys.value = await api.get<ApiKey[]>('/api/v1/api-keys')
    if (authStore.isAdmin) {
      users.value = await api.get<User[]>('/api/v1/users')
    }
  } catch {
    toast.error(t('settings.apiKeys.keysLoadFailed'))
  } finally {
    loading.value = false
  }
})

const handleCreate = async () => {
  createSaving.value = true
  try {
    const body: Record<string, any> = { name: createForm.value.name }
    if (createForm.value.expires_in_days) {
      body.expires_in_days = createForm.value.expires_in_days
    }
    if (authStore.isAdmin && createForm.value.user_id) {
      body.user_id = createForm.value.user_id
    }
    const response = await api.post<ApiKeyCreateResponse>('/api/v1/api-keys', body)
    createdKey.value = response
    keys.value.unshift({
      id: response.id,
      name: response.name,
      key_prefix: response.key_prefix,
      created_ts: response.created_ts,
      last_used_ts: null,
      expires_at: response.expires_at,
      is_active: true,
    })
    showCreateForm.value = false
    createForm.value = { name: '', expires_in_days: null, user_id: null }
    toast.success(t('settings.apiKeys.keyCreated'))
  } catch (e: any) {
    toast.error(e?.data?.message || t('settings.apiKeys.createFailed'))
  } finally {
    createSaving.value = false
  }
}

const toggleKey = async (key: ApiKey) => {
  try {
    const updated = await api.put<ApiKey>(`/api/v1/api-keys/${key.id}`, {})
    const idx = keys.value.findIndex((k) => k.id === key.id)
    if (idx !== -1) keys.value[idx] = updated
    toast.success(t('settings.apiKeys.keyToggled'))
  } catch (e: any) {
    toast.error(e?.data?.message || t('settings.apiKeys.toggleFailed'))
  }
}

const confirmDelete = (key: ApiKey) => {
  deletingKey.value = key
  showDeleteConfirm.value = true
}

const handleDelete = async () => {
  if (!deletingKey.value) return
  deleteLoading.value = true
  try {
    await api.del(`/api/v1/api-keys/${deletingKey.value.id}`)
    keys.value = keys.value.filter((k) => k.id !== deletingKey.value!.id)
    toast.success(t('settings.apiKeys.keyRevoked'))
  } catch (e: any) {
    toast.error(e?.data?.message || t('settings.apiKeys.deleteFailed'))
  }
  deleteLoading.value = false
  showDeleteConfirm.value = false
  deletingKey.value = null
}

const copyKey = async (key: string) => {
  try {
    await navigator.clipboard.writeText(key)
    toast.success(t('settings.apiKeys.keyCopied'))
  } catch {
    // Fallback
    const el = document.createElement('textarea')
    el.value = key
    document.body.appendChild(el)
    el.select()
    document.execCommand('copy')
    document.body.removeChild(el)
    toast.success(t('settings.apiKeys.keyCopied'))
  }
}

const copyPrefix = async (prefix: string) => {
  try {
    await navigator.clipboard.writeText(prefix)
    toast.success(t('settings.apiKeys.keyCopied'))
  } catch {
    // ignore
  }
}
</script>
