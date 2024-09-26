-- down.sql
-- Drop triggers first
DROP TRIGGER IF EXISTS trigger_update_user_realm_locked_at ON realm;
DROP TRIGGER IF EXISTS ensure_single_default_resource_group ON resource_group;
DROP TRIGGER IF EXISTS trigger_update_resource_group_client_locked_at ON client;
DROP TRIGGER IF EXISTS before_insert_and_update_realm ON realm;
DROP TRIGGER IF EXISTS trigger_update_resource_locked_at ON resource_group;

-- Drop functions
DROP FUNCTION IF EXISTS update_user_realm_locked_at();
DROP FUNCTION IF EXISTS manage_default_resource_group();
DROP FUNCTION IF EXISTS update_resource_group_client_locked_at();
DROP FUNCTION IF EXISTS generate_slug();
DROP FUNCTION IF EXISTS uuid_generate_v7();
DROP FUNCTION IF EXISTS update_resource_locked_at();

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