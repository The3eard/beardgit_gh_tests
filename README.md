# tasklog

A tiny terminal task tracker that stays out of your way.

## Usage

```sh
tasklog add "Write the changelog"
tasklog list                       # open tasks only
tasklog list --all                 # include completed
tasklog done 3                     # mark task #3 as done
tasklog rm 7                       # delete task #7
```

Tasks are stored in `~/.tasklog.json`.

## Status

Pre-release. Stable enough to dogfood, but the storage path will move to
platform-correct directories before the first stable release.

## License

MIT — see [LICENSE](LICENSE).
