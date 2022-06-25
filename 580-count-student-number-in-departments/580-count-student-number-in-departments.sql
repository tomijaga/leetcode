select dept_name, ifnull(cnt, 0) as student_number
from department d
left join
    (select dept_id, count(*) as cnt
    from student
    group by dept_id) s
on d.dept_id = s.dept_id
order by student_number desc, dept_name

