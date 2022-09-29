-- A specific listener for Ethereum blockhain emitted events
create table if not exists listeners.ethereum_events (
  id char(21) unique not null references listeners.listeners (id) on delete cascade,
  chain_id int not null,
  contract_address varchar(40) not null,
  event_hash varchar(64) not null,
  confirmations int not null
);

create or replace trigger update_intermediate_table before update on listeners.ethereum_events for each row execute function update_listener_intermediate_table ();

-- Keeps track of Ethereum emitted events that were found when listening
create table if not exists listeners.ethereum_events_indexes (
  id int primary key generated always as identity,
  listener_id char(21) references listeners.ethereum_events (id) on delete set null,
  block_hash char(64) not null,
  transaction_hash char(64) not null,
  log_index int not null,
  topics char(64) array not null,
  extra_data bytea not null,
  created_at timestamp not null default now(),
  constraint listener_log_unique unique (listener_id, block_hash, transaction_hash, log_index)
);
