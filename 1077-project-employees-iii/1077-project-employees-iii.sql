with exp as (select 
                p.project_id,
                max(experience_years)  as experience_years
             from employee e
             join project p
             on e.employee_id = p.employee_id
             group by p.project_id)
select 
    exp.project_id,
    p.employee_id
from employee e
left join project p
on p.employee_id = e.employee_id, exp
where p.project_id = exp.project_id 
    and e.experience_years = exp.experience_years

