select 
    name as warehouse_name,
    sum(units * p.height * p.length * p.width) as volume
from warehouse w
left join products p
on w.product_id = p.product_id
group by name