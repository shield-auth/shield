-- down.sql
-- Drop triggers first
DROP TRIGGER IF EXISTS ensure_single_default_resource_group ON resource_group;
DROP TRIGGER IF EXISTS before_insert_and_update_realm ON realm;
DROP TRIGGER IF EXISTS session_cleanup_trigger ON session;
DROP TRIGGER IF EXISTS enforce_max_concurrent_sessions ON client;


-- Drop functions
DROP FUNCTION IF EXISTS manage_default_resource_group();
DROP FUNCTION IF EXISTS generate_slug();
DROP FUNCTION IF EXISTS cleanup_expired_sessions();
DROP FUNCTION IF EXISTS check_max_concurrent_sessions();
DROP FUNCTION IF EXISTS uuid_generate_v7();

-- Drop tables
DROP TABLE IF EXISTS authenticator;
DROP TABLE IF EXISTS two_factor_confirmation;
DROP TABLE IF EXISTS two_factor_token;
DROP TABLE IF EXISTS password_reset_token;
DROP TABLE IF EXISTS verification_token;
DROP TABLE IF EXISTS session;
DROP TABLE IF EXISTS account;
DROP TABLE IF EXISTS resource;
DROP TABLE IF EXISTS resource_group;
DROP TABLE IF EXISTS "user";
DROP TABLE IF EXISTS client;
DROP TABLE IF EXISTS realm;