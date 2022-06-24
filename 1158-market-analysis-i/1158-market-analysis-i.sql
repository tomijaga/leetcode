select user_id as buyer_id, join_date, ifnull(order_2019, 0) as orders_in_2019
from users u
left join
    (select buyer_id, count(*) as order_2019
    from orders
    where order_date like "2019%"
    group by buyer_id) t
on u.user_id = t.buyer_id