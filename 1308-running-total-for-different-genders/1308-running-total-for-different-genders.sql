select s2.gender, s2.day, sum(s1.score_points) as total 
from scores s1, scores s2
where s1.day <= s2.day and s1.gender = s2.gender
group by s2.day, s2.gender
order by s2.gender, s2.day