CREATE TABLE IF NOT EXISTS todos (
  id          INTEGER PRIMARY KEY NOT NULL,
  description TEXT                NOT NULL,
  done        BOOLEAN             NOT NULL DEFAULT 0
);

/*
Business Tables Below
*/

CREATE TABLE IF NOT EXISTS users (
  id       TEXT PRIMARY KEY NOT NULL,
  username TEXT                NOT NULL,
  password TEXT                NOT NULL
);

CREATE TABLE IF NOT EXISTS stores (
  id          TEXT PRIMARY KEY NOT NULL,
  name        TEXT                NOT NULL,
  description TEXT                NOT NULL,
  owner_user_id     TEXT             NOT NULL,
  FOREIGN KEY (owner_user_id) REFERENCES users (id)
);