select customer_id
from (select *
    from customer
    group by customer_id, product_key) c
group by customer_id
having count(*) = (select count(*) as cnt from product)