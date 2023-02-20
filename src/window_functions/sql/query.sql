with view_stats as (
    select video_id, creator_id, count(1) as view_count, 
           date_part('year', views.created_at)*100 + date_part('month', views.created_at) as yearmonth
    from views
    join videos on videos.id=views.video_id
    group by video_id, creator_id, yearmonth
)
select video_id, creator_id, view_count, yearmonth,
       sum(view_count) over by_creator as channel_views,
       rank() over by_creator_views as rank
from view_stats
window by_creator as (partition by creator_id),
       by_creator_views as (partition by creator_id order by view_count desc)
order by creator_id, rank asc;
