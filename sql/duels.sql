select
    d.id
    , d.game_id
    , d.team_id
    , t.name as team_name
    , d.team_id = s.home_team_id as home
    , d.secondary_type
    , d.penalty_severity
    , d.penalty_minutes
    , d.player_id as player_id_drew_by
    , pd.full_name as player_name_drew_by
    , pd.position as player_position_drew_by
    , p.player_id as player_id_penalty_on
    , pp.full_name as player_name_penalty_on
    , pp.position as player_position_penalty_on
from
    events d
inner join
    events p
    on d.id = p.id
    and d.player_type = 'DrewBy'
    and p.player_type = 'PenaltyOn'
    and d.game_id = p.game_id
inner join
    teams t
    on t.id = d.team_id
inner join
    schedule s
    on s.id = d.game_id
inner join
    players pd
    on pd.id = d.player_id
    and pd.game_id = d.game_id
inner join
    players pp
    on pp.id = p.player_id
    and pp.game_id = p.game_id
;
