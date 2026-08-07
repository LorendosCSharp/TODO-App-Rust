
# Todo-CLI

Todo-CLI is a simple command-line task manager written in Rust. This project was created as a way to learn the language and explore some of its core concepts.

While building this project, I learned about:

-   Ownership and borrowing
    
-   Structs and vectors
    
-   Building command-line applications with `clap`
    
-   Serializing and deserializing Rust structs to JSON using `serde`
    

## Usage

The general command syntax is:

```text
todo <COMMAND> [OPTIONS]

```

The following commands are currently supported.

## Add a Task

Creates a new task.

```text
todo add <NAME> <DESCRIPTION> <TILL>

```

### Arguments

```text
<NAME>          Name of the task.
<DESCRIPTION>   Short description of the task.
<TILL>          Due date (currently stored as a string).

```

### Result

Returns a randomly generated 8-character task ID.

----------

## Remove a Task

Deletes a task.

```text
todo rem <ID>

```

### Arguments

```text
<ID>            ID of the task to remove.

```

### Result

Prints a confirmation message after the task has been removed.

----------

## Mark a Task as Done

Marks a task as completed.

```text
todo done <ID>

```

### Arguments

```text
<ID>            ID of the task.

```

### Result

Sets the task status to `true` and prints the updated task.
Formatted as "ID {ID} | {name} | {description} due to {till} {status}".

----------

## List Tasks

Lists tasks based on their completion status.

```text
todo list <SortingType>

```

### Arguments

```text
<SortingType>   all (default), done, undone

```

### Result

Displays all tasks matching the selected filter.

----------

## Modify a Task

Updates one or more fields of an existing task.

```text
todo modify <ID> -n <NAME> -d <DESCRIPTION> -t <TILL> -s <STATUS>

```

### Arguments

```text
<ID>            Required task ID.
<NAME>          -n, --name
<DESCRIPTION>   -d, --description
<TILL>          -t, --till
<STATUS>        -s, --status

```

### Result

Updates the specified fields of the task if the ID exists.

----------

## Search Tasks

Searches for tasks using one or more filters.
P.S. todo list uses the same function as this one

```text
todo search -i <ID> -n <NAME> -t <TILL> -s <STATUS>

```

### Arguments

```text
<ID>            -i, --id
<NAME>          -n, --name
<TILL>          -t, --till
<STATUS>        -s, --status

```

### Search Behavior

Searching by **ID** looks up a single task directly because each task has a unique ID.

When searching by **name**, **due date**, or **status**, the program performs a fallthrough search. Each filter is applied to the results of the previous one, reducing the set of matching tasks step by step.

For example:

```text
todo search -n Homework -t 2026-08-10 -s false

```

The search is performed in this order:

1.  Find all tasks named `Homework`.
    
2.  From those results, keep only tasks due on `2026-08-10`.
    
3.  From the remaining tasks, keep only those whose status is `false`.
    

----------

## Help

Display the built-in help.

```text
todo help

```

Every command also supports the standard help flags:

```text
-h
--help

```
P.S. the flags short variant can differ, because `clap` has auto-generated it for me and I have not checked them all
