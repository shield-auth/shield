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
    max_concurrent_sessions INTEGER,
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
    max_concurrent_sessions INTEGER NOT NULL DEFAULT 1,
    locked_at TIMESTAMP,
    realm_id UUID NOT NULL REFERENCES realm(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_locked_at CHECK (locked_at IS NULL OR locked_at <= CURRENT_TIMESTAMP)
);

CREATE UNIQUE INDEX realm_id_name_key ON client (realm_id, name);
CREATE INDEX idx_client_realm_locked ON client (realm_id, locked_at);

-- Create trigger function to check max_concurrent_sessions constraint
CREATE OR REPLACE FUNCTION check_max_concurrent_sessions()
RETURNS TRIGGER AS $$
DECLARE
    realm_max_sessions INTEGER;
    current_total_sessions INTEGER;
BEGIN
    -- Fetch the max_concurrent_sessions for the realm
    SELECT max_concurrent_sessions INTO realm_max_sessions
    FROM realm
    WHERE id = NEW.realm_id;

    -- Only perform the check if the realm has a max_concurrent_sessions limit set
    IF realm_max_sessions IS NOT NULL THEN
        -- Calculate the total max_concurrent_sessions for all clients in this realm, including the new or updated client
        SELECT COALESCE(SUM(max_concurrent_sessions), 0) INTO current_total_sessions
        FROM client
        WHERE realm_id = NEW.realm_id
        AND id <> NEW.id;  -- Exclude the current client during an update

        -- Add the new client's max_concurrent_sessions to the total
        current_total_sessions := current_total_sessions + NEW.max_concurrent_sessions;

        -- Check if the total exceeds the realm's max_concurrent_sessions
        IF current_total_sessions > realm_max_sessions THEN
            RAISE EXCEPTION 'Total max_concurrent_sessions for all clients in this realm (%s) exceeds the realm''s limit (%s)',
            current_total_sessions, realm_max_sessions;
        END IF;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger for enforcing the constraint on insert and update
CREATE TRIGGER enforce_max_concurrent_sessions
BEFORE INSERT OR UPDATE ON client
FOR EACH ROW
EXECUTE FUNCTION check_max_concurrent_sessions();


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
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_email_format CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$'),
    CONSTRAINT chk_locked_at CHECK (locked_at IS NULL OR locked_at <= CURRENT_TIMESTAMP),
    CONSTRAINT chk_email_verified_at CHECK (email_verified_at IS NULL OR email_verified_at >= created_at AND email_verified_at <= CURRENT_TIMESTAMP),
    CONSTRAINT chk_phone_format CHECK (phone ~ '^\+?[0-9]{10,14}$')
);

CREATE UNIQUE INDEX realm_email_idx ON "user" (realm_id, email);
CREATE INDEX realm_email_locked_at_idx ON "user" (realm_id, email, locked_at);
CREATE INDEX idx_user_name ON "user" (realm_id, first_name, last_name);

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
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_locked_at CHECK (locked_at IS NULL OR locked_at <= CURRENT_TIMESTAMP)
);

CREATE UNIQUE INDEX realm_client_user_resource_group_idx ON resource_group (name, client_id, user_id);
CREATE INDEX client_user_default_resource_group_idx ON resource_group (client_id, user_id, is_default) WHERE is_default = true;

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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    client_id UUID NOT NULL REFERENCES client(id) ON DELETE CASCADE,
    ip_address INET NOT NULL,
    user_agent TEXT,
    browser VARCHAR(255),
    browser_version VARCHAR(100),
    operating_system VARCHAR(255),
    device_type VARCHAR(50),
    country_code CHAR(2),
    expires TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_session_user_id ON session (user_id, expires);
CREATE INDEX idx_session_client_id ON session (client_id, expires);
CREATE INDEX idx_session_expires ON session (expires);

CREATE OR REPLACE FUNCTION cleanup_expired_sessions()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM session
    WHERE expires < CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER session_cleanup_trigger
AFTER INSERT OR UPDATE ON session
FOR EACH STATEMENT
EXECUTE FUNCTION cleanup_expired_sessions();

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