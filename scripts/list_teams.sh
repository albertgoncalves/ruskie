#!/usr/bin/env bash

set -eu

cat $WD/data/teams-2018-08-01-2019-08-01.json \
    | jq -c '
        .teams[] |
            { team_name: .name
            , team_id: .id
            , abbreviation: .abbreviation
            , venue_name: .venue.name
            , venue_id: .venue.id
            }
    '
