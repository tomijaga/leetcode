# Write your MySQL query statement below
select customer_number
from 
    (select customer_number, count(order_number) as order_number
    from orders
    group by customer_number) total_orders
order by order_number desc
limit 1