create extension if not exists tablefunc;

select * from crosstab(
  -- dataset query
  $q$
    select (date_part('year',created_at)*100 + date_part('month',created_at)) yearmonth, 
            date_part('hour',created_at) hour_of_day,
            count(1) no_of_requests 
    from logs
    group by date_part('year',created_at)*100 + date_part('month',created_at),
             date_part('hour',created_at) order by yearmonth desc, hour_of_day
  $q$,
  -- category query
  $q$ 
     select * from generate_series(0,23)
  $q$
) 
as (datepart float, h00 bigint, h01 bigint, h02 bigint, 
                    h03 bigint, h04 bigint, h05 bigint, h06 bigint, 
                    h07 bigint, h08 bigint, h09 bigint, h10 bigint, 
                    h11 bigint, h12 bigint, h13 bigint, h14 bigint,
                    h15 bigint, h16 bigint, h17 bigint, h18 bigint, 
                    h19 bigint, h20 bigint, h21 bigint, h22 bigint, 
                    h23 bigint)
order by datepart asc;
