select visitor_id, 
       visited, visited_time, 
       registered, registered_time, 
       paid, paid_time
from (
    -- the visits query
    select visitor_id, 1 AS visited, min(created_at) AS  visited_time
    from events
    where data->>'type' = 'VisitedEvent'
    group by visitor_id
) visits left join lateral (
  -- For each row, get the first time the visitor_id registered
  -- the registrations query
  select 1 AS registered, created_at AS registered_time
  from events
  where visitor_id = visits.visitor_id  and
        data->>'type' = 'RegisteredEvent' and
        created_at between visits.visited_time and (visits.visited_time + interval '2 months') -- 60 days
  order by created_at
  limit 1
) registrations on true left join lateral (
  -- the payments query
  select 1 as paid, created_at as paid_time
  from events
  where visitor_id = visits.visitor_id and
        data ->>'type' = 'PaidUserEvent' and
        created_at between visits.visited_time and (visits.visited_time + interval '2 months')
  order by created_at
  limit 1
) payments on true;
