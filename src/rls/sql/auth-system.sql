do language plpgsql $$
declare
  role_exists boolean;
begin
  select true into role_exists from pg_roles where rolname='app_rls';

  if role_exists is null then
    create role app_rls;
execute format('grant app_rls to %I',current_user);
    execute format('grant connect on database %I to app_rls',current_database());
  end if;

  select true into role_exists from pg_roles where rolname='app_rls_anonymous';
  if role_exists is null then
    create role app_rls_anonymous;
    grant app_rls_anonymous to app_rls;
  end if;


  select true into role_exists from pg_roles where rolname='app_rls_user';
  if role_exists is null then
    create role app_rls_user;
    grant app_rls_user to app_rls;
  end if;

  select true into role_exists from pg_roles where rolname='app_rls_admin';
  if role_exists is null then
    create role app_rls_admin;
    grant app_rls_admin to app_rls;
  end if;
end;
$$;
