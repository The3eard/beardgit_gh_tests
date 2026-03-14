# tasklog

A tiny terminal task tracker that stays out of your way.

## Usage

```sh
tasklog add "Write the changelog"
tasklog add "Triage parser issue" --tag bug --due 2026-05-01
tasklog list                       # open tasks only
tasklog list --all                 # include completed
tasklog done 3                     # mark task #3 as done
tasklog rm 7                       # delete task #7
```

Tasks are stored in `~/.tasklog.json`.

## License

MIT — see [LICENSE](LICENSE).
