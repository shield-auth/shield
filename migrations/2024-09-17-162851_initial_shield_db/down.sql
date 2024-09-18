-- Drop the trigger
DROP TRIGGER IF EXISTS maintain_single_default_resource ON resource;

-- Drop the function
DROP FUNCTION IF EXISTS update_default_resource();

-- Drop the resource table
DROP TABLE IF EXISTS resource;

-- Drop the authenticator table
DROP TABLE IF EXISTS authenticator;

-- Drop the two_factor_confirmation table
DROP TABLE IF EXISTS two_factor_confirmation;

-- Drop the two_factor_token table
DROP TABLE IF EXISTS two_factor_token;

-- Drop the password_reset_token table
DROP TABLE IF EXISTS password_reset_token;

-- Drop the verification_token table
DROP TABLE IF EXISTS verification_token;

-- Drop the session table
DROP TABLE IF EXISTS session;

-- Drop the account table
DROP TABLE IF EXISTS account;

-- Drop the user table
DROP TABLE IF EXISTS "user";

-- Drop the role_type enum
DROP TYPE IF EXISTS role_type;
