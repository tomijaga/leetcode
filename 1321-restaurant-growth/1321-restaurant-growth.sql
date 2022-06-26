
select 
    visited_on, 
    (select sum(amount)
     from customer 
     where to_days(c.visited_on) - to_days(visited_on) between 0 and 6  ) as amount, 
    (select round(sum(amount)/7, 2) 
     from customer 
     where to_days(c.visited_on) - to_days(visited_on) between 0 and 6   ) as average_amount
from customer c
where to_days(visited_on) - 6 in (select to_days(visited_on) from customer)
group by visited_on
order by visited_on