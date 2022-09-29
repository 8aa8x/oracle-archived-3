-- HTTP Responder will hold information needed to respond to HTTP(S) oracles
create table if not exists responders.http (
  id char(21) unique not null references responders.responders (id) on delete cascade,
  endpoint varchar(256) not null,
  headers text array not null default '{}',
  created_at timestamp not null default now(),
  updated_at timestamp not null default now()
);

create or replace trigger update_intermediate_table before update on responders.http for each row execute function update_responder_intermediate_table ();

-- A job queue that the HTTP responder can listen to and update
create table if not exists responders.http_jobs (
  id int primary key generated always as identity,
  responder_id char(21) references responders.http (id) on delete set null,
  json_body jsonb not null,
  created_at timestamp not null default now()
);
