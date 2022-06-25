select 
    if(id = (select count(*) from seat) and id % 2 = 1, 
        id,
        if(id % 2 = 1,
            id + 1,
            id - 1)) as id,
    student
from seat
order by id