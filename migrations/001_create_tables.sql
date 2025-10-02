CREATE TABLE "blogs" (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  "title" VARCHAR NOT NULL,
  "published" BOOLEAN DEFAULT false
);


CREATE TABLE "users" (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  "email" VARCHAR NOT NULL,
  "password_hash" VARCHAR NOT NULL,
  "name" VARCHAR
);


CREATE TABLE "profiles" (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  "user_id" UUID NOT NULL,
  "bio" TEXT,
  "avatar_url" VARCHAR
);

CREATE INDEX idx_profiles_user_id ON "profiles" ("user_id");

CREATE TABLE "sessions" (
  "id" UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  "user_id" UUID NOT NULL,
  "expires_at" TIMESTAMP NOT NULL
);

CREATE INDEX idx_sessions_user_id ON "sessions" ("user_id");