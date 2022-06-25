# Write your MySQL query statement below
select s1.sale_date, (s1.sold_num -  s2.sold_num) as diff
from sales s1, sales s2
where s1.sale_date = s2.sale_date
    and s1.fruit like 'apples'
    and s1.fruit not like s2.fruit
order by s1.sale_date