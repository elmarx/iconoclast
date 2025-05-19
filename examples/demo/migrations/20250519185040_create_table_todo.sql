CREATE TABLE todo
(
    id          bigint GENERATED ALWAYS AS IDENTITY,
    description text NOT NULL
)