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

function ls() {
  return "ls";
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

// cycle through the commands in order, if the command is not random (for testing)
let i = 0;

export function getExampleCommand() {
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
    weather_Paris,
    ls,
    man_weather,
    help,
  ];
  const output = commands[i]();
  i++;
  if (i >= commands.length) {
    i = 0;
  }
  return output;
}
