CREATE TYPE "reading_mode" AS ENUM ('pages', 'percentage');

CREATE TABLE "readings" (
    "id" uuid PRIMARY KEY NOT NULL,
    "book" uuid NOT NULL REFERENCES "books" ("id"),
    "user" uuid NOT NULL REFERENCES "users" ("id"),
    "total_pages" INTEGER NOT NULL,
    "progress" INTEGER NOT NULL,
    "mode" "reading_mode" NOT NULL DEFAULT 'pages',
    "started_at" DATE NOT NULL DEFAULT now(),
    "finished_at" DATE,
    "cancelled_at" DATE,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE "reading_entries" (
    "id" uuid PRIMARY KEY NOT NULL,
    "reading" uuid NOT NULL REFERENCES "readings" ("id"),
    "book" uuid NOT NULL REFERENCES "books" ("id"),
    "user" uuid NOT NULL REFERENCES "users" ("id"),
    "progress" INTEGER NOT NULL,
    "mode" "reading_mode" NOT NULL DEFAULT 'pages',
    "read_at" DATE NOT NULL DEFAULT now(),
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT now()
);
