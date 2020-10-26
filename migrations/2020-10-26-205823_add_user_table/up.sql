CREATE TABLE "auth"."user"
(
    "id"         uuid              NOT NULL DEFAULT uuid_generate_v4(),
    "first_name" character varying NOT NULL,
    "last_name"  character varying NOT NULL,
    "patronymic" character varying NOT NULL,
    "email"      character varying NOT NULL,
    "password"   character varying NOT NULL,
    CONSTRAINT "user_pk" PRIMARY KEY ("id")
);
