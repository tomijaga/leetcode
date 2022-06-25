select 
    name as warehouse_name,
    sum(units * p.volume) as volume
from warehouse w
left join 
    (select product_id, (width * length * height) as volume
    from products) p
on w.product_id = p.product_id
group by name