# Rust minigrep

![GitHub repo size](https://img.shields.io/github/repo-size/CMIW/Rust-minigrep)
![GitHub contributors](https://img.shields.io/github/contributors/CMIW/Rust-minigrep)
![GitHub stars](https://img.shields.io/github/stars/CMIW/Rust-minigrep?style=social)
![GitHub forks](https://img.shields.io/github/forks/CMIW/Rust-minigrep?style=social)

This is a simple command line tool, taken from the book "The Rust Programming Language" https://doc.rust-lang.org/book/title-page.html, to learn the language trying and practicing common Rust concepts.

The tool is a small version of the classic command tool grep, it takes as arguments a filename or the path of the file we want to search and the string we want to search for, then it reads the file, finds the lines in the file that contain the string argument, and prints those lines. We use an environment variable CASE_INSENSITIVE to set the case sensitivity, 0 for False and 1 for True.

## Prerequisites

Before you begin, follow the Rust installation guide from https://doc.rust-lang.org/book/ch01-01-installation.html.

## Installing Rust minigrep

To install Rust minigrep clone the project repo:

HTTPS:
```
https://github.com/CMIW/Rust-minigrep.git
```

SSH:
```
git@github.com:CMIW/Rust-minigrep.git
```

GitHub CLI:
```
gh repo clone CMIW/Rust-minigrep
```

## Using Rust minigrep

To use Rust minigrep, follow these steps:

* In a terminal move to the project folder named "Rust-minigrep"

* Run the following command
```
CASE_INSENSITIVE=1; cargo run <query to search for> <filename/file path>
```
