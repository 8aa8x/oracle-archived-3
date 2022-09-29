drop table if exists listeners_responders;

drop function if exists update_responder_intermediate_table;

drop trigger if exists set_timestamp on responders.http;

drop table if exists responders.http;

drop table if exists responders.responders;

drop schema if exists responders;

drop type if exists responder_type;

drop function if exists update_listener_intermediate_table;

drop trigger if exists set_timestamp on listeners.ethereum;

drop table if exists listeners.ethereum;

drop table if exists listeners.listeners;

drop schema if exists listeners;

drop type if exists listener_type;
