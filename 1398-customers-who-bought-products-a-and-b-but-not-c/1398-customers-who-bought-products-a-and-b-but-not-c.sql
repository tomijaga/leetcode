with ab as 
(select distinct o1.customer_id
from orders o1, orders o2
where o1.customer_id = o2.customer_id
    and 
    ((o1.product_name like "A" and o2.product_name like "B"))),
    
c as 
(select distinct customer_id 
from orders
where product_name like "C"),

ids as
(select ab.customer_id
from ab
left join c 
on ab.customer_id = c.customer_id
where c.customer_id is null)

select c.customer_id, customer_name
from ids 
left join customers c
on ids.customer_id = c.customer_id
order by c.customer_id