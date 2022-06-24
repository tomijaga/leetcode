select u.name as name, if(t.distance is not null, t.distance, 0) as travelled_distance
from users u
left join 
    (select user_id as id, sum(distance) as distance
        from rides
        group by user_id) t
on u.id = t.id
order by travelled_distance desc, name asc
