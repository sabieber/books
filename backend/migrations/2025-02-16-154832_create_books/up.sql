CREATE TABLE "books" (
  "id" uuid PRIMARY KEY NOT NULL,
  "user" uuid NOT NULL REFERENCES "users" ("id"),
  "shelf" uuid NOT NULL REFERENCES "shelves" ("id"),
  "title" text,
  "author" text,
  "isbn13" text,
  "isbn10" text,
  "google_books_id" text,
  "added_at" TIMESTAMPTZ NOT NULL DEFAULT now()
);
