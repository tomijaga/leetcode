select e.employee_id, (select count(team_id) from Employee where e.team_id = team_id) as team_size
from Employee e