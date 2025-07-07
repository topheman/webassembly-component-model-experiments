// cycle through the commands in order, if the command is not random (for testing)
let i = 0;

export function getRandomCommand(random = true) {
  const commands = [
    () => "echo foo",
    () => "echo $0",
    () => "echo $ROOT/$USER",
    () =>
      `export USER=${random ? (Math.random() > 0.5 ? "Tophe" : "Topheman") : "Topheman"}`,
    () => "greet $USER",
    () => "azertyuiop",
    () => "echo $?",
    () => "help",
    () => "man weather",
    () => "ls",
    () => "weather Paris",
  ];
  if (random) {
    return commands[Math.floor(Math.random() * commands.length)]();
  }
  const output = commands[i]();
  i++;
  if (i >= commands.length) {
    i = 0;
  }
  return output;
}
