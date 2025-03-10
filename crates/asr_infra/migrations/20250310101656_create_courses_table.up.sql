
CREATE TABLE IF NOT EXISTS "learning"."tbl_courses" (
    pk_course_id BIGINT PRIMARY KEY NOT NULL,
    title VARCHAR(150) NOT NULL,
    description TEXT
);