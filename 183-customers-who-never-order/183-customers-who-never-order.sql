# Write your MySQL query statement below
select name as "Customers"
from customers c
left join orders o
on o.customerId = c.id
where o.id is null

# select name as "Customers"
# from customers
# where id not in (
#     select customerId from orders
# )