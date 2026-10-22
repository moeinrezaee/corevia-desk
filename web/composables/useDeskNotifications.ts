import { computed, ref } from 'vue'

export type DeskNotificationTone = 'success' | 'warning' | 'danger' | 'info'

export interface DeskNotification {
  id: number
  message: string
  tone: DeskNotificationTone
}

const notifications = ref<DeskNotification[]>([])
const timers = new Map<number, ReturnType<typeof setTimeout>>()
let sequence = 0

const dismiss = (id: number) => {
  const timer = timers.get(id)
  if (timer) clearTimeout(timer)
  timers.delete(id)
  notifications.value = notifications.value.filter((notice) => notice.id !== id)
}

export const useDeskNotifications = () => {
  const push = (message: string, tone: DeskNotificationTone = 'info', duration = 4200) => {
    if (!message) return 0
    const id = ++sequence
    notifications.value = [...notifications.value, { id, message, tone }].slice(-4)
    if (duration > 0) {
      timers.set(id, setTimeout(() => dismiss(id), duration))
    }
    return id
  }

  return {
    notifications: computed(() => notifications.value),
    push,
    dismiss,
    success: (message: string) => push(message, 'success'),
    warning: (message: string) => push(message, 'warning'),
    danger: (message: string) => push(message, 'danger'),
    info: (message: string) => push(message, 'info')
  }
}
