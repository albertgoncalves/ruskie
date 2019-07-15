select
    id
    , group_concat(distinct full_name) full_name
    , count(distinct full_name) n
from
    players
group by
    id
;
