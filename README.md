# Mismo
<img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/RhythmDeolus/mismo/rust_test.yml">
<img alt="GitHub top language" src="https://img.shields.io/github/languages/top/RhythmDeolus/mismo">
<img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/RhythmDeolus/mismo">
<img alt="Github commit activity" src="https://img.shields.io/github/commit-activity/m/RhythmDeolus/mismo" />
<img alt="GitHub contributors" src="https://img.shields.io/github/contributors/RhythmDeolus/mismo">


## Prerequisites

Ensure you have the following installed:
* LLVM >= 14.0
* Rust >= 1.79.0

## Installation

Run the following command in your terminal

```shell
$ git clone https://github.com/RhythmDeolus/mismo.git && cd ./mismo && cargo install
```

## Usage

```
Usage: mismo [OPTIONS] [FILENAME]

Arguments:
  [FILENAME]  Path of the source file

Options:
  -s, --show-debug-info  Print debug info as well
      --show-time        Show time for compilation and execution
  -h, --help             Print help
  -V, --version          Print version
```

### Examples

```shell
$ mismo run

$ mismo run file_name --show-debug-info --show-time
```
