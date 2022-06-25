select product_id as product_id, round(total/units, 2) as average_price
from
    (select p.product_id, sum(u.units) as units, sum(p.price * u.units) as total
    from prices p, unitsSold u
    where p.product_id = u.product_id
        and u.purchase_date <= p.end_date
        and u.purchase_date >= p.start_date
    group by product_id
    ) t
