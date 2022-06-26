select product_name, p.product_id, order_id, order_date
from products p
inner join 
(select *
from orders
where (product_id, order_date) in
(select product_id, max(order_date) as order_date
    from orders
    group by product_id) ) o
on p.product_id = o.product_id
order by product_name, p.product_id, order_id
