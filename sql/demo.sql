select
    s.id
    , s.date
    , ht.name as home_team
    , s.home_team_score
    , ht.venue_name = s.venue_name as home
    , at.name as away_team
    , s.away_team_score
    , at.venue_name = s.venue_name as home
    , s.venue_name
    , ht.venue_name
    , at.venue_name
from
    schedules s
inner join
    teams ht
    on ht.id = s.home_team_id
inner join
    teams at
    on at.id = s.away_team_id
where
    s.type = 'R'
    and (ht.venue_name = s.venue_name) = (at.venue_name = s.venue_name)
order by
    s.venue_name
    , ht.venue_name
    , at.venue_name
;
