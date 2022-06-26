
   
select p1.product_id, new_price as price
from products p1,
      (select product_id, max(change_date) as date
        from products 
        where change_date <= "2019-08-16"
        group by product_id) p2
where p1.change_date = date
    and p1.product_id = p2.product_id
     
union

select product_id, 10 as price
from products
where product_id not in (select product_id
        from products 
        where change_date <= "2019-08-16"
        group by product_id)
