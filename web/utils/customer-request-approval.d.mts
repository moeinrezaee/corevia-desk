import type { CoreviaBridgeOperation, CoreviaBridgeResponse } from '../types/corevia-bridge'

export type CustomerRequestApprovalResult =
  | { status: 'screen-required' }
  | { status: 'permission-failed' | 'approval-failed'; response: CoreviaBridgeResponse }
  | { status: 'approved-screen-only'; response: CoreviaBridgeResponse; permissions: string[] }
  | { status: 'approved'; permissions: string[] }

export interface CustomerRequestSelection {
  screen: boolean
  input: boolean
}

export function createCustomerRequestSelection(): CustomerRequestSelection

export function approveCustomerRequest(options: {
  connectionId: string | number
  screen: boolean
  input: boolean
  request: (
    operation: CoreviaBridgeOperation,
    payload?: Record<string, unknown>,
    approvalReference?: string
  ) => Promise<CoreviaBridgeResponse>
}): Promise<CustomerRequestApprovalResult>
