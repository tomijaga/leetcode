# select o1.customer_id, o1.product_id
# from orders o1,
#     (select customer_id, max(cnt) as cnt
#     from (select customer_id, count(product_id) as cnt
#             from orders
#             group by customer_id, product_id) o
#     group by customer_id) o2
# where o1.customer_id = o2.customer_id
# group by o1.customer_id, o1.product_id
# having count(*) = o2.cnt and o1.customer_id = o2.customer_id

select o.customer_id, p.product_id, p.product_name
from customers c
inner join 
    (select o1.customer_id, o1.product_id, o2.cnt
    from orders o1, (select _o.customer_id, max(_o.cnt) as cnt
            from (select __o.customer_id, count(__o.product_id) as cnt
                    from orders __o
                    group by customer_id, product_id) _o
            group by customer_id) o2
    where o1.customer_id = o2.customer_id        
    group by o1.customer_id, o1.product_id
    having count(o1.product_id) = o2.cnt 
) o
on c.customer_id = o.customer_id
inner join products p
on p.product_id = o.product_id