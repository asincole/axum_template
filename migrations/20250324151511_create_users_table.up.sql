create table if not exists "users"
(
    user_id    uuid primary key                       not null default uuid_generate_v1mc(),
    first_name character varying                      not null,
    last_name  character varying                      not null,
    username   text collate "case_insensitive" unique not null,
    email      text collate "case_insensitive" unique not null,
    created_at timestamptz                            not null default now(),
    updated_at timestamptz                            not null default now(),
    version    integer                                not null default 1
);

SELECT trigger_updated_at('users');
SELECT trigger_increment_version('users');
