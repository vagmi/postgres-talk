drop table if exists movie_search;

create table movie_search (
    movie_id bigint primary key references movies(id),
    content tsvector
);

create index idx_movie_search on movie_search using gin (content);

insert into movie_search 
select id, to_tsvector('english', coalesce(name, '') || ' ' || coalesce(abstract, ''))
from movies left join movie_abstracts_en on id=movie_id;

