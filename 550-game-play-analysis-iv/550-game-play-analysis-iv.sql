with player as (select player_id, min(event_date) as event_date
    from activity
    group by player_id)
select distinct round(count(*) / (select count(*) from player), 2) as fraction
from activity
where (player_id, to_days(event_date) -1 ) in (select player_id, to_days(event_date) from player )
