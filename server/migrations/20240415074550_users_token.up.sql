ALTER TABLE users
ADD COLUMN token VARCHAR(36) AFTER password,
ADD UNIQUE (token);
