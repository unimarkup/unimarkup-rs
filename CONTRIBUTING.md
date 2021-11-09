# Contributing to unimarkup-rs

We would love for you to contribute to unimarkup-rs and help make it better!
As a contributor, here are the guidelines we would like you to follow:

- [Commit Message Guidelines](#commit)

## <a name="commit"></a> Commit Message Guidelines

We have very precise rules over how our git commit messages can be formatted.  This leads to **more
readable messages** that are easy to follow when looking through the **project history**.  But also,
we use the git commit messages to **generate the unimarkup-rs change log**.

### Commit Message Format
Each commit message consists of a **header**, a **body** and a **footer**.  The header has a special
format that includes a **type**, a **scope** and a **subject**:

```
<type>(<scope>): <subject>
<BLANK LINE>
<body>
<BLANK LINE>
<footer>
```

The **header** is mandatory and the **scope** of the header is optional.

Any line of the commit message cannot be longer 100 characters! The header should not be longer than 50 characters!
This allows the message to be easier to read on GitHub as well as in various git tools.

The footer should contain a [closing reference to an issue](https://help.github.com/articles/closing-issues-via-commit-messages/) if any.

Samples: (even more [samples](https://github.com/Unimarkup/unimarkup-rs/commits/main))

```
docs(changelog): update changelog to beta.5
```
```
fix(release): fix parsing of headings

Heading levels were parsed incorrectly, with an off-by-one error.
```

### Revert
If the commit reverts a previous commit, it should begin with `revert: `, followed by the header of the reverted commit. 
In the body it should say: `This reverts commit <hash>.`, where the hash is the SHA of the commit being reverted.

### Type
Must be one of the following:

* **fix**: Fix a bug
* **feat**: Add a new feature
* **ci**: Changes to CI configuration files and scripts
* **chore**: Miscellaneous (should only be used for automatically generated commits)
* **docs**: Documentation only changes
* **style**: Changes that do not affect the semantic of the code
* **refactor**: A code change that neither fixes a bug nor adds a feature
* **perf**: A code change that improves performance
* **test**: Adding tests or correcting existing tests
* **build**: Changes that affect the build system or external dependencies (example scopes: cargo, rustc...)

### Scope
The scope should describe the part of unimarkup-rs affected (as perceived by the person reading the changelog generated from commit messages.)

The following is the list of supported scopes:

* **parser**
* **logger**
* **frontend**
* **middleend**
* **backend**
* **renderer**
* **sql**
* **database**

### Subject
The subject contains a succinct description of the change:

* use the imperative, present tense: "change" not "changed" nor "changes"
* don't capitalize the first letter
* no dot (.) at the end

### Body
Just as in the **subject**, use the imperative, present tense: "change" not "changed" nor "changes".
The body should include the motivation for the change and contrast this with previous behavior.

### Footer
The footer should contain any information about **Breaking Changes** and is also the place to
reference GitHub issues that this commit **Closes**.

**Breaking Changes** should start with the word `BREAKING CHANGE:` with a space or two newlines. The rest of the commit message is then used for this.