select 
    _p2.id as p1, 
    _p1.id as p2, 
    abs((_p2.y_value - _p1.y_value) * (_p2.x_value - _p1.x_value)) as area
from points _p1, points _p2
where _p1.id > _p2.id
having area > 0
order by area desc, _p2.id, _p1.id