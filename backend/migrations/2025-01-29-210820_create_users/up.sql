CREATE TABLE "users" (
  "id" uuid PRIMARY KEY NOT NULL,
  "name" varchar(32) UNIQUE NOT NULL,
  "password" text NOT NULL,
  "salt" text NOT NULL,
  "elevated" bool NOT NULL
);
