CREATE TABLE task
(
    id          bigint GENERATED ALWAYS AS IDENTITY,
    task_id     uuid DEFAULT gen_random_uuid() unique not null,
    description text                                  not null
);