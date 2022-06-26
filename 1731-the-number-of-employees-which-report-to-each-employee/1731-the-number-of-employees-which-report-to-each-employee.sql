select 
    m.employee_id, 
    e.name, 
    m.reports_count,
    m.average_age
from employees e
inner join
   (select 
        reports_to as employee_id, 
        count(reports_to) as reports_count, 
        round(avg(age)) as average_age
    from employees
    where reports_to is not null
    group by reports_to) m
on e.employee_id = m.employee_id
order by employee_id
