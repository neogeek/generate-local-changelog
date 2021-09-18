# generate-local-changelog

> Generate a CHANGELOG for your project using only local git history. No internet connection or git server API is required.

[![Tests](https://github.com/neogeek/generate-local-changelog/actions/workflows/test.workflow.yml/badge.svg)](https://github.com/neogeek/generate-local-changelog/actions/workflows/test.workflow.yml)

## Install

```bash
$ brew tap neogeek/generate-local-changelog
$ brew install generate-local-changelog
```

## Usage

```bash
$ generate-local-changelog > CHANGELOG.md
```

```bash
$ generate-local-changelog --unreleased > CHANGELOG.md
```

```bash
$ generate-local-changelog --include-incomplete-merges > CHANGELOG.md
```

## FAQs

### What are incomplete merges?

Typically a PR merged from GitHub will look like this:

```
* c4ca423 (2 minutes ago) Merge pull request #1 from username/repo
|\
| * 8a0b923 (10 minutes ago)
| * 820dcc5 (15 minutes ago)
|/
* 832627b (1 hour ago)
```

But on occassion a PR can appear like this:

```
* c4ca423 (2 minutes ago) [feat] PR subject line (#1)
* 8a0b923 (10 minutes ago)
* 820dcc5 (15 minutes ago)
* 832627b (1 hour ago)
```

Git will not treat these as valid merges and therefore they will be missing from the generated CHANGELOG. To include them, add `--include-incomplete-merges` flag. This will of course be different from repo to repo. Only use this flag if you notice PRs missing from your CHANGELOG.
