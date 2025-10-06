import { defineConfig } from 'tsdown'

export default defineConfig({
  entry: ['./src/**/*', '!**/*.{spec,test}.*'],
  platform: 'node',
  sourcemap: true,
  unbundle: true,
  format: ['esm'],
  dts: {
    sourcemap: true,
    tsconfig: './tsconfig.lib.json',
  },
})
