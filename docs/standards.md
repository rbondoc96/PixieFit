# Project Repository Standards

## 📜 Table of Contents
- [Commit Standards](#commit-standards)

---

## 🫱🏽‍🫲🏿 Commit Standards
<a id="commit-standards"></a>

### Structure of a Commit Message
```
git commit -m "<issue_no | package | "ROOT"> <tag_type> <message>"
```

Here are some examples:
- `CLIENT feat: User login form`
- `#31 fix: Logo no longer disappears on smaller screen sizes`
- `SERVER style: eslint fixes`
- `ROOT docs: update README`

### General Rules
1. Do not mix tag types. Make sure that everything everything in your commit makes sense as a single bundle and actually NEEDS to be included.
    - For example, if your commit has both style updates and bug fixes, break them into separate commits.
2. If the commit references a specific issue, please include an issue number if applicable. Otherwise, please specify the package that the commit targets.
    - A commit should **NOT** apply to more than a single package.
    - For commits that target the project root or do not apply to ANY packages, please use the `ROOT` (case-sensitive) specifier
3. Please make the `message` section short and as descriptive as possible.

### Tag Types
The following words should be used to tag commits. They can be case-insensitive. This is by no means an exhaustive list so if you find that another tag word fits your commit better, please feel free to use it.

- **build**: Changes that affect how the app is built or change its dependencies
- **dev**: Changes to files that affect the developer experience (such as `.vscode/settings.json`)
- **feat**: A new feature, ideally for larger commits
- **docs**: Updates to documentation, such as comments, Markdown files, etc.
- **fix**: Bug fixes
- **refactor**: A code change that does not change how the code works
- **style**: Code style changes, such as white-space, formatting, linting fixes, etc.
- **test**: Addition of automated tests or updating existing tests
- **update**: A new feature or removal of code. Can be used in place of `feat`.
