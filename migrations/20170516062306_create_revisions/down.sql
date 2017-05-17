ALTER TABLE pages ADD COLUMN body TEXT;
UPDATE pages SET body = revisions.body FROM revisions WHERE pages.id = revisions.page_id;
ALTER TABLE pages ALTER COLUMN body SET NOT NULL;

DROP TABLE revisions;
