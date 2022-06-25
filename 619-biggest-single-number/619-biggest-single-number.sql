select if(count(num) = 1, num, null) as num
from myNumbers
group by num
order by num desc
limit 1