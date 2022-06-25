select ifnull(round(count(*) / const.len * 100, 2), 0) as immediate_percentage
from delivery d, (select count(*) as len from delivery) const
where order_date like customer_pref_delivery_date
