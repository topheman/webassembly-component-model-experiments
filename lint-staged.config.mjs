
export default {
  'packages/**': 'biome check --write',
  // `cargo fmt doesn't accept files
  'crates/**': () => 'cargo fmt',
}
