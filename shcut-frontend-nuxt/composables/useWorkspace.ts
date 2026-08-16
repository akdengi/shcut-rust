import { useAuthStore } from '~/stores/auth'

interface WorkspaceSettings {
  company_name: string
  logo_url: string
}

const settings = ref<WorkspaceSettings>({
  company_name: 'ShCut',
  logo_url: '',
})

export const useWorkspace = () => {
  const fetchSettings = async () => {
    try {
      const data = await $fetch<WorkspaceSettings>('/api/v1/settings')
      settings.value = {
        company_name: data.company_name || 'ShCut',
        logo_url: data.logo_url || '',
      }
    } catch {
      // keep defaults
    }
  }

  const updateSettings = async (payload: Partial<WorkspaceSettings>) => {
    const authStore = useAuthStore()
    const data = await $fetch<WorkspaceSettings>('/api/v1/settings', {
      method: 'PUT',
      body: payload,
      headers: authStore.token ? { Authorization: `Bearer ${authStore.token}` } : {},
    })
    settings.value = {
      company_name: data.company_name || 'ShCut',
      logo_url: data.logo_url || '',
    }
    return data
  }

  const uploadLogo = async (file: File): Promise<string> => {
    const authStore = useAuthStore()
    const formData = new FormData()
    formData.append('file', file)

    const data = await $fetch<{ logo_url: string }>('/api/v1/settings/logo', {
      method: 'POST',
      body: formData,
      headers: authStore.token ? { Authorization: `Bearer ${authStore.token}` } : {},
    })

    settings.value.logo_url = data.logo_url
    return data.logo_url
  }

  return {
    settings: readonly(settings),
    fetchSettings,
    updateSettings,
    uploadLogo,
  }
}
