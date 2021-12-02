#!/usr/bin/env bash

set -euo pipefail

usage() {
  cat << EOF
Usage: $(basename "${0}")

Sets up the next Advent of Code day directory.

EOF
  exit 0
}

[[ $# != 0 ]] && usage

rust_edition=2021
aoc_year=2021
total_days=25

##
## Return the name of the directory for the specified day.
##
day_dir() {
  local day_num="${1}"
  printf "day%02d" "${day_num}"
}

##
## Create the directory and initial contents for the specified day.
##
create_day() {
  local day_num="${1}"
  local day_dir
  day_dir="$(day_dir "${day_num}")"

  # Create a new Cargo bin directory.
  cargo new \
    --bin "${day_dir}" \
    --edition "${rust_edition}" \
    --vcs none

  # Create a launch.json file for configuring dap-mode.
  cat << EOF > "${day_dir}/launch.json"
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "lldb-default",
            "type": "lldb-vscode",
            "request": "launch",
            "program": "\${workspaceFolder}/target/debug/${day_dir}",
            "args": [],
	    "env": {},
            "cwd": "\${workspaceFolder}",
            "stopOnEntry": false,
	    "debuggerRoot": "\${workspaceFolder}"
        }
     ]
}
EOF

  # Create an empty input/input.txt file for the day.
  local input_dir="${day_dir}/input"
  local input_file="${input_dir}/input.txt"
  mkdir "${input_dir}"
  touch "${input_file}"

  # Remind to copy the day's input from the website. Need to open the website in a
  # browser since it requires you to be logged in and each user's input is different.
  echo >&2 "Save the day's input into ${day_dir}/input/input.txt"
  open "https://adventofcode.com/${aoc_year}/day/${day_num}/input"
}

# Create the next day.
for day_num in $(seq 1 "${total_days}"); do
  if [[ ! -e "$(day_dir "${day_num}")" ]]; then
    create_day "${day_num}"
    exit 0
  fi
done

echo >&2 "You've already created the last day!"
