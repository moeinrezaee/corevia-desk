const actionCompleted = (response) => response?.status === 'accepted'
  && response?.data?.executed === true

export const createCustomerRequestSelection = () => ({
  screen: true,
  input: false
})

/** Apply the customer's explicit per-session choices before exposing a session. */
export const approveCustomerRequest = async ({ connectionId, screen, input, request }) => {
  if (!screen) return { status: 'screen-required' }

  // The native service may default keyboard access on. Revoke it before the
  // session is authorized, then enable it only after an explicit checked choice.
  const revokeInput = await request(
    'change-connection-permission',
    { connectionId, permission: 'keyboard', enabled: false },
    'nuxt-customer-keyboard-approval'
  )
  if (!actionCompleted(revokeInput)) {
    return { status: 'permission-failed', response: revokeInput }
  }

  const approval = await request(
    'approve-connection',
    { connectionId },
    'nuxt-customer-incoming-approval'
  )
  if (!actionCompleted(approval)) {
    return { status: 'approval-failed', response: approval }
  }

  if (!input) return { status: 'approved', permissions: ['screen'] }

  const enableInput = await request(
    'change-connection-permission',
    { connectionId, permission: 'keyboard', enabled: true },
    'nuxt-customer-keyboard-approval'
  )
  if (!actionCompleted(enableInput)) {
    return {
      status: 'approved-screen-only',
      response: enableInput,
      permissions: ['screen']
    }
  }

  return { status: 'approved', permissions: ['screen', 'input'] }
}
