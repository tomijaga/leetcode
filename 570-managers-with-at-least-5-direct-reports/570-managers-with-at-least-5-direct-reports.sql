    
select name
from employee e
inner join
    (select managerId
    from employee
    group by managerId
    having count(managerId) > 4) r
on e.id = r.managerid