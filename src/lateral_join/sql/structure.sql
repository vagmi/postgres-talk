create extension if not exists "uuid-ossp";

create table events (
    id bigserial primary key,
    visitor_id uuid not null,
    data jsonb not null,
    created_at timestamptz not null default now()
);
