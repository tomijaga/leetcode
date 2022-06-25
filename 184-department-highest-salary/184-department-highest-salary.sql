select d.d_name as Department, e.name as employee, d.salary
from employee e,
   (select max(salary) as salary, departmentId, part.name as d_name
    from employee e1
    inner join department part
    on e1.departmentId = part.id
    group by departmentId) d
where d.departmentId = e.departmentId and d.salary = e.salary 
