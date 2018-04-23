## Delay Command

Delays a command for a number of millseconds. When the same command is sent again before the delay time, the delay is reset.

## Install

### MacOS

```shell
brew tap kjstouffer/delay-cmd github.com/kjstouffer/delay-cmd.git
brew install delay-cmd
```

### Other

`cargo install --git https://github.com/kjstouffer/delay-cmd.git`

## Usage

Start the server with `delay-cmd --server`. This will start a UDP server listening on `127.0.0.1:3400`.

Send a command with `delay-cmd -d <DELAY> <CMD>`

## Pitfalls
- current max command length is 1024 characters
- commands which rely on PWD or CWD will use the server's working directory, which could have unexpected behavior
- when sending a command, proper escape sequences must be made eg. `delay-cmd "echo \"Hello World!\""`


