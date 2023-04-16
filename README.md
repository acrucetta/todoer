# Task CLI Project

This is a CLI application that helps users manage their tasks. Users can add, remove, update, and list tasks using this application. This project is built using Rust programming language and relies on the `clap`, `csv`, and `dotenv` crates.

## Installation

To install this application, follow these steps:

1. Clone the repository to your local machine.
2. Run `cargo build --release` to build the application.
3. Set the `DOER_OUTPUT_DIR` environment variable to the desired output directory for the task list file. Alternatively, you can create a `.env` file in the project directory with the following content:

`DOER_OUTPUT_DIR=/path/to/task/file`

4. Run the application using the command `./target/release/task-cli <SUBCOMMAND> [ARGS]`.

## Usage

The application supports the following subcommands:

- `add`: Add a new task. Example: `task-cli add "Buy groceries"`
- `do`: Mark a task as done. Example: `task-cli do 1`
- `hold`: Put a task on hold. Example: `task-cli hold 1`
- `reset`: Reset a task. Example: `task-cli reset 1`
- `rm`: Remove a task. Example: `task-cli rm 1`
- `ls`: List all tasks with filtering options. Example: `task-cli ls --tag personal --status todo`

The filtering options for the `ls` subcommand are:

- `--tag`: Filter tasks by tag. Example: `task-cli ls --tag personal`
- `--status`: Filter tasks by status. Example: `task-cli ls --status done`
- `--due`: Filter tasks by due date. Example: `task-cli ls --due 2022-01-01`
- `--priority`: Filter tasks by priority. Example: `task-cli ls --priority 1`

The `--view` option for the `ls` subcommand determines how the tasks are displayed. The available options are:

- `tags`: Display the tags for each task. Example: `task-cli ls --view tags`
- `due`: Display the due date for each task. Example: `task-cli ls --view due`

## Example

```
❯ task ls --tag=test --view=tags

# test
---------------
[X][34 - Low] test (03-30)
[X][35 - Low] test task (03-30)
[X][46 - High] Test task (03-31)
[X][36 - Low] test task 2 (04-05)

❯ task ls --tag=test --view=due

Due: 2023-03-30 (Thursday)
--------------------------------
# test
[X][#34 - Low] test
[X][#35 - Low] test task

Due: 2023-03-31 (Friday)
--------------------------------
[X][#46 - High] Test task

Due: 2023-04-05 (Wednesday)
--------------------------------
[X][#36 - Low] test task 2
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
