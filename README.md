# Todo CLI Project

A simple, efficient, and extendable command-line tool for managing tasks. This project is designed to help you create, view, update, and delete tasks with various attributes such as title, description, and priority level.

## Table of Contents
- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Documentation](#documentation)
- [License](#license)

## Overview
The **Todo CLI Project** is a Rust-based task management application that allows users to manage tasks with different priorities and statuses. It serves as a practical example of using Rust's powerful type system and command-line interface libraries to build an effective productivity tool.

### Current Status
This project is currently in development and is considered a **pre-release**. We encourage users to try it out and provide feedback, but keep in mind that it may contain unfinished features or incomplete functionality.

## Features
- Create, update, and remove tasks.
- Assign priorities to tasks (High, Medium, Low).
- Mark tasks as done or undone.
- View all tasks, completed tasks, or pending tasks.
- Modify task titles, descriptions, and priorities.
- Clear tasks based on their status or by ID.

## Installation
To install and run the Todo CLI locally, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/todo-cli.git
   cd todo-cli

2. Build the project:
   ```bash

3. Run the executable:
   ```bash

## usage
To use the Todo CLI, you can run the following commands:

- `todo add <title> <description> <priority>`: Add a new task with the specified title, description, and priority level.
- `todo list`: List all tasks.
- `todo list --completed`: List all completed tasks.
- `todo list --pending`: List all pending tasks.
- `todo update <id> <title> <description> <priority>`: Update the task with the specified ID.
- `todo done <id>`: Mark the task with the specified ID as done.
- `todo undone <id>`: Mark the task with the specified ID as undone.
- `todo remove <id>`: Remove the task with the specified ID.
- `todo clear --completed`: Remove all completed tasks.
- `todo clear --pending`: Remove all pending tasks.

## documentation
For more information on how to use the Todo CLI, check out the [documentation](


