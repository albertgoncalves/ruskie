#!/usr/bin/env bash

set -eu

echo $WD/data

start="2018-08-01"
end="2019-08-01"

curl "https://statsapi.web.nhl.com/api/v1/teams" \
    > "$WD/data/teams-$start-$end.json"

team_id="54"

curl "https://statsapi.web.nhl.com/api/v1/schedule?teamId=$team_id&startDate=$start&endDate=$end" \
    > "$WD/data/schedule-$team_id-$start-$end.json"

game_id="2018020861"

curl "http://www.nhl.com/stats/rest/shiftcharts?cayenneExp=gameId=$game_id" \
    > "$WD/data/shifts-$game_id.json"
curl "https://statsapi.web.nhl.com/api/v1/game/$game_id/feed/live?site=en_nhl" \
    > "$WD/data/game-$game_id.json"
