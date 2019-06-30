select
    s.id
    , s.date
    , s.status_abstract
    , ht.name as home_team
    , ht.venue_name = s.venue_name as home
    , at.name as away_team
    , at.venue_name = s.venue_name as home
    , s.venue_name
    , ht.venue_name as home_venue_name
    , at.venue_name as away_venue_name
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
