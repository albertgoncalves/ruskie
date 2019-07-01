#!/usr/bin/env bash

set -eu

x="$WD/data/teams.json"

if [ ! -f $x ]; then
    curl "https://statsapi.web.nhl.com/api/v1/teams" > $x
fi
