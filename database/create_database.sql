drop database if exists `cafelatte`;

create database `cafelatte`;

use `cafelatte`;

create table `user` (
    `id` int primary key auto_increment,
    `name` varchar(40) not null,
    `surname` varchar(40) not null,
    `phone_number` varchar(40),
    `email` varchar(50) not null,
    `password` varchar(100) not null
);
