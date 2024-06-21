# Changelog

All notable changes to the `tomato-clock-cli` project will be documented in this file.

## [0.1.0] - 2024-06-13

### Added

- Start and stop pomodoro sessions
- Set custom durations for pomodoros and breaks
- Track your progress with a visual timer
- Receive notifications when a pomodoro or break ends

[0.1.0]: https://github.com/olivierlacan/keep-a-changelog/releases/tag/v0.1.0

## v0.1.0 (2024-06-21)

<csr-id-12790a00cb1dc328837701f1e8ace4143a96273b/>
<csr-id-d302a0c372677d834b5f3398aa14311f78e4518e/>
<csr-id-64402904c24b7d401c19722075f8d33aa38fef46/>
<csr-id-88487a27a237562a97fd25eecbcc7b017bf7c976/>
<csr-id-14d9d8d987908dd637b9ebf54501e8ad60a80e6b/>
<csr-id-795cd5090cb513e7528178141ae6d99515ee2914/>
<csr-id-20f9cb6a4b3f72ebfb74d8aa4baad6723a8a3d01/>
<csr-id-c1da29015d671b7af3e35db17fb6eef7db2899e4/>
<csr-id-7bebfe8b62e789afe0a1bfec5d5cf0f0515db88e/>
<csr-id-444e7e0badf8a285c843c092e72bc85796ee5de9/>
<csr-id-490566706326c8b44d291b0191f9cc27ab660ef5/>
<csr-id-f1b9f8769b96b8929144babe1c6f6281aafe537f/>
<csr-id-bb7c19417d3df0f79b02f541e5afdaa6d89e8b79/>
<csr-id-b4a2d35169c19671ee0cc166a1579e47a87ed2e7/>
<csr-id-70175ebb16b2c4946c57ebf7d45c5b31055c8729/>
<csr-id-794b24974cb867b2d9dfbea22060be846f832bde/>
<csr-id-46744bac17a607fdb74030a4795c76649ed5f404/>
<csr-id-3c9b9d33a912916c0cc8e35b7744358b29168569/>
<csr-id-e282ddfdd139d2fe19510fae41562bb3df6a7b53/>
<csr-id-1844ee8396ac65fe6de58ecfe924219182dddc47/>

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

 - <csr-id-b8b542c2bb82b42626031e486d005d5a7bc40694/> Make app run under the terminal's raw mode to support more complex features in the future.

### Chore

 - <csr-id-9c2edc6e6c1900b335fecd74d56419fe082ffe11/> Update timer interval and add pause functionality
 - <csr-id-a2c40622325920999808fbbda4dfbd86c6586522/> Update logo URL in README.md
 - <csr-id-da3c34eace37aa89e650d28bad56613d4037fb23/> Update repository and homepage URLs in Cargo.toml

### Refactor

- <csr-id-e282ddfdd139d2fe19510fae41562bb3df6a7b53/> Add support for long breaks and intervals
- <csr-id-1844ee8396ac65fe6de58ecfe924219182dddc47/> Add support for long breaks and intervals
   This commit refactors the code to add support for long breaks and intervals. It introduces a new struct `Interval` with properties such as name, color, duration, and message_done. The `execute_interval` function is modified to handle both short breaks and long breaks based on the `long_break_counter` variable. The code now allows for a specified number of work intervals before triggering a long break. This change improves the flexibility and customization of the interval timer functionality.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update timer interval and add pause functionality ([`9c2edc6`](https://github.com/miguoliang/tomato-timer-cli/commit/9c2edc6e6c1900b335fecd74d56419fe082ffe11))
    - Update logo URL in README.md ([`a2c4062`](https://github.com/miguoliang/tomato-timer-cli/commit/a2c40622325920999808fbbda4dfbd86c6586522))
    - Update repository and homepage URLs in Cargo.toml ([`da3c34e`](https://github.com/miguoliang/tomato-timer-cli/commit/da3c34eace37aa89e650d28bad56613d4037fb23))
    - Make app run under the terminal's raw mode to support more complex features in the future. ([`b8b542c`](https://github.com/miguoliang/tomato-timer-cli/commit/b8b542c2bb82b42626031e486d005d5a7bc40694))
</details>

