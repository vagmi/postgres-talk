create table logs (
    id serial primary key,
    message text,
    created_at timestamptz not null default now()
);
create table names(
    id serial primary key,
    message text,
    created_at timestamptz not null default now()
);
