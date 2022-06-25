# Write your MySQL query statement below
select 
    from_id as person1,
    to_id as person2,
    count(*) as call_count,
    sum(duration)  total_duration
from
    (select 
        if (from_id > to_id, to_id, from_id) as from_id ,
        if (from_id > to_id, from_id, to_id) as to_id,
        duration
    from calls) t
group by from_id, to_id

