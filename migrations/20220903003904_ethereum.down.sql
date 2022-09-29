drop table if exists ethereum.contracts_events;

drop trigger if exists set_timestamp on ethereum.events;

drop table if exists ethereum.events;

drop trigger if exists set_timestamp on ethereum.contracts;

drop table if exists ethereum.contracts;

drop function if exists trigger_set_timestamp;

drop schema if exists ethereum;
