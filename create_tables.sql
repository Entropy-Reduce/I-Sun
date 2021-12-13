# drop tables
drop table if exists tx;
drop table if exists user;
drop table if exists user_dapp_index;

# create tables
create table tx (
    id int primary key auto_increment,
    from_ varchar(255) not null,
    to_ varchar(255) not null,
    time_ int not null,
    price int not null,
    dapp_id int not null,
    index_ int not null
);

create table user (
    id int primary key auto_increment,
    identifier varchar(255) not null unique,
    principal varchar(255)
);

create table user_dapp_index (
    user_id int not null,
    dapp_id int not null,
    index_ int not null
);

# test data

insert into user (identifier, principal) values ("aaa", "aaaaa");
insert into user (identifier, principal) values ("bbb", "bbbbb");
insert into user (identifier, principal) values ("ccc", "ccccc");
insert into user (identifier, principal) values ("ddd", "ddddd");
insert into user (identifier, principal) values ("eee", "eeeee");

insert into user_dapp_index (user_id, dapp_id, index_) values (1, 1, 1);
insert into user_dapp_index (user_id, dapp_id, index_) values (1, 1, 2);
insert into user_dapp_index (user_id, dapp_id, index_) values (1, 1, 3);
insert into user_dapp_index (user_id, dapp_id, index_) values (1, 2, 4);
insert into user_dapp_index (user_id, dapp_id, index_) values (1, 2, 6);
insert into user_dapp_index (user_id, dapp_id, index_) values (1, 3, 7);
insert into user_dapp_index (user_id, dapp_id, index_) values (1, 3, 8);
insert into user_dapp_index (user_id, dapp_id, index_) values (1, 3, 9);

insert into user_dapp_index (user_id, dapp_id, index_) values (2, 1, 3);
insert into user_dapp_index (user_id, dapp_id, index_) values (2, 1, 4);
insert into user_dapp_index (user_id, dapp_id, index_) values (2, 1, 5);
insert into user_dapp_index (user_id, dapp_id, index_) values (2, 1, 8);
insert into user_dapp_index (user_id, dapp_id, index_) values (2, 2, 1);
insert into user_dapp_index (user_id, dapp_id, index_) values (2, 2, 2);
insert into user_dapp_index (user_id, dapp_id, index_) values (2, 2, 3);
insert into user_dapp_index (user_id, dapp_id, index_) values (2, 3, 1);
insert into user_dapp_index (user_id, dapp_id, index_) values (2, 3, 4);

insert into tx (from_, to_, time_, price, dapp_id, index_) values ("aaa", "bbb", 101, 100, 1, 1);
insert into tx (from_, to_, time_, price, dapp_id, index_) values ("aaa", "ccc", 102, 100, 1, 2);
insert into tx (from_, to_, time_, price, dapp_id, index_) values ("aaa", "ddd", 103, 100, 1, 3);
insert into tx (from_, to_, time_, price, dapp_id, index_) values ("aaa", "eee", 104, 100, 1, 1);
insert into tx (from_, to_, time_, price, dapp_id, index_) values ("bbb", "aaa", 105, 100, 1, 2);
insert into tx (from_, to_, time_, price, dapp_id, index_) values ("bbb", "aaa", 106, 100, 1, 3);
insert into tx (from_, to_, time_, price, dapp_id, index_) values ("ccc", "ddd", 107, 100, 1, 1);
insert into tx (from_, to_, time_, price, dapp_id, index_) values ("ddd", "bbb", 108, 100, 1, 2);
insert into tx (from_, to_, time_, price, dapp_id, index_) values ("eee", "bbb", 109, 100, 1, 3);
insert into tx (from_, to_, time_, price, dapp_id, index_) values ("ccc", "bbb", 110, 100, 1, 1);

# source E:\Temp\Rust projects\DATA_SYNC\create_tables.sql