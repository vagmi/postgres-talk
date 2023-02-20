select json_build_object(
  'id', id, 
  'name', name, 
  'date', date, 
  'cast', movie_casts.cast,
  'movies_in_series', other_movies.movies_in_series
) data from
(select id, name, date, parent_id 
from movies where id=$1) movie left join lateral (
  select json_agg(
      json_build_object('role', role, 
                        'person_id', person_id, 
                        'actor_name', people.name)) cast
  from casts 
  join people on casts.person_id = people.id
  where casts.movie_id = movie.id and role != ''
) movie_casts on true left join lateral (
  select json_agg(
    json_build_object('id', m.id,
                      'name', name,
                      'date', date
                      ) 
  ) movies_in_series
  from movies m
  where (m.id=movie.parent_id or m.parent_id=movie.parent_id) and m.id != movie.id
) other_movies on true;
