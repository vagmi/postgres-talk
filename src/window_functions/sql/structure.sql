create extension if not exists "uuid-ossp";

create table videos (
    id uuid primary key,
    creator_id uuid not null,
    created_at timestamptz not null default now()
);

create table views (
    id bigserial primary key,
    video_id uuid not null,
    user_id uuid not null,
    time_played int not null,
    created_at timestamptz not null default now()
);

create index idx_views_video_id on views(video_id);
create index idx_views_user_id on views(user_id);
