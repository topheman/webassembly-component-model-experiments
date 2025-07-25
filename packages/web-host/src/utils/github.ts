const pluginSourceUrlMapping = {
  echo: "https://github.com/topheman/webassembly-component-model-experiments/tree/master/crates/plugin-echo",
  weather:
    "https://github.com/topheman/webassembly-component-model-experiments/tree/master/crates/plugin-weather",
  greet:
    "https://github.com/topheman/webassembly-component-model-experiments/tree/master/crates/plugin-greet",
  ls: "https://github.com/topheman/webassembly-component-model-experiments/tree/master/crates/plugin-ls",
  cat: "https://github.com/topheman/webassembly-component-model-experiments/tree/master/crates/plugin-cat",
  echoc:
    "https://github.com/topheman/webassembly-component-model-experiments/blob/master/c_modules/plugin-echo/component.c",
} as const;

export function getPluginSourceUrl(
  pluginName: keyof typeof pluginSourceUrlMapping | (string & {}),
) {
  if (pluginName in pluginSourceUrlMapping) {
    return pluginSourceUrlMapping[
      pluginName as keyof typeof pluginSourceUrlMapping
    ];
  }
  return null;
}
