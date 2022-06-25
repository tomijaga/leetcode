with const as (select count(*) as len from users)
select contest_id, round(count(*) / const.len * 100, 2) as percentage
from register, const
group by contest_id
order by percentage desc, contest_id