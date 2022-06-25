select 
    c.country_name, 
    if(average <= 15, 
       "Cold",
        if(average>=25, 
            "Hot",
            "Warm")) as weather_type
from countries c,
    (select country_id, avg(weather_state) as average
    from weather
    where day like "2019-11%"
    group by country_id) w
where c.country_id = w.country_id