# Project Repository Standards

## 📜 Table of Contents
- [Commit Standards](#commit-standards)

---

## 🫱🏽‍🫲🏿 Commit Standards
<a id="commit-standards"></a>

### Structure of a Commit Message
```
git commit -m "<tag_type>(scope): <message>"
```

Here are some examples:
- `feat: User login form`
- `fix: Logo no longer disappears on smaller screen sizes`
- `style(app): eslint fixes`
- `docs: update README`

### General Rules
1. Try not to mix tag types. Make sure that everything in your commit makes sense as a single bundle and actually NEEDS to be included.
    - For example, if your commit has both style updates and bug fixes, try to break them into separate commits.
2. Use the `scope` section to refer to specific project components (e.g. `api`, `app`)
3. Please make the `message` section short and as descriptive as possible.

### Tag Types
The following words should be used to tag commits. They can be case-insensitive. This is by no means an exhaustive list so if you find that another tag word fits your commit better, please feel free to use it.

- **build**: Changes that affect how the project is built or change its dependencies
- **chore**: Misc. changes; changes that affect the developer experience (code formatting configs, IDE configs, etc.)
- **feat**: A new feature or completion of a new feature. May be used in place of `update`
- **update**: Any work that leads to completion of a task, finding a solution to a bug, etc. May be used in place of `feat`
- **docs**: Updates to documentation, such as comments, Markdown files, etc.
- **fix**: Bug fixes
- **refactor**: A code change that does not change how the code works
- **perf**: Changes that improve the efficiency or performance of the code
- **style**: Code style changes, such as white-space, formatting, linting fixes, etc.
- **test**: Addition of automated tests or updating existing tests
