-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS set_updated_at ON todos;
DROP FUNCTION IF EXISTS update_updated_at_column;
