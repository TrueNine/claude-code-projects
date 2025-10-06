import process from 'node:process'

// Main library entry point
export const VERSION = '0.0.1'

export interface AgentCliOptions {
  useRust?: boolean
  verbose?: boolean
  silent?: boolean
}

export function createAgentCli(options: AgentCliOptions = {}): { version: string; options: AgentCliOptions; init(repositoryPath?: string): Promise<any>; update(): Promise<any>; compose(type: string, template?: string): Promise<any> } {
  return {
    version: VERSION as string,
    options,

    // Core methods that will delegate to either TypeScript or Rust implementations
    async init(repositoryPath?: string) {
      if (options.useRust) {
        return await import('./core/rust-bridge').then((m) => m.init(repositoryPath))
      }
      // TypeScript implementation
      return { success: true, path: repositoryPath || process.cwd() }
    },

    async update() {
      if (options.useRust) {
        return await import('./core/rust-bridge').then((m) => m.update())
      }
      // TypeScript implementation
      return { success: true, updated: [] }
    },

    async compose(type: string, template?: string) {
      if (options.useRust) {
        return await import('./core/rust-bridge').then((m) => m.compose(type, template))
      }
      // TypeScript implementation
      return { success: true, type, content: `Generated ${type} prompt` }
    },
  }
}
