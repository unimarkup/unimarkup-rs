# Contributing to unimarkup-rs

Thank you for considering contributing to `unimarkup-rs`, it means a lot to us!\
Below are the most important topics to get you started.

- [Discussions](#discussions)
- [Issue/PR Labels](#issuepr-labels)
- [Development Setup](#development-setup)
- [Commit Message Convention](#commit-message-convention)

## Discussions

We use [GitHub Discussions](https://github.com/unimarkup/unimarkup-rs/discussions) for exchanges with the community.
It is a good place to start if you have any questions.

## Issue/PR Labels

There are two labels to help possible contributors to get involved:

- [good-first-issue](https://github.com/unimarkup/unimarkup-rs/labels/good-first-issue) ... This label is used to mark issues that should be easy to implement **without** extensive understanding of the project
- [help-needed](https://github.com/unimarkup/unimarkup-rs/labels/help-needed) ... This label is used to mark issues, where project members need help to resolve it

For better asynchronous communication, we use the following labels:

- [waiting-on-assignee](https://github.com/unimarkup/unimarkup-rs/labels/waiting-on-assignee) ... This label is used to indicate that the author or reviewer is awaiting response from the assignee
- [waiting-on-author](https://github.com/unimarkup/unimarkup-rs/labels/waiting-on-author) ... This label is used to indicate that the assignee or reviewer is awaiting response from the author
- [waiting-on-reviewer](https://github.com/unimarkup/unimarkup-rs/labels/waiting-on-reviewer) ... This label is used to indicate that the assignee or author is awaiting response from the reviewer

To keep track of feature requests, we use the following labels:

- [declined](https://github.com/unimarkup/unimarkup-rs/labels/declined) ... This label is used to mark issues/PRs that they won't be considered/implemented further
- [req-ready](https://github.com/unimarkup/unimarkup-rs/labels/req-ready) ... This label is used to mark `[REQ]` issues that they have enough information to be implemented

## Development Setup

We use the following tools for development:

1. [rustup](https://rustup.rs/) to get a Rust compiler.
2. [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer) as a Language Server to be used with LSP.
3. `clippy` as linter. i.e. for VS Code this setting is in the `rust-analyzer` extension settings (Change `cargo check` command to `clippy`)
4. Optional: Enable auto-formatting of the code, by using `rustfmt` default settings.

Add-ons for testing:

- [insta](https://github.com/mitsuhiko/insta) for snapshot testing
- [nextest](https://github.com/nextest-rs/nextest) for better formatted test output

## Commit Message Convention

We have our own convention for git commit messages that are inspired by [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/).\
This leads to more **readable** messages that are easy to follow when looking through the [project's history](https://github.com/unimarkup/unimarkup-rs/commits/main).
In addition, we use git commit messages to automatically get a **change log** using [release-please](https://github.com/googleapis/release-please) from Google.
As a result, commit messages not following our convention won't appear in the generated change log.

**Note:** We use `squash merging` for pull requests, so we are able to correct your commit messages, but please try to stick to the convention to make merging easier for us.

### Commit Message Format

Each commit message consists of a **header**, a **body**, and a **footer**.\
The header itself consists of a **type** and a **subject**:

~~~
<type>: <subject>
<BLANK LINE>
<body>
<BLANK LINE>
<footer>
~~~

**Note:** Any line of the commit message must not be longer than 72 characters, and the header should not be longer than 50 characters.
This makes messages more readable on GitHub and in various git tools.

### Type

The commit type must be one of the following:

- **feat** ... Added a new feature
- **fix** ... Fixed a bug of any kind
- **arch** ... Neither adds features nor fixes bugs (e.g. renaming or restructuring)
- **chore** ... Made other changes (should only be used for automatically generated commits)

### Subject

The subject should contain a succinct description of the change:

- use the imperative, present tense: "change" not "changed" nor "changes"
- don't capitalize the first letter
- do not add a dot `.` at the end

**Note:** It helps to write the subject in a way that continues the phrase `This commit will ...`.

### Breaking Changes

To mark a commit as breaking change, add an exclamation mark after the commit type, i.e. `feat!: introduce some new feature`.
Another option would be to start the footer with `BREAKING CHANGE: `, but prefer to use the exclamation mark as it is directly visible in the [project's history](https://github.com/unimarkup/unimarkup-rs/commits/main).

### Body

The body should include the motivation for the change, and contrast this with previous behavior.

**Note:** Just as in the [subject](#subject), use the imperative, present tense: "change" not "changed" nor "changes".

### Footer

The footer may contain information about **Breaking Changes**, or reference GitHub issues that this commit **Closes**.

**Note:** The footer should be used rarely, because breaking changes should be marked by an exclamation mark, and issues should be closed by PRs.

### Examples

~~~
feat: add support to parse headings
~~~

~~~
fix: fix parsing of headings

Heading levels were parsed incorrectly with an off-by-one error.
~~~

### Hooks

We provide our own git hooks in the [.hooks](.hooks/) directory to help write commit messages according to our convention.

To use our `commit-msg` git hook, copy the [commit-msg file](.hooks/commit-msg) into your `.git/hooks/` directory.
Alternatively set our [.hooks](.hooks/) directory as your global git hooks directory with the following shell command: 
`git config core.hooksPath ./.hooks`

## Pull Requests

Create a pull request (PR) if you want to integrate your changes.
In most cases you want to set `main` as the target branch.

The title of the PR must be written like a commit message.
The section [commit message convention](#commit-message-convention) above describes how these messages should be written.
