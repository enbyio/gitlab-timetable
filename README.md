# Timetable creator from gitlab issues

This project is designed to automate pulling all gitlab issues for a specific gitlab project / group and tabulate them by person and time spent into an asciidoc file Table.

## Config:
- Add your correct gitlab url project or group url and token in the config.toml. 
- Add the correct milestone/s (can also be left empty for all tickets)
- Select additional parameters (only count closed tickets for example)

## Usage:
- if the config.toml is inside the running directory just use cargo run
- otherwise run with --config "path-to-file"