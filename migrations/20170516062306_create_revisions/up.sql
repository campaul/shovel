CREATE TABLE revisions (
  id SERIAL PRIMARY KEY,
  page_id int4 REFERENCES pages(id),
  body TEXT NOT NULL,
  version int4 DEFAULT 1,
  UNIQUE(page_id, version)
);
INSERT INTO revisions (page_id, body) SELECT id, body from pages;

ALTER TABLE pages DROP COLUMN body;
