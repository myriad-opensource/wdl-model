import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    include: ['test/**/*.ts'],
    exclude: ['test/**/*.d.ts', 'test/**/*.bench.ts'],
  },
  benchmark: {
    include: ['test/**/*.bench.ts'],
    exclude: ['test/**/*.d.ts'],
  },
});
