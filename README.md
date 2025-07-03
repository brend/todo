# todo

A dead simple task list manager for the command line.

## Usage

### Create a task

``` sh
$ todo add "Buy milk"
```

This will output the ID of the added task:

```
17
```

### List tasks

``` sh
$ todo list
```

This will print a list of tasks width their IDs, status,
and description:

```
12  [ ] Read README.md
17  [ ] Buy milk
```

### Mark a task as done by ID

``` sh
$ todo complete 1
```

### Delete a task by ID

``` sh
$ todo remove 1
```

### View help

``` sh
$ todo -h
```

### View information

``` sh
$ todo info
```

Right now, this will print the location
of the data file.
