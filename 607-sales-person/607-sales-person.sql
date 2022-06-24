# Write your MySQL query statement below


select name 
from SalesPerson s
where s.sales_id not in
(select o.sales_id
    from orders o
    where o.com_id in (select com_id from company where name like "RED")) 
