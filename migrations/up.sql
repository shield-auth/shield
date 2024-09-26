CREATE OR REPLACE FUNCTION uuid_generate_v7 () RETURNS uuid AS $$
BEGIN
  -- use random v4 uuid as starting point (which has the same variant we need)
  -- then overlay timestamp
  -- then set version 7 by flipping the 2 and 1 bit in the version 4 string
  return encode(
    set_bit(
      set_bit(
        overlay(uuid_send(gen_random_uuid())
                placing substring(int8send(floor(extract(epoch from clock_timestamp()) * 1000)::bigint) from 3)
                from 1 for 6
        ),
        52, 1
      ),
      53, 1
    ),
    'hex')::uuid;
END
$$ LANGUAGE plpgsql volatile;

-- Create realms table
CREATE TABLE realm (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    name TEXT NOT NULL UNIQUE,
    slug TEXT NOT NULL UNIQUE,
    locked_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Add trigger for auto-generating slug
CREATE OR REPLACE FUNCTION generate_slug()
RETURNS TRIGGER AS $$
BEGIN
    NEW.slug := LOWER(REGEXP_REPLACE(NEW.name, '[^a-zA-Z0-9]+', '-', 'g'));
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER before_insert_and_update_realm
BEFORE INSERT OR UPDATE ON realm
FOR EACH ROW
EXECUTE FUNCTION generate_slug();

-- Create clients table
CREATE TABLE client (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    name TEXT NOT NULL,
    two_factor_enabled_at TIMESTAMP,
    locked_at TIMESTAMP,
    realm_id UUID NOT NULL REFERENCES realm(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_locked_at CHECK (locked_at IS NULL OR locked_at <= CURRENT_TIMESTAMP)
);

CREATE UNIQUE INDEX realm_id_name_key ON client (realm_id, name);
CREATE INDEX idx_client_realm_locked ON client (realm_id, locked_at);

-- Create users table
CREATE TABLE "user" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    first_name TEXT NOT NULL,
    last_name TEXT,
    email TEXT NOT NULL,
    email_verified_at TIMESTAMP,
    phone TEXT,
    image TEXT,
    two_factor_enabled_at TIMESTAMP,
    password_hash TEXT,
    is_temp_password BOOLEAN DEFAULT TRUE,
    locked_at TIMESTAMP,
    realm_id UUID NOT NULL REFERENCES realm(id) ON DELETE CASCADE,
    realm_locked_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_email_format CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$'),
    CONSTRAINT chk_locked_at CHECK (locked_at IS NULL OR locked_at <= CURRENT_TIMESTAMP),
    CONSTRAINT chk_email_verified_at CHECK (email_verified_at IS NULL OR email_verified_at >= created_at AND email_verified_at <= CURRENT_TIMESTAMP),
    CONSTRAINT chk_phone_format CHECK (phone ~ '^\+?[0-9]{10,14}$'),
    CONSTRAINT chk_realm_locked_at CHECK (realm_locked_at IS NULL OR realm_locked_at <= CURRENT_TIMESTAMP)
);

CREATE UNIQUE INDEX realm_email_idx ON "user" (realm_id, email);
CREATE INDEX realm_email_locked_at_idx ON "user" (realm_id, email, locked_at) WHERE locked_at IS NULL;
CREATE INDEX idx_user_name ON "user" (realm_id, first_name, last_name);
CREATE INDEX idx_user_realm_locked_at ON "user" (realm_id, realm_locked_at) WHERE realm_locked_at IS NULL;

-- Trigger to update realm_locked_at
CREATE OR REPLACE FUNCTION update_user_realm_locked_at()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE "user"
    SET realm_locked_at = NEW.locked_at
    WHERE realm_id = NEW.id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_user_realm_locked_at
AFTER UPDATE OF locked_at ON realm
FOR EACH ROW
WHEN (OLD.locked_at IS DISTINCT FROM NEW.locked_at)
EXECUTE FUNCTION update_user_realm_locked_at();

-----------------------------------------------------------
-- Create resources_groups table
CREATE TABLE resource_group (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    realm_id UUID NOT NULL REFERENCES realm(id) ON DELETE CASCADE,
    client_id UUID NOT NULL REFERENCES client(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    is_default BOOLEAN DEFAULT FALSE,
    locked_at TIMESTAMP,
    client_locked_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_locked_at CHECK (locked_at IS NULL OR locked_at <= CURRENT_TIMESTAMP),
    CONSTRAINT chk_client_locked_at CHECK (client_locked_at IS NULL OR client_locked_at <= CURRENT_TIMESTAMP) 
);

CREATE UNIQUE INDEX realm_client_user_resource_group_idx ON resource_group (realm_id, client_id, user_id, name);
CREATE INDEX client_user_default_resource_group_idx ON resource_group (client_id, user_id) WHERE is_default = true;
CREATE INDEX idx_resource_group_client_locked_at ON resource_group (client_id, client_locked_at) WHERE client_locked_at IS NULL;
CREATE UNIQUE INDEX idx_single_default_resource_group ON resource_group (client_id, user_id) WHERE is_default = true;

-- Trigger to update client_locked_at
CREATE OR REPLACE FUNCTION update_resource_group_client_locked_at()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE resource_group
    SET client_locked_at = NEW.locked_at
    WHERE client_id = NEW.id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_resource_group_client_locked_at
AFTER UPDATE OF locked_at ON client
FOR EACH ROW
WHEN (OLD.locked_at IS DISTINCT FROM NEW.locked_at)
EXECUTE FUNCTION update_resource_group_client_locked_at();

-- Function to manage default resource group
CREATE OR REPLACE FUNCTION manage_default_resource_group()
RETURNS TRIGGER AS $$
BEGIN
    -- If the new row is being set as default
    IF NEW.is_default THEN
        -- Set all other resource groups for the same user and client to non-default
        UPDATE resource_group
        SET is_default = FALSE
        WHERE user_id = NEW.user_id
          AND client_id = NEW.client_id
          AND id != NEW.id;
    ELSE
        -- Check if this was the only default group
        IF NOT EXISTS (
            SELECT 1
            FROM resource_group
            WHERE user_id = NEW.user_id
              AND client_id = NEW.client_id
              AND is_default = TRUE
              AND id != NEW.id
        ) THEN
            -- If so, force this group to be default
            NEW.is_default := TRUE;
        END IF;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to execute the function
CREATE TRIGGER ensure_single_default_resource_group
BEFORE INSERT OR UPDATE OF is_default ON resource_group
FOR EACH ROW
EXECUTE FUNCTION manage_default_resource_group();
-----------------------------------------------------------

-- Create resources table
CREATE TABLE resource (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    group_id UUID NOT NULL REFERENCES resource_group(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    value TEXT NOT NULL,
    description TEXT,
    locked_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_locked_at CHECK (locked_at IS NULL OR locked_at <= CURRENT_TIMESTAMP)
);

CREATE UNIQUE INDEX resource_group_and_resource_idx ON resource (group_id, name);

-- Add a trigger to update resource.locked_at when resource_group.locked_at changes
CREATE OR REPLACE FUNCTION update_resource_locked_at()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE resource
    SET locked_at = NEW.locked_at
    WHERE group_id = NEW.id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_resource_locked_at
AFTER UPDATE OF locked_at ON resource_group
FOR EACH ROW
WHEN (OLD.locked_at IS DISTINCT FROM NEW.locked_at)
EXECUTE FUNCTION update_resource_locked_at();

-- Create accounts table
CREATE TABLE account (
    user_id UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
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
    user_id UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
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
    user_id UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    provider_account_id TEXT NOT NULL,
    credential_public_key TEXT NOT NULL,
    counter INTEGER NOT NULL,
    credential_device_type TEXT NOT NULL,
    credential_backed_up BOOLEAN NOT NULL,
    transports TEXT,
    PRIMARY KEY (user_id, credential_id)
);
