select (date_part('year',created_at)*100 + 
        date_part('month',created_at)) yearmonth,
       date_part('hour',created_at) hour_of_day,
       count(1) no_of_requests
from logs
group by date_part('year',created_at)*100 + 
          date_part('month',created_at),
          date_part('hour',created_at) 
order by yearmonth desc, hour_of_day;
