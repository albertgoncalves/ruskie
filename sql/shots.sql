with shots as (
    select
        e.game_id
        , s.team_id
        , c.type
        , c.season
        , e.period
        , e.period_time
        , e.x
        , e.y
        , e.player_id
        , e.event = 'Goal' as goal
        , (s.team_id = e.team_id) as event_for
        , (p.team_id = c.home_team_id) as home
        , max((p.position_abbreviation = 'G')) as goalie
        , count(distinct s.player_id) as skaters
    from
        shifts s
    inner join
        events e
        on s.game_id = e.game_id
        and s.period = e.period
        and e.period_time > s.start_time
        and e.period_time <= s.end_time
        and s.period <= 3
        and s.event = ''
        and e.event in ('Goal', 'Missed Shot', 'Shot')
        and e.player_type in ('Scorer', 'Shooter')
    inner join
        players p
        on p.id = s.player_id
        and p.game_id = s.game_id
    inner join
        schedule c
        on c.id = e.game_id
    group by
        e.game_id
        , s.team_id
        , e.period
        , e.period_time
        , e.x
        , e.y
        , e.player_id
        , e.event = 'Goal'
        , (s.team_id = e.team_id)
        , (p.team_id = c.home_team_id)
    order by
        e.game_id
        , e.period
        , e.period_time
        , (p.team_id = c.home_team_id)
)

, penalty_shots as (
    select
        game_id
        , period
        , period_time
        , true as penalty
    from
        events
    where
        penalty_severity = 'Penalty Shot'
    group by
        game_id
        , period
        , period_time
)

, for_against as (
    select
        f.game_id
        , f.type
        , f.season
        , f.period
        , f.period_time
        , f.x
        , f.y
        , f.goal
        , coalesce(s.penalty, false) as penalty
        , f.team_id as team_for
        , a.team_id as team_against
        , f.player_id
        , p.full_name
        , p.position_abbreviation
        , p.shoots_catches
        , f.home
        , f.skaters as skaters_for
        , f.goalie as goalie_for
        , a.skaters as skaters_against
        , a.goalie as goalie_against
    from
        shots f
    inner join
        shots a
        on f.game_id = a.game_id
        and f.period = a.period
        and f.period_time = a.period_time
        and f.x = a.x
        and f.y = a.y
        and f.event_for
        and not a.event_for
    inner join
        players p
        on p.id = f.player_id
        and p.game_id = f.game_id
    left join
        penalty_shots s
        on f.game_id = s.game_id
        and f.period = s.period
        and f.period_time = s.period_time
)

select
    *
from
    for_against
where
    type = 'R'
    and skaters_for = 6
    and goalie_for = 1
    and skaters_against = 6
    and goalie_against = 1
    and penalty
order by
    period
    , period_time
    , team_for
;
