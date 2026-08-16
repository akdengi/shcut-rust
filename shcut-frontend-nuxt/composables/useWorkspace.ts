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
    const data = await $fetch<WorkspaceSettings>('/api/v1/settings', {
      method: 'PUT',
      body: payload,
    })
    settings.value = {
      company_name: data.company_name || 'ShCut',
      logo_url: data.logo_url || '',
    }
    return data
  }

  return {
    settings: readonly(settings),
    fetchSettings,
    updateSettings,
  }
}
