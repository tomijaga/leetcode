# Write your MySQL query statement below
select user_id, max(time_stamp) as last_stamp
from
    (select *
    from logins 
    where time_stamp like "2020%") table_2020
group by user_id

