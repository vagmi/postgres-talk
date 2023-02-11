create table logs (
    id serial primary key,
    message text,
    created_at timestamptz not null default now()
);
