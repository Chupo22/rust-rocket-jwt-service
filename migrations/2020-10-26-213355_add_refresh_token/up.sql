CREATE TABLE "auth"."refresh_token"
(
    "id"            uuid NOT NULL DEFAULT public.uuid_generate_v4(),
    "user_id"       uuid NOT NULL,
    "token" uuid NOT NULL,
    CONSTRAINT "pk_refresh_token" PRIMARY KEY ("id"),
    CONSTRAINT "fk_refresh_token_x_user"
        FOREIGN KEY ("user_id")
        REFERENCES "auth"."user" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION
);
