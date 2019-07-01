#!/usr/bin/env bash

set -eu

cat "$WD/data/teams.json" \
    | jq -c '
        .teams[] | {
            team_name: .name,
            team_id: .id,
            abbreviation: .abbreviation,
            venue_name: .venue.name,
            venue_id: .venue.id,
        }
    '
