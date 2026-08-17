<template>
  <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">User Management</h1>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Admin only</p>
      </div>
      <div class="flex items-center gap-2">
        <button @click="navigateTo('/')" class="p-2 rounded-lg text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors" title="Close">
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
          New User
        </button>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-16">
      <div class="w-8 h-8 border-2 border-indigo-600 border-t-transparent rounded-full animate-spin" />
    </div>

    <!-- Users table -->
    <div v-else class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 overflow-hidden">
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
          <thead class="bg-gray-50 dark:bg-gray-800/50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                User
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Role
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Created
              </th>
              <th class="px-6 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                Actions
              </th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
            <tr v-for="user in users" :key="user.id" class="hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors">
              <td class="px-6 py-4">
                <div class="flex items-center gap-3">
                  <div class="w-8 h-8 rounded-full bg-indigo-100 dark:bg-indigo-900 flex items-center justify-center shrink-0">
                    <span class="text-indigo-600 dark:text-indigo-400 text-xs font-bold">
                      {{ user.nickname?.charAt(0).toUpperCase() || '?' }}
                    </span>
                  </div>
                  <div>
                    <p class="text-sm font-medium text-gray-900 dark:text-white">{{ user.nickname }}</p>
                    <p class="text-xs text-gray-500 dark:text-gray-400">{{ user.email }}</p>
                  </div>
                </div>
              </td>
              <td class="px-6 py-4">
                <span
                  :class="[
                    'inline-flex items-center px-2 py-0.5 rounded-md text-xs font-medium',
                    user.role === 'admin'
                      ? 'bg-purple-50 dark:bg-purple-900/30 text-purple-700 dark:text-purple-400'
                      : 'bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
                  ]"
                >
                  {{ user.role }}
                </span>
              </td>
              <td class="px-6 py-4 text-sm text-gray-500 dark:text-gray-400">
                {{ new Date(user.created_ts).toLocaleDateString() }}
              </td>
              <td class="px-6 py-4 text-right">
                <div class="flex items-center justify-end gap-2">
                  <button
                    @click="openEdit(user)"
                    class="text-sm font-medium text-indigo-600 dark:text-indigo-400 hover:underline"
                  >
                    Edit
                  </button>
                  <button
                    v-if="user.id !== currentUserId"
                    @click="confirmDelete(user)"
                    class="text-sm font-medium text-red-600 dark:text-red-400 hover:underline"
                  >
                    Delete
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Edit User Drawer -->
    <Teleport to="body">
      <Transition
        enter-active-class="transition ease-out duration-300"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition ease-in duration-200"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div v-if="showEditForm" class="fixed inset-0 z-50 flex justify-end">
          <div class="absolute inset-0 bg-black/40" @click="showEditForm = false" />
          <div class="relative w-full max-w-lg bg-white dark:bg-gray-900 shadow-xl overflow-y-auto">
            <div class="p-6">
              <div class="flex items-center justify-between mb-6">
                <h2 class="text-lg font-semibold text-gray-900 dark:text-white">Edit User</h2>
                <button @click="showEditForm = false" class="p-1.5 rounded-lg text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800">
                  <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>

              <form @submit.prevent="handleEditSave" class="space-y-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nickname</label>
                  <input
                    v-model="editForm.nickname"
                    type="text"
                    class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2.5 text-sm text-gray-900 dark:text-white focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Email</label>
                  <input
                    v-model="editForm.email"
                    type="email"
                    class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2.5 text-sm text-gray-900 dark:text-white focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
                  />
                </div>
                <div v-if="editingUser && editingUser.role !== 'admin' && editingUser.id !== currentUserId">
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Role</label>
                  <select
                    v-model="editForm.role"
                    class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 px-3 py-2.5 text-sm text-gray-900 dark:text-white focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
                  >
                    <option value="user">User - can create/edit shortcuts</option>
                    <option value="view">View - can only view shortcuts</option>
                  </select>
                </div>

                <div class="flex items-center justify-end gap-3 pt-2">
                  <button
                    type="button"
                    @click="showEditForm = false"
                    class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
                  >
                    Cancel
                  </button>
                  <button
                    type="submit"
                    :disabled="editSaving"
                    class="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 disabled:opacity-50 transition-colors"
                  >
                    {{ editSaving ? 'Saving...' : 'Save' }}
                  </button>
                </div>
              </form>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- Delete Confirmation -->
    <ConfirmDialog
      v-if="deletingUser"
      title="Delete User"
      :message="`Are you sure you want to delete user '${deletingUser.nickname}' (${deletingUser.email})?`"
      @confirm="handleDelete"
      @cancel="deletingUser = null"
    />

    <!-- Create User Modal -->
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
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Create User</h3>
            <form @submit.prevent="handleCreate" class="space-y-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nickname</label>
                <input
                  v-model="createForm.nickname"
                  type="text"
                  required
                  class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2.5 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
                  placeholder="John Doe"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Email</label>
                <input
                  v-model="createForm.email"
                  type="email"
                  required
                  class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2.5 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
                  placeholder="john@example.com"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Password</label>
                <input
                  v-model="createForm.password"
                  type="password"
                  required
                  minlength="6"
                  class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2.5 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
                  placeholder="Min 6 characters"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Role</label>
                <select
                  v-model="createForm.role"
                  class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 px-3 py-2.5 text-sm text-gray-900 dark:text-white focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 outline-none transition-colors"
                >
                  <option value="user">User - can create/edit shortcuts</option>
                  <option value="view">View - can only view shortcuts</option>
                </select>
              </div>
              <div class="flex items-center justify-end gap-3 pt-2">
                <button
                  type="button"
                  @click="showCreateForm = false"
                  class="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg transition-colors"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  :disabled="createSaving"
                  class="px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 disabled:opacity-50 transition-colors"
                >
                  {{ createSaving ? 'Creating...' : 'Create' }}
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
import type { User } from '~/types/api'

definePageMeta({
  middleware: 'auth',
})

const api = useApi()
const authStore = useAuthStore()
const toast = useToast()

const loading = ref(true)
const users = ref<User[]>([])
const showEditForm = ref(false)
const editingUser = ref<User | null>(null)
const editForm = ref({ nickname: '', email: '', role: 'view' as string })
const editSaving = ref(false)
const deletingUser = ref<User | null>(null)
const showCreateForm = ref(false)
const createForm = ref({ nickname: '', email: '', password: '', role: 'view' as string })
const createSaving = ref(false)

const currentUserId = computed(() => authStore.user?.id)

onMounted(async () => {
  try {
    users.value = await api.get<User[]>('/api/v1/users')
  } catch {
    toast.error('Failed to load users')
  } finally {
    loading.value = false
  }
})

const openEdit = (user: User) => {
  editingUser.value = user
  editForm.value = { nickname: user.nickname, email: user.email, role: user.role }
  showEditForm.value = true
}

const handleEditSave = async () => {
  if (!editingUser.value) return
  editSaving.value = true
  try {
    const updated = await api.put<User>(`/api/v1/users/${editingUser.value.id}`, {
      nickname: editForm.value.nickname,
      email: editForm.value.email,
      role: editForm.value.role,
    })
    const idx = users.value.findIndex((u) => u.id === updated.id)
    if (idx !== -1) users.value[idx] = updated
    showEditForm.value = false
    toast.success('User updated')
  } catch (e: any) {
    toast.error(e?.data?.message || 'Failed to update user')
  } finally {
    editSaving.value = false
  }
}

const confirmDelete = (user: User) => {
  deletingUser.value = user
}

const handleCreate = async () => {
  createSaving.value = true
  try {
    const newUser = await api.post<User>('/api/v1/users', {
      nickname: createForm.value.nickname,
      email: createForm.value.email,
      password: createForm.value.password,
      role: createForm.value.role,
    })
    users.value.unshift(newUser)
    showCreateForm.value = false
    createForm.value = { nickname: '', email: '', password: '', role: 'view' }
    toast.success('User created')
  } catch (e: any) {
    if (e?.statusCode === 409) {
      toast.error('User with this email already exists')
    } else {
      toast.error('Failed to create user')
    }
  } finally {
    createSaving.value = false
  }
}

const handleDelete = async () => {
  if (!deletingUser.value) return
  try {
    await api.del(`/api/v1/users/${deletingUser.value.id}`)
    users.value = users.value.filter(u => u.id !== deletingUser.value!.id)
    toast.success('User deleted')
  } catch (e: any) {
    toast.error(e?.data?.message || 'Failed to delete user')
  }
  deletingUser.value = null
}
</script>
