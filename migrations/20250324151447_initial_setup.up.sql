-- Inspiration drawn HEAVILY from https://github.com/launchbadge/realworld-axum-sqlx
-- Other recommendation links:
--      https://justatheory.com/2012/04/postgres-use-timestamptz/

-- This extension gives us `uuid_generate_v1mc()` which generates UUIDs that cluster better than `gen_random_uuid()`
-- while still being difficult to predict and enumerate.
-- Also, while unlikely, `gen_random_uuid()` can in theory produce collisions which can trigger spurious errors on
-- insertion, whereas it's much less likely with `uuid_generate_v1mc()`.
create extension if not exists "uuid-ossp";

-- We try to ensure every table has `created_at` and `updated_at` columns, which can help immensely with debugging
-- and auditing.
--
-- While `created_at` can just be `default now()`, setting `updated_at` on update requires a trigger which
-- is a lot of boilerplate. These two functions save us from writing that every time as instead we can just do
--
-- select trigger_updated_at('<table name>');
--
-- after a `CREATE TABLE`.
create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
return NEW;
end;
$$ language plpgsql;

create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plpgsql;

create or replace function inc_version()
returns trigger as
$$
begin
    -- Set version to 1 if it's null (first update) or increment otherwise
    if new.version is null then
        new.version := 1;
else
        new.version := new.version + 1;
end if;
return new;
end;
$$ LANGUAGE plpgsql;

create or replace function trigger_increment_version(tablename regclass)
    returns void as
$$
begin
execute format('
        CREATE TRIGGER set_version
        BEFORE UPDATE ON %s
        FOR EACH ROW
        WHEN (OLD IS DISTINCT FROM NEW)
        EXECUTE FUNCTION inc_version();', tablename);
end;
$$ language plpgsql;

-- Finally, this is a text collation that sorts text case-insensitively, useful for `UNIQUE` indexes
-- over things like usernames and emails, without needing to remember to do case-conversion.
create collation if not exists case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);
