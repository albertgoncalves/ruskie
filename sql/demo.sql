select
    g.id
    , g.date
    , g.status_abstract
    , ht.name as home_team
    , ht.venue_name = g.venue_name as home
    , at.name as away_team
    , at.venue_name = g.venue_name as home
    , g.venue_name
    , ht.venue_name as home_venue_name
    , at.venue_name as away_venue_name
from
    games g
inner join
    teams ht
    on ht.id = g.home_team_id
inner join
    teams at
    on at.id = g.away_team_id
where
    g.type = 'R'
    and (ht.venue_name = g.venue_name) = (at.venue_name = g.venue_name)
order by
    g.venue_name
    , ht.venue_name
    , at.venue_name
;
