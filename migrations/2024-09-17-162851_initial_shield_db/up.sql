-- Create realms table
CREATE TABLE realm (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    slug TEXT NOT NULL UNIQUE, -- TODO: this needs to be auto generated from name.
    locked_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)

-- Create clients table
CREATE TABLE client (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    two_factor_enabled_at TIMESTAMP, -- if enabled at client level then it will be mandatory for tenant/user 
    locked_at TIMESTAMP,
    realm_id SERIAL REFERENCES realm(id) on DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX realm_id_name_key ON client (realm_id, name);

-- Create users table
CREATE TABLE "user" (
    id TEXT PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT,
    email TEXT NOT NULL,
    email_verified_at TIMESTAMP,
    image TEXT,
    two_factor_enabled_at TIMESTAMP, -- User can opt-in even in case it is not enabled on client level
    password TEXT,
    is_temp_password BOOLEAN DEFAULT TRUE,
    locked_at TIMESTAMP,
    client_id SERIAL REFERENCES client(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for users table
CREATE UNIQUE INDEX client_id_email_idx ON "user" (client_id, email);
CREATE INDEX client_id_email_locked_at_idx ON "user" (client_id, email, locked_at);

-----------------------------------------------------------------

-- Create resources table
CREATE TABLE resource (
    id SERIAL PRIMARY KEY,
    client_id SERIAL REFERENCES "client"(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    value TEXT NOT NULL,
    is_default BOOLEAN DEFAULT FALSE,
    locked_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create index for resources table
CREATE INDEX client_id_user_id_is_default_idx ON resource (client_id, user_id, is_default);
CREATE UNIQUE INDEX client_id_user_id_value_idx ON resource (client_id, user_id, value);

-- Add constraint to ensure only one default resource per user
ALTER TABLE resource ADD CONSTRAINT unique_default_resource_per_user 
    EXCLUDE USING btree (user_id WITH =) 
    WHERE (is_default = true);

-- Function to update is_default flag
CREATE OR REPLACE FUNCTION update_default_resource()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.is_default THEN
        UPDATE resource
        SET is_default = FALSE
        WHERE user_id = NEW.user_id AND id != NEW.id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to maintain single default resource per user
CREATE TRIGGER maintain_single_default_resource
BEFORE INSERT OR UPDATE ON resource
FOR EACH ROW
EXECUTE FUNCTION update_default_resource();
-----------------------------------------------------------------

-- Create accounts table
CREATE TABLE account (
    user_id TEXT NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    type TEXT NOT NULL,
    provider TEXT NOT NULL,
    provider_account_id TEXT NOT NULL,
    refresh_token TEXT,
    access_token TEXT,
    expires_at INTEGER,
    token_type TEXT,
    scope TEXT,
    id_token TEXT,
    session_state TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (provider, provider_account_id)
);

-- Create sessions table
CREATE TABLE session (
    session_token TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    expires TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create verification_token table
CREATE TABLE verification_token (
    identifier TEXT NOT NULL,
    token TEXT NOT NULL,
    expires TIMESTAMP NOT NULL,
    PRIMARY KEY (identifier, token)
);

-- Create password_reset_token table
CREATE TABLE password_reset_token (
    identifier TEXT NOT NULL,
    token TEXT NOT NULL,
    expires TIMESTAMP NOT NULL,
    PRIMARY KEY (identifier, token)
);

-- Create two_factor_token table
CREATE TABLE two_factor_token (
    identifier TEXT NOT NULL,
    token TEXT NOT NULL,
    expires TIMESTAMP NOT NULL,
    PRIMARY KEY (identifier, token)
);

-- Create two_factor_confirmation table
CREATE TABLE two_factor_confirmation (
    identifier TEXT NOT NULL,
    token TEXT NOT NULL,
    expires TIMESTAMP NOT NULL,
    PRIMARY KEY (identifier, token)
);

-- Create authenticators table
CREATE TABLE authenticator (
    credential_id TEXT NOT NULL UNIQUE,
    user_id TEXT NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    provider_account_id TEXT NOT NULL,
    credential_public_key TEXT NOT NULL,
    counter INTEGER NOT NULL,
    credential_device_type TEXT NOT NULL,
    credential_backed_up BOOLEAN NOT NULL,
    transports TEXT,
    PRIMARY KEY (user_id, credential_id)
);
