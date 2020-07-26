# Project Euler

Solving Project Euler problems in various programming languages such as Rust, C++, Go, or Python!

Project Euler website: <https://projecteuler.net>

## Setup

- If using Rust, install it! At least version 1.44 is required.

<https://www.rust-lang.org/tools/install>

- If using Python, install it! At least version 3.8 is required.

<https://www.python.org/downloads/>

- If using C++, install g++ or some compiler! A fairly recent version of C++ is required.

Compiler g++ is often pre-installed on operating systems. But if any issue, Google how to do it :)

- If using Go, install it! Version 1.14 or above is recommended.

<https://golang.org/dl/>

## Usage

Run a problem by specifying its number (e.g., 1) and its version (filename without extention, e.g.: v1):

```
make run-rust PROBLEM=1 VERSION=v1
```

```
make run-python PROBLEM=1 VERSION=v1
```

```
make run-cpp PROBLEM=1 VERSION=v1
```

If supported, optimized runs are also available with:

```
make optimized-run-rust PROBLEM=1 VERSION=v1
```

```
make optimized-run-cpp PROBLEM=1 VERSION=v1
```

For GoLang, no main file has been created at the moment (ever?), so just use:

```
go run src/problems/[folder]/[file].go
```
