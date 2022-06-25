select 
    substr(trans_date, 1, 7) as month, 
    country, 
    count(*) as trans_count,
    sum(if(state like "approved", 1, 0)) as approved_count,
    sum(amount) as trans_total_amount,
    sum(if(state like "approved", amount, 0)) as approved_total_amount
from transactions
group by month, country