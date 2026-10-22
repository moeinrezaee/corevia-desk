import { spawn } from 'node:child_process'
import { createInterface } from 'node:readline'

// The MCP server talks to a local Corevia Desk process over stdio. It never
// opens a public port and it never accepts an arbitrary command from MCP.

export const createUnavailableRustBridge = () => ({
  async invoke(request) {
    return {
      requestId: request.requestId,
      operation: request.operation,
      surface: 'mcp',
      profile: request.profile,
      status: 'unsupported',
      error: {
        code: 'local-agent-required',
        message: 'A local Corevia Desk native agent is required for this action.'
      }
    }
  }
})

export const createRustBridgeAdapter = ({ dispatch, setApprovalKey = () => {} }) => {
  if (typeof dispatch !== 'function') {
    throw new TypeError('A bounded Rust bridge dispatch function is required.')
  }

  return {
    setApprovalKey(key) {
      setApprovalKey(Buffer.from(key))
    },
    async invoke(request) {
      return dispatch({
        ...request,
        surface: 'mcp'
      })
    }
  }
}

export const createProcessRustBridge = ({
  executable = process.env.COREVIA_DESK_RUST_BRIDGE,
  args = ['--corevia-mcp-bridge'],
  cwd,
  env = {},
  timeoutMs = 30_000
} = {}) => {
  let child
  let output
  let reader
  let sequence = 0
  const pending = new Map()

  const rejectPending = (error) => {
    for (const { reject } of pending.values()) reject(error)
    pending.clear()
  }

  const start = () => {
    if (child) return
    if (!executable) {
      throw new Error('COREVIA_DESK_RUST_BRIDGE is not configured.')
    }

    child = spawn(executable, args, {
      cwd,
      env: { ...process.env, ...env },
      stdio: ['pipe', 'pipe', 'inherit']
    })
    reader = createInterface({ input: child.stdout, crlfDelay: Infinity })
    reader.on('line', (line) => {
      if (!line.trim()) return
      let response
      try {
        response = JSON.parse(line)
      } catch {
        return
      }
      const waiter = pending.get(response.requestId)
      if (!waiter) return
      pending.delete(response.requestId)
      clearTimeout(waiter.timer)
      waiter.resolve(response)
    })
    child.once('error', (error) => {
      rejectPending(error)
      child = undefined
    })
    child.once('exit', (code, signal) => {
      const error = new Error(`The Corevia Desk native bridge exited (${code ?? signal ?? 'unknown'}).`)
      rejectPending(error)
      child = undefined
      reader?.close()
      reader = undefined
    })
  }

  const close = () => {
    rejectPending(new Error('The Corevia Desk native bridge was closed.'))
    reader?.close()
    reader = undefined
    if (child && !child.killed) child.kill()
    child = undefined
  }

  return {
    async invoke(request) {
      try {
        start()
      } catch (error) {
        return {
          requestId: request.requestId,
          operation: request.operation,
          surface: 'mcp',
          profile: request.profile,
          status: 'unsupported',
          error: {
            code: 'local-agent-required',
            message: error instanceof Error ? error.message : 'A local Corevia Desk native agent is required.'
          }
        }
      }

      const requestId = `${request.requestId}-${(++sequence).toString(36)}`
      const wireRequest = { ...request, requestId }
      return new Promise((resolve) => {
        const timer = setTimeout(() => {
          pending.delete(requestId)
          resolve({
            requestId,
            operation: request.operation,
            surface: 'mcp',
            profile: request.profile,
            status: 'error',
            error: {
              code: 'bridge-timeout',
              message: 'The local Corevia Desk bridge did not respond in time.'
            }
          })
        }, timeoutMs)
        pending.set(requestId, { resolve, reject: resolve, timer })
        try {
          child.stdin.write(`${JSON.stringify(wireRequest)}\n`)
        } catch {
          clearTimeout(timer)
          pending.delete(requestId)
          resolve({
            requestId,
            operation: request.operation,
            surface: 'mcp',
            profile: request.profile,
            status: 'error',
            error: {
              code: 'bridge-unavailable',
              message: 'The local Corevia Desk bridge could not receive the request.'
            }
          })
        }
      })
    },
    close
  }
}

export const createConfiguredRustBridge = (options = {}) => {
  return process.env.COREVIA_DESK_RUST_BRIDGE
    ? createProcessRustBridge(options)
    : createUnavailableRustBridge()
}
