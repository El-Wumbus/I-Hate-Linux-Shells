# I Hate Linux Shells

This whole repository is a lie, but that doesn't mean you shouldn't use this linux shell i'm trying to make.
I'm not a rust programmer.

## Installation

```bash
git clone https://github.com/El-Wumbus/I-Hate-Linux-Shells
cd I-Hate-Linux-Shells
cargo isntall --path=.
```

## Usage

Currently, the shell has two commands:

* cd - Change directories.
* exit - Quit the program.
  
The shell supports running commands in the background with `&` (there must be a space before the `&`).
The shell supports multiple commands per line with `;`, example: `cd ./qinfo;make run`.

```bash
$ ihlsh
decator > echo hello
hello
decator >
```
