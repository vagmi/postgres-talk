select json_agg(
    json_build_object(
  'id', id, 
  'name', name, 
  'date', date, 
  'abstract', abstract,
  'cast', movie_casts.cast,
  'movies_in_series', other_movies.movies_in_series
)) data from
(
    select movie_id from movie_search, plainto_tsquery($1) query 
    where content @@ query order by ts_rank_cd(content, query) desc
    
) search_results join lateral

(select movies.id, name, date, parent_id, abstract
from movies 
left join movie_abstracts_en on movies.id=movie_abstracts_en.movie_id
where id=search_results.movie_id
) movie on movie.id=search_results.movie_id 
left join lateral (
  select movie.id movie_id, json_agg(
      json_build_object('role', role, 
                        'person_id', person_id, 
                        'actor_name', people.name)) cast
  from casts 
  join people on casts.person_id = people.id
  where casts.movie_id = movie.id and role != ''
) movie_casts on movie.id=movie_casts.movie_id left join lateral (
  select movie.id movie_id, json_agg(
    json_build_object('id', m.id,
                      'name', name,
                      'date', date,
                      'abstract', abstract
                      ) 
  ) movies_in_series
  from movies m left join movie_abstracts_en ma on ma.movie_id = m.id
  where (m.id=movie.parent_id or m.parent_id=movie.parent_id) and m.id != movie.id
) other_movies on movie.id=other_movies.movie_id;
