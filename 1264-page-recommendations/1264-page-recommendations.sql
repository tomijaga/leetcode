
select distinct page_id as recommended_page
from 
    (select if(user2_id = 1, user1_id, user2_id) as user_id
    from friendship 
    where user1_id = 1 or user2_id = 1) f
inner join likes l
on f.user_id = l.user_id
where page_id not in (select page_id from likes where user_id = 1)