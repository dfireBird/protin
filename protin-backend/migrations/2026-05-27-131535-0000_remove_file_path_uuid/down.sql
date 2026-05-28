-- This file should undo anything in `up.sql`
ALTER TABLE "pastes" ADD COLUMN "file_path" UUID NOT NULL;

