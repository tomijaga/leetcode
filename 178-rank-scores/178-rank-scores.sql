with uniq as (select distinct score from scores )
select 
   score,
   (select count(*) from (select distinct score s from scores ) u where u.s >= s1.score) as rank
from scores s1
order by score desc
