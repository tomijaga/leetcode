select employee_id, if(name like "M%" or employee_id % 2 = 0, 0, salary) as bonus
from employees
order by employee_id