# Write your MySQL query statement below
select name, r.amount as balance
from
    (select account, sum(amount) as amount
    from transactions
    group by account
    having sum(amount) > 10000
    ) r
left join users u
on r.account = u.account
