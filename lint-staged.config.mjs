
export default {
  'packages/*/!(overrides)/**': 'biome check --write',
  'packages/plugin-echo/**/*.ts': () => 'npm run typecheck --workspace=packages/plugin-echo',
  'packages/web-host/**/*.{ts,tsx}': () => 'npm run typecheck --workspace=packages/web-host',
  // `cargo fmt doesn't accept files
  'crates/**': () => 'cargo fmt',
}
