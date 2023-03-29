# Task Manager CLI

A command-line interface (CLI) for managing tasks.

## Installation

1. Install Rust programming language
2. Clone the repository

```css
git clone https://github.com/YOUR_USERNAME/YOUR_REPOSITORY.git
```

3. Change to the repository directory

```css
cd YOUR_REPOSITORY
```

4. Build and run the project

```css
cargo run
```

## Usage

The following commands are available:

- `add [TASK]`: Add a new task
- `do [ID]`: Complete a task by its ID
- `rm [ID]`: Remove a task by its ID
- `reset [ID]`: Reset a task by its ID
- `ls [--tag TAG] [--status STATUS] [--due DUE] [--priority PRIORITY]`: List all tasks

The `ls` command can be filtered by one or more of the following options:

- `--tag TAG`: Filter by tag
- `--status STATUS`: Filter by status (todo, done, blocked, or hold)
- `--due DUE`: Filter by due date (YYYY-MM-DD, today, tomorrow, thisweek, sometime, or overdue)
- `--priority PRIORITY`: Filter by priority (low, medium, or high)

## Example Output

```css
$ task ls --tag work --status todo --due today --priority high

Due: 2023-03-30 (Thursday)
--------------------------------
# test
[ ][#34 - Low] test
[ ][#35 - Medium] test task

Due: 2023-04-05 (Wednesday)
--------------------------------
[ ][#36 - High] test task 2
```

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
