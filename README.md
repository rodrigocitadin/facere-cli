# facere-cli

**facere-cli** is a simple command-line tool for managing pending tasks. It allows users to create, list, and update task statuses using a JSON-based local storage.

## Installation

### Install from Crates.io

```sh
cargo install facere-cli
```

### Build from Source

```sh
git clone https://github.com/yourusername/facere-cli.git &&
cd facere-cli &&
cargo install --path .
```

## Usage

### Create a New Task

```sh
facere-cli add <NAME> <YYYY-MM-DD>
```

example:

```sh
facere-cli add "Buy groceries" 2025-02-26
```

### List All Tasks

```sh
facere-cli list
```

### Toggle Task State

```sh
facere-cli toggle <TASK_ID>
```

example:

```sh
facere-cli toggle 1
```

### List Tasks for a Specific Date

```sh
facere-cli filter <YYYY-MM-DD>
```

example

```sh
facere-cli filter 2025-02-26
```

## Storage 

Tasks are stored in a JSON file in the current directory. Future versions may use a more appropriate location.
