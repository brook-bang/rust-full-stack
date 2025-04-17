drop table if exists ezy_course_c4;

create table ezy_course_c4
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    posted_time TIMESTAMP default now()
);

insert into ezy_course_c4
(
    course_id,
    tutor_id,
    course_name,
    posted_time
)
values
(
    1,
    1,
    'First Course',
    '2022-01-01 11:00:00'
);
insert into ezy_course_c4
(
    course_id,
    tutor_id,
    course_name,
    posted_time
)
values
(
    2,
    1,
    'Second Course',
    '2022-01-01 11:00:01'
);

