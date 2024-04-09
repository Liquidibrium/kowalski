-- Add up migration script here


create table users
(
    id         UUID primary key,
    created_at timestamp    not null default now(),
    updated_at timestamp    not null default now(),
    first_name varchar(255),
    last_name  varchar(255),
    email      varchar(255) not null,
    password   varchar(255),
    status     varchar(255) not null,
    provider   varchar(255) not null
);

create table teams
(
    id         UUID primary key,
    created_at timestamp    not null default now(),
    updated_at timestamp    not null default now(),
    name       varchar(255) not null
);

create table team_members
(
    id         UUID primary key,
    created_at timestamp    not null default now(),
    updated_at timestamp    not null default now(),
    user_id    UUID         not null,
    team_id    UUID         not null,
    role       varchar(255) not null
);


create table analyzer_requests
(
    id                  UUID primary key,
    created_at          timestamp     not null default now(),
    updated_at          timestamp     not null default now(),
    user_id             UUID,
    status              varchar(255)  not null,
    url_link            varchar(1024) not null,
    -- this might not be required to be saved or should be encrypted
    analyzer_result     jsonb,
    team_id             UUID          not null,
    repository_id       UUID          not null,

    branch_from         varchar(255)  not null,
    head_sha            varchar(255)  not null,

    branch_to           varchar(255)  not null,
    base_sha            varchar(255)  not null,

    pull_request_number varchar(255)  not null,
    pull_request_title  varchar(255)  not null
);


create table repositories
(
    id           UUID primary key,
    created_at   timestamp     not null default now(),
    updated_at   timestamp     not null default now(),
    git_provider varchar(255)  not null,

    owner        varchar(255)  not null,
    name         varchar(255)  not null,
    url          varchar(1024) not null,

    team_id      UUID          not null
);

create table api_keys
(
    id         UUID primary key,
    key        varchar(1024) not null,
    created_at timestamp     not null default now(),
    updated_at timestamp     not null default now(),
    user_id    UUID          not null
);
