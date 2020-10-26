CREATE TABLE "auth"."refresh_token"
(
    "id"            uuid NOT NULL DEFAULT uuid_generate_v4(),
    "user_id"       uuid NOT NULL,
    "refresh_token" uuid NOT NULL,
    CONSTRAINT "refresh_token_pk" PRIMARY KEY ("id"),
    CONSTRAINT "refresh_token_x_user_fk"
        FOREIGN KEY ("user_id")
        REFERENCES "auth"."user" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION
);
