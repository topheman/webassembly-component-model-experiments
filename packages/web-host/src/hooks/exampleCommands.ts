import { useState } from "react";

function echo_foo() {
  return "echo foo";
}

function echo_bar() {
  return "echo bar";
}

function echo_baz() {
  return "echo baz";
}

function echo_$0() {
  return "echo $0";
}

function echo_$ROOT_$USER() {
  return "echo $ROOT/$USER";
}

function export_USER_WebAssembly() {
  return "export USER=WebAssembly";
}

function export_USER_Tophe() {
  return "export USER=Tophe";
}

function greet_USER() {
  return "greet $USER";
}

function azertyuiop() {
  return "azertyuiop";
}

function echo_$question_mark() {
  return "echo $?";
}

function help() {
  return "help";
}

function man_weather() {
  return "man weather";
}

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

function weather_Paris() {
  return "weather Paris";
}

function export_DATE() {
  return `export DATE=${new Date().toISOString()}`;
}

function echo_DATE() {
  return `echo $DATE`;
}

const commands = [
  echo_foo,
  echo_bar,
  echo_baz,
  echo_$0,
  echo_$ROOT_$USER,
  greet_USER,
  echo_$0,
  export_USER_WebAssembly,
  echo_$ROOT_$USER,
  echo_$0,
  echo_$question_mark,
  azertyuiop,
  echo_$question_mark,
  echo_$question_mark,
  export_DATE,
  echo_DATE,
  export_USER_Tophe,
  echo_$ROOT_$USER,
  ls,
  () => cat("README.md"),
  ls,
  () => ls("data"),
  () => ls("data/processed"),
  () => ls("data/processed/2024"),
  () => ls("documents"),
  () => cat("documents/config.json"),
  weather_Paris,
  man_weather,
  help,
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
      const output = commands[index]();
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
