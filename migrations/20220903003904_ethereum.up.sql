-- This schema relates specifically to Ethereum oracles and other
-- Ethereum-related data
create schema if not exists ethereum;

-- Procedure to always set the updated_at timestamp on update
create or replace function trigger_set_timestamp ()
  returns trigger
  language plpgsql
  as $$
begin
  new.updated_at := current_timestamp;
  return new;
end;
$$;

-- Indexed Ethereum contract metadata
create table if not exists ethereum.contracts (
  id int primary key generated always as identity,
  chain_id int not null,
  address varchar(40) not null,
  abi jsonb not null,
  created_at timestamp not null default now(),
  updated_at timestamp not null default now()
);

create or replace trigger set_timestamp before update on ethereum.contracts for each row execute function trigger_set_timestamp ();

-- Indexed EVM events
create table if not exists ethereum.events (
  id int primary key generated always as identity,
  signature text not null,
  hash varchar(64) not null,
  abi jsonb not null,
  created_at timestamp not null default now(),
  updated_at timestamp not null default now()
);

create or replace trigger set_timestamp before update on ethereum.events for each row execute function trigger_set_timestamp ();

-- Join table for `eth_contracts` and `eth_listeners`
create table if not exists ethereum.contracts_events (
  contract_id int not null references ethereum.contracts (id),
  event_id int not null references ethereum.events (id)
);
