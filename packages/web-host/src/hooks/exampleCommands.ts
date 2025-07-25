import { useState } from "react";

function ls(path?: string) {
  if (path) {
    return `ls ${path}`;
  }
  return "ls";
}

function cat(path?: string) {
  if (path) {
    return `cat ${path}`;
  }
  return "cat";
}

const commands = [
  "echo foo",
  "echo bar",
  "echo baz",
  "echo $0",
  "echo $ROOT/$USER",
  "greet $USER",
  "echo $0",
  "export USER=WebAssembly",
  "echo $ROOT/$USER",
  "echo $0",
  "echo $?",
  "azertyuiop",
  "echo $?",
  "echo $?",
  () => `export DATE=${new Date().toISOString()}`,
  "echo $DATE",
  "export USER=Tophe",
  "echo $ROOT/$USER",
  () => ls(),
  () => cat("README.md"),
  () => ls(),
  () => ls("data"),
  () => ls("data/processed"),
  () => ls("data/processed/2024"),
  () => ls("documents"),
  () => cat("documents/config.json"),
  "weather Paris",
  "man weather",
  "help",
  "echoc This is the same as `echo`, implemented in C",
  "echoc try `man echo` vs `man echoc`",
  "echoc qux",
];

export function useGetExampleCommand() {
  const [index, setIndex] = useState<number>(0);
  const [command, setCommand] = useState<string>("");
  const [remaining, setRemaining] = useState<number>(commands.length);
  const [done, setDone] = useState<boolean>(false);

  return {
    getExampleCommand: function getExampleCommand() {
      if (commands.length - index - 1 === 0) {
        setDone(true);
        setIndex(0);
      }
      const command = commands[index];
      const output = typeof command === "function" ? command() : command;
      setCommand(output);
      setRemaining((left) => left - 1);
      setIndex((index) => index + 1);
      return output;
    },
    currentExampleCommand: command,
    remainingExampleCommands: remaining,
    doneExampleCommands: done,
  };
}
