select s.student_id, s.student_name, sj.subject_name, ifnull(count(e.subject_name), 0) as attended_exams
from students s
cross join subjects sj
left join examinations e
on s.student_id = e.student_id and sj.subject_name = e.subject_name
group by s.student_id, sj.subject_name
order by s.student_id, sj.subject_name
