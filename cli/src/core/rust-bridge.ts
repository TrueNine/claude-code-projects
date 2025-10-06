import { spawn } from 'node:child_process'
import { existsSync } from 'node:fs'
import { dirname, join } from 'node:path'
import process from 'node:process'
import { fileURLToPath } from 'node:url'

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

export interface RustBridgeOptions {
  verbose?: boolean
  timeout?: number
}

/**
 * Bridge to communicate with Rust core engine
 */
export class RustBridge {
  private readonly binaryPath: string

  constructor(private options: RustBridgeOptions = {}) {
    this.binaryPath = this.getRustBinaryPath()
  }

  private getRustBinaryPath(): string {
    const isWindows = process.platform === 'win32'
    const binaryName = isWindows ? 'agents-core.exe' : 'agents-core'
    return join(__dirname, '..', '..', 'rust', 'target', 'release', binaryName)
  }

  private async runCommand(args: string[], input?: string): Promise<any> {
    if (!existsSync(this.binaryPath)) {
      throw new Error(`Rust binary not found at ${this.binaryPath}. Run 'pnpm run build:rust' first.`)
    }

    return new Promise((resolve, reject) => {
      const child = spawn(this.binaryPath, args, {
        stdio: this.options.verbose ? 'inherit' : ['pipe', 'pipe', 'pipe'],
        shell: false,
      })

      let stdout = ''
      let stderr = ''

      if (child.stdout) {
        child.stdout.on('data', (data) => {
          stdout += data.toString()
        })
      }

      if (child.stderr) {
        child.stderr.on('data', (data) => {
          stderr += data.toString()
        })
      }

      if (input && child.stdin) {
        child.stdin.write(input)
        child.stdin.end()
      }

      const timeout = setTimeout(() => {
        child.kill('SIGKILL')
        reject(new Error(`Command timed out after ${this.options.timeout || 30000}ms`))
      }, this.options.timeout || 30000)

      child.on('close', (code) => {
        clearTimeout(timeout)

        if (code === 0) {
          try {
            const result = stdout.trim() ? JSON.parse(stdout.trim()) : { success: true }
            resolve(result)
          } catch {
            // If output is not JSON, return success with raw output
            resolve({ success: true, output: stdout.trim() })
          }
        } else {
          reject(new Error(`Rust command failed with exit code ${code}: ${stderr.trim()}`))
        }
      })

      child.on('error', (error) => {
        clearTimeout(timeout)
        reject(new Error(`Failed to execute Rust binary: ${error.message}`))
      })
    })
  }

  async init(path?: string): Promise<{ success: boolean, path: string }> {
    const args = ['init', '--json']
    if (path) {
      args.push('--path', path)
    }
    return this.runCommand(args)
  }

  async update(options: { backup?: boolean, version?: string } = {}): Promise<{ success: boolean, updated: string[] }> {
    const args = ['update', '--json']
    if (options.backup) {
      args.push('--backup')
    }
    if (options.version) {
      args.push('--version', options.version)
    }
    return this.runCommand(args)
  }

  async compose(type: string, options: { interactive?: boolean, template?: string } = {}): Promise<{ success: boolean, type: string, content: string }> {
    const args = ['compose', type, '--json']
    if (options.interactive) {
      args.push('--interactive')
    }
    if (options.template) {
      args.push('--template', options.template)
    }
    return this.runCommand(args)
  }

  async prune(options: { force?: boolean, dryRun?: boolean } = {}): Promise<{ success: boolean, pruned: string[] }> {
    const args = ['prune', '--json']
    if (options.force) {
      args.push('--force')
    }
    if (options.dryRun) {
      args.push('--dry-run')
    }
    return this.runCommand(args)
  }

  async sync(options: { remote?: string, branch?: string } = {}): Promise<{ success: boolean, synced: string[] }> {
    const args = ['sync', '--json']
    if (options.remote) {
      args.push('--remote', options.remote)
    }
    if (options.branch) {
      args.push('--branch', options.branch)
    }
    return this.runCommand(args)
  }

  async checkHealth(): Promise<{ success: boolean, version: string, platform: string }> {
    return this.runCommand(['--version', '--json'])
  }
}

// Default bridge instance
export const rustBridge: RustBridge = new RustBridge()

// Convenience functions
export const init = (path?: string): Promise<{ success: boolean; path: string }> => rustBridge.init(path)
export const update = (options?: { backup?: boolean; version?: string }): Promise<{ success: boolean; updated: string[] }> => rustBridge.update(options)
export const compose = (type: string, options?: { interactive?: boolean; template?: string }): Promise<{ success: boolean; type: string; content: string }> => rustBridge.compose(type, options)
export const prune = (options?: { force?: boolean; dryRun?: boolean }): Promise<{ success: boolean; pruned: string[] }> => rustBridge.prune(options)
export const sync = (options?: { remote?: string; branch?: string }): Promise<{ success: boolean; synced: string[] }> => rustBridge.sync(options)
export const checkHealth = (): Promise<{ success: boolean; version: string; platform: string }> => rustBridge.checkHealth()
