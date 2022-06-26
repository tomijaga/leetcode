select 
    request_at as day,
    round(avg(if(status like "cancelled_by_driver", 
           if(d.banned like "Yes", 0, 1),
           if(status like "cancelled_by_client",
                if(c.banned like "Yes", 0, 1),
                0))), 2) as "Cancellation Rate"
from trips t
inner join users c
on t.client_id = c.users_id and c.banned = "No"
inner join users d
on t.driver_id = d.users_id and d.banned = "No"
where request_at between "2013-10-01" and "2013-10-03"
group by request_at