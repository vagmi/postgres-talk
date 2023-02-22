with view_stats as (
    select 
        date_part('year', views.created_at)*100 + date_part('month', views.created_at) as yearmonth,
        video_id, creator_id, count(1) as view_count
    from views
    join videos on videos.id=views.video_id
    group by video_id, creator_id, yearmonth
)
select yearmonth, creator_id, video_id, view_count, 
       sum(view_count) over by_creator as total_views,
       to_char(view_count/(sum(view_count) over by_creator) * 100, 'fm99%') percent_views,
       rank() over by_creator_views as rank
from view_stats
window by_creator as (partition by creator_id),
       by_creator_views as (partition by creator_id order by view_count desc)
order by creator_id, yearmonth;
