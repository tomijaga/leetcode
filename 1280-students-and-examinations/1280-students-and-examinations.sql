select s.student_id, s.student_name, sj.subject_name, ifnull(cnt, 0) as attended_exams
from students s
cross join subjects sj
left join
   (select student_id, subject_name, count(*) as cnt
    from examinations
    group by student_id, subject_name) e
on s.student_id = e.student_id and sj.subject_name = e.subject_name
order by student_id, subject_name
