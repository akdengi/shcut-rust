<template>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
      <div>
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Collections</h1>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
          Organize your shortcuts into collections
        </p>
      </div>
      <button
        @click="openCreate"
        class="inline-flex items-center gap-2 px-4 py-2.5 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors shrink-0"
      >
        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        New Collection
      </button>
    </div>

    <!-- Loading -->
    <div v-if="collectionsStore.loading" class="flex justify-center py-16">
      <div class="w-8 h-8 border-2 border-indigo-600 border-t-transparent rounded-full animate-spin" />
    </div>

    <!-- List -->
    <div v-else-if="collectionsStore.items.length" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <CollectionCard
        v-for="collection in collectionsStore.items"
        :key="collection.id"
        :collection="collection"
      />
    </div>

    <!-- Empty -->
    <EmptyState
      v-else
      title="No collections yet"
      description="Create your first collection to organize shortcuts."
    >
      <template #action>
        <button
          @click="openCreate"
          class="inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-indigo-600 rounded-lg hover:bg-indigo-700 transition-colors"
        >
          Create collection
        </button>
      </template>
    </EmptyState>

    <!-- Create Drawer -->
    <Teleport to="body">
      <Transition
        enter-active-class="transition ease-out duration-300"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition ease-in duration-200"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div v-if="showForm" class="fixed inset-0 z-50 flex justify-end">
          <div class="absolute inset-0 bg-black/40" @click="showForm = false" />
          <div class="relative w-full max-w-lg bg-white dark:bg-gray-900 shadow-xl overflow-y-auto">
            <div class="p-6">
              <div class="flex items-center justify-between mb-6">
                <h2 class="text-lg font-semibold text-gray-900 dark:text-white">New Collection</h2>
                <button @click="showForm = false" class="p-1.5 rounded-lg text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800">
                  <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
              <CollectionForm
                @submit="handleFormSubmit"
                @cancel="showForm = false"
              />
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from '#imports'
import { useCollectionsStore } from '~/stores/collections'
import { useToast } from '~/composables/useToast'

definePageMeta({
  middleware: 'auth',
})

const collectionsStore = useCollectionsStore()
const toast = useToast()

const showForm = ref(false)

onMounted(() => {
  collectionsStore.fetchCollections()
})

const openCreate = () => {
  showForm.value = true
}

const handleFormSubmit = async (payload: any) => {
  try {
    await collectionsStore.createCollection(payload)
    toast.success('Collection created')
    showForm.value = false
  } catch (e: any) {
    toast.error(e?.data?.message || 'Failed to create collection')
  }
}
</script>
