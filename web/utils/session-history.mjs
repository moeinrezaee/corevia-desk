const normalized = (value) => typeof value === 'string'
  ? value.trim().replace(/\/r$/iu, '').toLowerCase()
  : ''

const unavailableNames = new Set(['not available', 'در دسترس نیست'])

/** @param {{ id?: string, peerId?: string, address?: string, name?: string, platform?: string }} session */
export const sessionDeviceKey = (session) => {
  const peerId = normalized(session?.peerId)
  if (peerId) return `peer:${peerId}`

  const address = normalized(session?.address)
  if (address) return `address:${address}`

  const name = normalized(session?.name)
  if (name && !unavailableNames.has(name)) {
    return `name:${normalized(session?.platform)}:${name}`
  }

  const id = normalized(session?.id)
  return id ? `session:${id}` : session
}

const endedAt = (session) => Number.isFinite(session?.endedAt) ? session.endedAt : 0

/** Keep the latest terminal session for each peer while preserving raw session records. */
export const latestSessionPerDevice = (sessions) => {
  const latestByDevice = new Map()

  for (const session of sessions) {
    const key = sessionDeviceKey(session)
    const current = latestByDevice.get(key)
    if (!current || endedAt(session) > endedAt(current)) latestByDevice.set(key, session)
  }

  return [...latestByDevice.values()].sort((left, right) => endedAt(right) - endedAt(left))
}
