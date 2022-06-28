select L1.log_id as start_id, min(L2.log_id) as end_id
from 
    (select * from logs where log_id - 1 not in (select * from logs)) L1,
    (select * from logs where log_id + 1 not in (select * from logs)) L2
where L1.log_id <= L2.log_id
group by L1.log_id