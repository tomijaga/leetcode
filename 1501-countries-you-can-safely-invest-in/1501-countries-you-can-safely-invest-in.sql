# when global as (select sum(duration) d from calls)
select c.name as country
from country c
left join person p
on c.country_code like substr(p.phone_number, 1, 3)
inner join calls ca
on ca.caller_id = p.id or ca.callee_id = p.id
group by c.country_code
having avg(ca.duration) > (select sum(duration)/count(*) d from calls)
