CREATE TABLE pages (
  id SERIAL PRIMARY KEY,
  slug VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  UNIQUE (slug)
)
