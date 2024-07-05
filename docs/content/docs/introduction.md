---
title: "Introduction"
weight: 1
# bookFlatSection: false
# bookToc: true
# bookHidden: false
# bookCollapseSection: false
# bookComments: false
# bookSearchExclude: false
---

# Tomato Timer CLI

[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=miguoliang_tomato-clock-cli&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=miguoliang_tomato-clock-cli) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
