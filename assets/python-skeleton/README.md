# Python Skeleton Repo
This is an *exploratory* skeleton repo for Python projects using both Poetry and either/both CircleCI or GitHub_Actions.
the `pyproject.toml` is pre-loaded with dev dependenceis used in the supplied pre-commit hook and CircleCI tests.


## CircleCI Badge
Add CircleCI Passing/Failing Badge: [link to docs](https://circleci.com/docs/status-badges/?utm_source=google&utm_medium=sem&utm_campaign=sem-google-dg--uscan-en-dsa-maxConv-auth-brand&utm_term=g_-_c__dsa_&utm_content=&gclid=Cj0KCQiAz9ieBhCIARIsACB0oGLRozHy2fiAiThYNATH7_Nw_i_2fv1oTzfkBexHpv7gn9zhKzPm_KYaAt-EEALw_wcB)   
(You will also likely find an easy insertion in the CircleCI section related to the repo.)

```markdown
[![<ORG_NAME>](https://circleci.com/<VCS>/<ORG_NAME>/<PROJECT_NAME>.svg?style=svg)](<LINK>)
```

## GitHub Actions Badge
Add a GitHub Actions Passing/Failing Badge: [link to docs](https://docs.github.com/en/actions/monitoring-and-troubleshooting-workflows/adding-a-workflow-status-badge)
```markdown
![hidden words](https://github.com/<OWNER>/<REPOSITORY>/actions/workflows/<WORKFLOW_FILE>/badge.svg)
```

## Dev-Dependencies Specified
- formatting: `isort` & `black`
- linting: `pylint`
- lsp & typechecking: `pyright`
- testing: `pytest` + `coverage` (via `pytest-cov`)
- auto-documentation: `pdoc` (*not* ~~"pdoc3"~~, which should be strongly avoided)


## Run Pre-Commit Hook Manually
from anywhere in project:
```zsh
git hook run pre-commit
```

**Note**: the `pre-commit` file needs to be placed in the `.git/hooks/` directory.
