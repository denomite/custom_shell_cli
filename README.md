# Custom Shell CLI

A lightweight, custom shell command-line interface (CLI) build with Rust.

## Features

-   **Navigations**: Change directories(cd)
-   **File Operations**: Copy ('cp'), move ('mv'), remove ('rm'), create directories ('mkdir'), create files ('touch'), write to file ('write')
-   **Text Display**: Echo text or strings('echo')
-   **File Viewing**: List files ('ls'/'dir'), read file contents ('cat')
-   **System Utilities**: Show current directory ('pwd'), username ('whoami'), clear screen('clear'/'cls'), system information ('sysinfo')
-   **Command History**: View past commands ('history')
-   **Help System**: List all available commands ('help') or ('assist')
-   **Exit**: Gracefully exit the shell ('exit')
-   **Date/Time**: Display current time, date, and ISO format ('date')
-   **Windows Support**: Includes a compiled executable ('target/release/custom_shell.exe') and custom icon ('icon.ico')
-   **Visuals**: Colored output for commands, errors and messages
-   **Modular Code**: Organized into separate files for maintainability
-   **Cli Parsing**: Powered by the 'clap' crate for robust argument handling

## Instalation

### Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install)(latest stable version recommended)
-   Windows (currently optimized; cross-platform support in progress)

### Usage

-   Start the shell and type commands:
    > > dir
    > > echo Hellow, world!
    > > cd my_folder
    > > help
    > > assist
    > > sysinfo
    > > exit

## Future improvements

-   Tab completion for commands and paths
-   Support for piping (|) and redirection (>, <)
-   Cross-platform compatibility (linux/macOS)
-   Customizable prompt and configuration file
