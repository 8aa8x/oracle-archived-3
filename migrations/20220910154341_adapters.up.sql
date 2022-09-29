-- Adapters live in this schema
create schema if not exists adapters;

-- An adapter for Ethereum Events -> HTTP Jobs
create table if not exists adapters.ethereum_http (
  id int primary key generated always as identity,
  ethereum_runs_id int not null references listeners.ethereum_events_indexes (id),
  http_jobs_id int not null references responders.http_jobs (id),
  created_at timestamp not null default now()
);
