#!/usr/bin/env bash

set -eu

echo $WD/data
echo $START
echo $END

f () {
    if [ ! -f $2 ]; then
        curl $1 > $2
    fi
}

f "https://statsapi.web.nhl.com/api/v1/teams" \
    "$WD/data/teams/$START-$END.json"

team_id="54"

f "https://statsapi.web.nhl.com/api/v1/schedule?teamId=$team_id&startDate=$START&endDate=$END" \
    "$WD/data/schedule/$team_id-$START-$END.json"

game_id="2018020861"

f "http://www.nhl.com/stats/rest/shiftcharts?cayenneExp=gameId=$game_id" \
    "$WD/data/shifts/$game_id.json"
f "https://statsapi.web.nhl.com/api/v1/game/$game_id/feed/live?site=en_nhl" \
    "$WD/data/game/$game_id.json"
