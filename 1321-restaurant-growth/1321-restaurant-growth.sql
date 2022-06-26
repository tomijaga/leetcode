with const as 
    (select 
        visited_on, 
        sum(amount) as amount 
     from customer 
     group by visited_on)
select 
    b.visited_on, 
    sum(a.amount) as amount, 
    round(avg(a.amount), 2) as average_amount
from 
    const a,
    const b
where to_days(b.visited_on) - to_days(a.visited_on) between 0 and 6
group by visited_on
having count(*) = 7
order by visited_on