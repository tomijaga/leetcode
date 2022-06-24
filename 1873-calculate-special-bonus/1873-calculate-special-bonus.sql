# Write your MySQL query statement below
select employee_id, 
    if(name not like 'm%', 
       if( employee_id % 2 = 0, 0, salary),
       0) as bonus
from employees
order by employee_id