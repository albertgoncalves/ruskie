#!/usr/bin/env bash

set -eu

start="2018-08-01"
end="2019-08-01"

cat "$WD/data/teams-$start-$end.json" \
    | jq -c '
        .teams[] | {
            team_name: .name,
            team_id: .id,
            abbreviation: .abbreviation,
            venue_name: .venue.name,
            venue_id: .venue.id,
        }
    '
