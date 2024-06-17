# Changelog

All notable changes to the `tomato-clock-cli` project will be documented in this file.

## [0.1.0] - 2024-06-13

### Added

- Start and stop pomodoro sessions
- Set custom durations for pomodoros and breaks
- Track your progress with a visual timer
- Receive notifications when a pomodoro or break ends

[0.1.0]: https://github.com/olivierlacan/keep-a-changelog/releases/tag/v0.1.0

## v0.1.0 (2024-06-17)

### Chore

 - <csr-id-12790a00cb1dc328837701f1e8ace4143a96273b/> Update email addresses in code of conduct and security files
 - <csr-id-d302a0c372677d834b5f3398aa14311f78e4518e/> Update CONTRIBUTING.md with a more detailed contributing guide
 - <csr-id-64402904c24b7d401c19722075f8d33aa38fef46/> Update markdownlint configuration to disable specific rules
 - <csr-id-88487a27a237562a97fd25eecbcc7b017bf7c976/> Update SonarCloud configuration to include source code in 'src' directory
 - <csr-id-14d9d8d987908dd637b9ebf54501e8ad60a80e6b/> Add SonarCloud configuration files for code analysis
 - <csr-id-795cd5090cb513e7528178141ae6d99515ee2914/> Add SonarCloud workflow for code analysis
 - <csr-id-20f9cb6a4b3f72ebfb74d8aa4baad6723a8a3d01/> Update Cargo.toml, introduction page, and Hugo configuration for Tomato Clock CLI integration
 - <csr-id-c1da29015d671b7af3e35db17fb6eef7db2899e4/> Update Cargo.toml and introduction page with Tomato Timer CLI details and features
 - <csr-id-7bebfe8b62e789afe0a1bfec5d5cf0f0515db88e/> Update introduction page with Tomato Timer CLI details and features
 - <csr-id-444e7e0badf8a285c843c092e72bc85796ee5de9/> Remove outdated Hugo GitHub Actions workflow
 - <csr-id-490566706326c8b44d291b0191f9cc27ab660ef5/> Update default branch name in GitHub Actions workflow
 - <csr-id-f1b9f8769b96b8929144babe1c6f6281aafe537f/> Update default branch name in GitHub Actions workflow
 - <csr-id-bb7c19417d3df0f79b02f541e5afdaa6d89e8b79/> Add GitHub Actions workflow for building and deploying Hugo website
 - <csr-id-b4a2d35169c19671ee0cc166a1579e47a87ed2e7/> Add Hugo theme submodule and configuration files
 - <csr-id-70175ebb16b2c4946c57ebf7d45c5b31055c8729/> Add CONTRIBUTING.md file
 - <csr-id-794b24974cb867b2d9dfbea22060be846f832bde/> Add security policy documentation
 - <csr-id-46744bac17a607fdb74030a4795c76649ed5f404/> Add Gitpod configuration files
 - <csr-id-3c9b9d33a912916c0cc8e35b7744358b29168569/> Refactor interval timer code to support customizable break durations

### Refactor

 - <csr-id-e282ddfdd139d2fe19510fae41562bb3df6a7b53/> Add support for long breaks and intervals
 - <csr-id-1844ee8396ac65fe6de58ecfe924219182dddc47/> Add support for long breaks and intervals
   This commit refactors the code to add support for long breaks and intervals. It introduces a new struct `Interval` with properties such as name, color, duration, and message_done. The `execute_interval` function is modified to handle both short breaks and long breaks based on the `long_break_counter` variable. The code now allows for a specified number of work intervals before triggering a long break. This change improves the flexibility and customization of the interval timer functionality.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 24 commits contributed to the release over the course of 5 calendar days.
 - 20 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update email addresses in code of conduct and security files ([`12790a0`](https://github.com/miguoliang/tomato-clock-cli/commit/12790a00cb1dc328837701f1e8ace4143a96273b))
    - Update CONTRIBUTING.md with a more detailed contributing guide ([`d302a0c`](https://github.com/miguoliang/tomato-clock-cli/commit/d302a0c372677d834b5f3398aa14311f78e4518e))
    - Update markdownlint configuration to disable specific rules ([`6440290`](https://github.com/miguoliang/tomato-clock-cli/commit/64402904c24b7d401c19722075f8d33aa38fef46))
    - Update SonarCloud configuration to include source code in 'src' directory ([`88487a2`](https://github.com/miguoliang/tomato-clock-cli/commit/88487a27a237562a97fd25eecbcc7b017bf7c976))
    - Add SonarCloud configuration files for code analysis ([`14d9d8d`](https://github.com/miguoliang/tomato-clock-cli/commit/14d9d8d987908dd637b9ebf54501e8ad60a80e6b))
    - Add SonarCloud workflow for code analysis ([`795cd50`](https://github.com/miguoliang/tomato-clock-cli/commit/795cd5090cb513e7528178141ae6d99515ee2914))
    - Update Cargo.toml, introduction page, and Hugo configuration for Tomato Clock CLI integration ([`20f9cb6`](https://github.com/miguoliang/tomato-clock-cli/commit/20f9cb6a4b3f72ebfb74d8aa4baad6723a8a3d01))
    - Update Cargo.toml and introduction page with Tomato Timer CLI details and features ([`c1da290`](https://github.com/miguoliang/tomato-clock-cli/commit/c1da29015d671b7af3e35db17fb6eef7db2899e4))
    - Update introduction page with Tomato Timer CLI details and features ([`7bebfe8`](https://github.com/miguoliang/tomato-clock-cli/commit/7bebfe8b62e789afe0a1bfec5d5cf0f0515db88e))
    - Remove outdated Hugo GitHub Actions workflow ([`444e7e0`](https://github.com/miguoliang/tomato-clock-cli/commit/444e7e0badf8a285c843c092e72bc85796ee5de9))
    - Create hugo.yml ([`ae26652`](https://github.com/miguoliang/tomato-clock-cli/commit/ae26652956c285802d4fed5f39a333a982512c83))
    - Update default branch name in GitHub Actions workflow ([`4905667`](https://github.com/miguoliang/tomato-clock-cli/commit/490566706326c8b44d291b0191f9cc27ab660ef5))
    - Update default branch name in GitHub Actions workflow ([`f1b9f87`](https://github.com/miguoliang/tomato-clock-cli/commit/f1b9f8769b96b8929144babe1c6f6281aafe537f))
    - Add GitHub Actions workflow for building and deploying Hugo website ([`bb7c194`](https://github.com/miguoliang/tomato-clock-cli/commit/bb7c19417d3df0f79b02f541e5afdaa6d89e8b79))
    - Add Hugo theme submodule and configuration files ([`b4a2d35`](https://github.com/miguoliang/tomato-clock-cli/commit/b4a2d35169c19671ee0cc166a1579e47a87ed2e7))
    - Add CONTRIBUTING.md file ([`70175eb`](https://github.com/miguoliang/tomato-clock-cli/commit/70175ebb16b2c4946c57ebf7d45c5b31055c8729))
    - Add security policy documentation ([`794b249`](https://github.com/miguoliang/tomato-clock-cli/commit/794b24974cb867b2d9dfbea22060be846f832bde))
    - Create CODE_OF_CONDUCT.md ([`dc503ff`](https://github.com/miguoliang/tomato-clock-cli/commit/dc503ff3153f753898e7d1b972ffc352826f5d8c))
    - Create LICENSE ([`a118209`](https://github.com/miguoliang/tomato-clock-cli/commit/a118209519fbb3c764b4c7c19e60e3e8b0a0c8c6))
    - Add Gitpod configuration files ([`46744ba`](https://github.com/miguoliang/tomato-clock-cli/commit/46744bac17a607fdb74030a4795c76649ed5f404))
    - Refactor interval timer code to support customizable break durations ([`3c9b9d3`](https://github.com/miguoliang/tomato-clock-cli/commit/3c9b9d33a912916c0cc8e35b7744358b29168569))
    - Add support for long breaks and intervals ([`e282ddf`](https://github.com/miguoliang/tomato-clock-cli/commit/e282ddfdd139d2fe19510fae41562bb3df6a7b53))
    - Add support for long breaks and intervals ([`1844ee8`](https://github.com/miguoliang/tomato-clock-cli/commit/1844ee8396ac65fe6de58ecfe924219182dddc47))
    - Init commit ([`48d801d`](https://github.com/miguoliang/tomato-clock-cli/commit/48d801dc256a029fc193f19684b59561ba5c5982))
</details>

