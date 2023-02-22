drop table if exists posts;
drop table if exists users;

create table users (
    id bigserial primary key,
    handle text not null unique
);

create table posts (
    id bigserial primary key,
    user_id bigint not null references users(id),
    title text not null,
    body text not null
);

grant usage on sequence users_id_seq to app_rls_user, app_rls_admin, app_rls_anonymous;
grant usage on sequence posts_id_seq to app_rls_user, app_rls_admin, app_rls_anonymous;


alter table users enable row level security;
alter table posts  enable row level security;

grant select, insert,update, delete on table users to app_rls_anonymous, app_rls_user, app_rls_admin;

create policy select_current_user
on users
for select 
to app_rls_user using (handle = nullif(current_setting('rls.username', true), ''));

create policy update_current_user
on users
for update
to app_rls_user using (handle = nullif(current_setting('rls.username', true), ''));

create policy crud_admin_users
on users
for all
to app_rls_admin using (true);

create policy read_all_posts
on users
for select 
to app_rls_anonymous using (true);


