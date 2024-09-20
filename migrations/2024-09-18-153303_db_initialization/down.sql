-- Drop authenticators table
DROP TABLE IF EXISTS authenticator;

-- Drop two_factor_confirmation table
DROP TABLE IF EXISTS two_factor_confirmation;

-- Drop two_factor_token table
DROP TABLE IF EXISTS two_factor_token;

-- Drop password_reset_token table
DROP TABLE IF EXISTS password_reset_token;

-- Drop verification_token table
DROP TABLE IF EXISTS verification_token;

-- Drop sessions table
DROP TABLE IF EXISTS session;

-- Drop accounts table
DROP TABLE IF EXISTS account;

-- Drop resources table
DROP TABLE IF EXISTS resource;

-- Drop resource_groups table
DROP TABLE IF EXISTS resource_group;

-- Drop users table
DROP TABLE IF EXISTS "user";

-- Drop clients table
DROP TABLE IF EXISTS client;

-- Drop realms table
DROP TABLE IF EXISTS realm;

-- Drop functions and triggers
DROP FUNCTION IF EXISTS uuid_generate_v7();
DROP TRIGGER IF EXISTS ensure_single_default_resource_group ON resource_group;
DROP FUNCTION IF EXISTS manage_default_resource_group();
DROP TRIGGER IF EXISTS before_insert_and_update_realm ON realm;
DROP FUNCTION IF EXISTS generate_slug();