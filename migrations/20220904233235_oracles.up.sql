---- Listeners ----
-------------------
-- If you need to add new types in the future refer to this
-- https://stackoverflow.com/a/7834949
create type listener_type as enum (
  'ethereum'
);

-- This schema holds relative information for all listeners
create schema if not exists listeners;

-- Intermediate tables for listeners
create table if not exists listeners.listeners (
  id char(21) primary key,
  type listener_type not null,
  user_id int not null references auth.users (id),
  active boolean not null default true,
  title varchar(64) not null,
  created_at timestamp not null default now(),
  updated_at timestamp not null default now()
);

create or replace trigger set_timestamp before update on listeners.listeners for each row execute function trigger_set_timestamp ();

-- Can be used by new listeners to auto-update the `updated_at` column
create or replace function listeners.update_listener_intermediate_table ()
  returns trigger
  language plpgsql
  as $$
begin
  update
    listeners.listeners
  set
    updated_at = current_timestamp
  where
    listeners.listeners.id = new.id;
  return new;
end;
$$;

---- Responders ----
--------------------
-- If you need to add new types in the future refer to this
-- https://stackoverflow.com/a/7834949
create type responder_type as enum (
  'http'
);

-- This schema holds relative information for all responders
create schema if not exists responders;

-- Intermediate tables for responders
create table if not exists responders.responders (
  id char(21) primary key,
  type responder_type not null
);

-- Can be used by new responders to auto-update the `updated_at` column
create or replace function responders.update_responder_intermediate_table ()
  returns trigger
  language plpgsql
  as $$
begin
  update
    responders.responders
  set
    updated_at = current_timestamp
  where
    responders.responders.id = new.id;
  return new;
end;
$$;

---- Joins ----
---------------
-- Join table between listeners and responders
-- (intentionally added to the `public` schema)
create table if not exists listeners_responders (
  listener_id char(21) not null references listeners.listeners on delete cascade,
  responder_id char(21) not null references responders.responders on delete cascade
);
