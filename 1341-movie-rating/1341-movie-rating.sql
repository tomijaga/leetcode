with grouped_users as (select user_id, count(*) as cnt
    from movieRating
    group by user_id),
    grouped_movies as  (select movie_id, avg(rating) as rating
    from movieRating
    where created_at like "2020-02%"
    group by movie_id)
select min(u.name) as results
from users u
inner join grouped_users m
on u.user_id = m.user_id
where m.cnt in (select max(cnt) from grouped_users)
group by m.cnt

union

select min(m.title) as results
from movies m 
inner join grouped_movies r
on m.movie_id = r.movie_id
where r.rating in (select max(rating) from grouped_movies)
group by r.rating