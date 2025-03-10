CREATE SCHEMA IF NOT EXISTS "learning";


CREATE TABLE IF NOT EXISTS "learning"."tbl_users" (
    pk_user_id BIGINT PRIMARY KEY NOT NULL,
    username VARCHAR(150) NOT NULL
);

