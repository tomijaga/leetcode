select 
    p.product_id, 
    round(sum(p.price * u.units) / sum(u.units), 2)  as average_price
from prices p, unitsSold u
where p.product_id = u.product_id
    and u.purchase_date <= p.end_date
    and u.purchase_date >= p.start_date
group by product_id
