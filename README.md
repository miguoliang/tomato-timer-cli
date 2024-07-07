# Tomato Timer CLI

[![Quality Gate Status][s0]][l0] [![Latest Version][s1]][l1] [![License: MIT][s2]][l2]

<img src="https://miguoliang.github.io/tomato-timer-cli/logo.png" alt="Tomato Timer CLI" height="100px">

---

## Table of Contents

- [Tomato Timer CLI](#tomato-timer-cli)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Features](#features)
  - [Perquisites](#perquisites)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Change Log](#change-log)
  - [Contributing](#contributing)
  - [Code of Conduct](#code-of-conduct)
  - [License](#license)

## Introduction

The Tomato Timer CLI is a command-line tool designed to help you manage your time effectively using the Pomodoro Technique. With this tool, you can break your work into focused intervals called "pomodoros" and take short breaks in between. This technique helps improve productivity and maintain focus throughout your work sessions.

## Features

- Start and stop pomodoro sessions
- Set custom durations for pomodoros and breaks
- Track your progress with a visual timer
- Receive notifications when a pomodoro or break ends unless no audio device found

## Perquisites

- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Installation

Run the following command to install the Tomato Timer CLI. Cargo will download and compile the source code for you.

```bash
cargo install tomato-timer-cli
```

## Usage

To run with default settings, follow the instructions below.

```bash
tomato-timer-cli
```

To get help information, follow the instructions below.

```bash
tomato-timer-cli --help
```

## Change Log

Please refer to the [CHANGELOG](CHANGELOG.md) for more information.

## Contributing

If you would like to contribute to this project, please read the [CONTRIBUTING](CONTRIBUTING.md) guidelines.

## Code of Conduct

Please read the [CODE_OF_CONDUCT](CODE_OF_CONDUCT.md) before contributing.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

[s0]: https://sonarcloud.io/api/project_badges/measure?project=miguoliang_tomato-clock-cli&metric=alert_status
[l0]: https://sonarcloud.io/summary/new_code?id=miguoliang_tomato-clock-cli

[s1]: https://img.shields.io/crates/v/tomato-timer-cli.svg
[l1]: https://crates.io/crates/tomato-timer-cli

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: ./LICENSE
