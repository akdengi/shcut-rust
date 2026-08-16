import { ref } from 'vue'

export interface Toast {
  id: number
  message: string
  type: 'success' | 'error' | 'info' | 'warning'
  timeout?: number
}

const toasts = ref<Toast[]>([])
let nextId = 0

export const useToast = () => {
  const show = (message: string, type: Toast['type'] = 'info', timeout = 4000) => {
    const id = nextId++
    const toast: Toast = { id, message, type, timeout }
    toasts.value.push(toast)

    if (timeout > 0) {
      setTimeout(() => dismiss(id), timeout)
    }

    return id
  }

  const dismiss = (id: number) => {
    const idx = toasts.value.findIndex((t) => t.id === id)
    if (idx !== -1) toasts.value.splice(idx, 1)
  }

  const success = (msg: string) => show(msg, 'success')
  const error = (msg: string) => show(msg, 'error')
  const info = (msg: string) => show(msg, 'info')
  const warning = (msg: string) => show(msg, 'warning')

  return { toasts, show, dismiss, success, error, info, warning }
}
