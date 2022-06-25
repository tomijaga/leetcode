select p.product_id, p.product_name
from
    (select distinct product_id
    from sales 
     group by product_id
     having min(sale_date) >="2019-01-01" 
     and max(sale_date) <= "2019-03-31") s
left join product p
on p.product_id = s.product_id