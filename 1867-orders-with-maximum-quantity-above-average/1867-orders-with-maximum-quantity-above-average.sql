with const as (select order_id, max(quantity) as max_q, avg(quantity) as avg_q
    from ordersDetails
    group by order_id)
select order_id
from const o1
where max_q > (select max(avg_q) from const) 