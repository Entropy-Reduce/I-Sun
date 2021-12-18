# drop tables
drop table if exists tx;
drop table if exists user;
drop table if exists user_dapp_index;
drop table if exists tokens;
drop table if exists pairs;
drop table if exists swap_tokens;
drop table if exists token_user;
drop table if exists swap_tx;
drop table if exists token_tx;
drop table if exists swap_user;

# create tables
create table tx (
    id bigint primary key auto_increment,
    from_ varchar(255) not null,
    to_ varchar(255) not null,
    time_ bigint not null,
    price bigint not null,
    dapp_id bigint not null,
    index_ bigint not null
);

create table user (
    identifier varchar(255) primary key,
    principal varchar(255) not null
);

create table user_dapp_index (
    identifier varchar(255) not null,
    dapp_id bigint not null,
    index_ bigint not null
);

create table tokens (
    canister_id varchar(255) primary key,
    decimals bigint not null,
    fee bigint not null,
    index_ bigint not null,
    logo text,
    name_ varchar(255) not null,
    owner_ varchar(255) not null,
    symbol varchar(255) not null,
    timestamp_ bigint not null,
    supply bigint not null
);

create table pairs (
    id varchar(255) primary key,
    supply bigint not null,
    token0 text,
    token1 text,
    lp_token text,
    creator text,
    last_update_time bigint not null,
    price0_cumulative bigint not null,
    price1_cumulative bigint not null,
    k bigint not null
);

create table swap_tokens(
    id varchar(255) primary key,
    name_ varchar(255) not null,
    symbol varchar(255) not null,
    decimals bigint not null,
    fee bigint not null,
    supply bigint not null
);

create table token_user(
    principal varchar(255) primary key,
    balances  text,
    transactions  text
);

create table swap_user(
    principal varchar(255) primary key,
    balances text,
    lp_balances text,
    transactions text
);

create table token_tx(
    canister_id varchar(255) not null,
    index_ bigint,
    content text
);

create table swap_tx(
    index_ bigint primary key,
    content text
);

-- # test data

-- insert into user (identifier, principal) values ("aaa", "aaaaa");
-- insert into user (identifier, principal) values ("bbb", "bbbbb");
-- insert into user (identifier, principal) values ("ccc", "ccccc");
-- insert into user (identifier, principal) values ("ddd", "ddddd");
-- insert into user (identifier, principal) values ("eee", "eeeee");

-- insert into user_dapp_index (user_id, dapp_id, index_) values (1, 1, 1);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (1, 1, 2);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (1, 1, 3);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (1, 2, 4);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (1, 2, 6);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (1, 3, 7);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (1, 3, 8);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (1, 3, 9);

-- insert into user_dapp_index (user_id, dapp_id, index_) values (2, 1, 3);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (2, 1, 4);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (2, 1, 5);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (2, 1, 8);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (2, 2, 1);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (2, 2, 2);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (2, 2, 3);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (2, 3, 1);
-- insert into user_dapp_index (user_id, dapp_id, index_) values (2, 3, 4);

-- insert into tx (from_, to_, time_, price, dapp_id, index_) values ("aaa", "bbb", 101, 100, 1, 1);
-- insert into tx (from_, to_, time_, price, dapp_id, index_) values ("aaa", "ccc", 102, 100, 1, 2);
-- insert into tx (from_, to_, time_, price, dapp_id, index_) values ("aaa", "ddd", 103, 100, 1, 3);
-- insert into tx (from_, to_, time_, price, dapp_id, index_) values ("aaa", "eee", 104, 100, 1, 1);
-- insert into tx (from_, to_, time_, price, dapp_id, index_) values ("bbb", "aaa", 105, 100, 1, 2);
-- insert into tx (from_, to_, time_, price, dapp_id, index_) values ("bbb", "aaa", 106, 100, 1, 3);
-- insert into tx (from_, to_, time_, price, dapp_id, index_) values ("ccc", "ddd", 107, 100, 1, 1);
-- insert into tx (from_, to_, time_, price, dapp_id, index_) values ("ddd", "bbb", 108, 100, 1, 2);
-- insert into tx (from_, to_, time_, price, dapp_id, index_) values ("eee", "bbb", 109, 100, 1, 3);
-- insert into tx (from_, to_, time_, price, dapp_id, index_) values ("ccc", "bbb", 110, 100, 1, 1);

-- # source E:\Temp\Rust projects\DATA_SYNC\create_tables.sql
