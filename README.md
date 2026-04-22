# tasklog

A tiny terminal task tracker that stays out of your way.

`tasklog` is a single-binary Rust CLI for jotting down todos, marking them done,
and grepping through what you've already finished — without spinning up a full
project-management app for what is, fundamentally, a list of strings.

## Why

I wanted a todo tool that:

- Lives entirely in the terminal — no daemons, no sync servers, no accounts.
- Uses a plain JSON file under `$XDG_DATA_HOME` so I can grep, diff, or back it
  up the same way I do anything else.
- Starts in under 50 ms cold so I'll actually use it as muscle memory rather
  than treating it like a chore.

## Install

```sh
cargo install tasklog
```

Or build from source:

```sh
git clone https://github.com/The3eard/beardgit_gh_tests
cd beardgit_gh_tests
cargo build --release
./target/release/tasklog --help
```

## Usage

```sh
tasklog add "Write the changelog for v0.3.0"
tasklog add "Triage parser issue" --tag bug --due 2026-05-01
tasklog list                       # open tasks only
tasklog list --all                 # include completed
tasklog list --tag bug             # filter by tag
tasklog done 3                     # mark task #3 as done
tasklog rm 7                       # delete task #7
tasklog search "changelog"         # full-text search across title + tags
```

Tasks are stored at:

| Platform | Path                                                       |
| -------- | ---------------------------------------------------------- |
| Linux    | `~/.local/share/tasklog/tasks.json`                        |
| macOS    | `~/Library/Application Support/tasklog/tasks.json`         |
| Windows  | `%APPDATA%\tasklog\tasks.json`                             |

## Status

Stable enough for daily use; see [CHANGELOG.md](CHANGELOG.md) for what landed
when. The TUI dashboard (`tasklog tui`) is in active development on
`feat/tui-dashboard`.

## Contributing

PRs welcome. Please run `cargo fmt && cargo clippy --all-targets -- -D warnings`
before pushing — CI will reject otherwise.

## License

MIT — see [LICENSE](LICENSE).
