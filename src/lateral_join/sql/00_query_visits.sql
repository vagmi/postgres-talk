select visitor_id, 1 AS visited, min(created_at) as visited_time
from events
where data->>'type' = 'VisitedEvent'
group by visitor_id;
