# Write your MySQL query statement below
select id, 
    if(p_id is null, 
       "Root",
       if(id in (select p_id as id from tree),
            "Inner",
            "Leaf")
      ) as type
from tree
order by id