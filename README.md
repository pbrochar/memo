# Memo CLI

Memo CLI is a command-line tool for managing key-value pairs with optional time-to-live (TTL) values. This tool allows you to add, get, remove, list, set, and copy memo entries.

## Requirements

Memo CLI requires Rust to be installed on your system.

## Installation

To install Memo CLI, clone the repository and install using install script:

```sh
curl https://raw.githubusercontent.com/pbrochar/memo/refs/heads/main/install.sh | bash
```

## Usage

### add

Add a new memo entry with a key and value. Optionally, you can specify a TTL value in seconds.

```sh
memo add mykey "This is a value" --ttl 3600
```

### get

Get the value of a memo entry by key.

```sh
memo get mykey
```

You can get the value in your clipboard by using the `--clipboard or -c` flag.

```sh
memo get -c mykey
```

### cp

Copy the value of a memo entry to your clipboard.
It's a shortcut for `memo get -c`.
```sh
memo cp mykey
```

### set

Set the value of a memo entry by key.
It's useful if you want to update a key.


### rm

Remove a memo entry by key.

### list

List all memo entries.
You can use the --pretty flag to display the list in a more readable format.

## Auto-completion

Memo CLI supports dynamic auto-completion.
For exemple:
```bash
memo get <tab>
```

