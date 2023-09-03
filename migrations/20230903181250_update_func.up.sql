-- Add up migration script here
DROP FUNCTION IF EXISTS upsert_user(VARCHAR(255), VARCHAR(255), TEXT, VARCHAR(255), VARCHAR(255), INT);

CREATE TYPE upsert_result AS (
    success BOOLEAN,
    old_name VARCHAR(255),
    old_scopes TEXT,
    old_refresh_token VARCHAR(255),
    old_access_token VARCHAR(255),
    old_expires_in INT,
    old_created_at TIMESTAMP,
    old_updated_at TIMESTAMP
);

CREATE OR REPLACE FUNCTION upsert_user(
    p_name VARCHAR(255),
    p_user_id VARCHAR(255),
    p_scopes TEXT,
    p_refresh_token VARCHAR(255),
    p_access_token VARCHAR(255),
    p_expires_in INT
)
RETURNS upsert_result AS $$
DECLARE
    v_result upsert_result;
BEGIN
    -- Try fetching old values
    SELECT INTO v_result.old_name, v_result.old_scopes, v_result.old_refresh_token,
               v_result.old_access_token, v_result.old_expires_in, 
               v_result.old_created_at, v_result.old_updated_at
    name, scopes, refresh_token, access_token, expires_in, created_at, updated_at
    FROM users
    WHERE user_id = p_user_id;
    
    -- Perform the upsert operation
    INSERT INTO users (name, user_id, scopes, refresh_token, access_token, expires_in)
    VALUES (p_name, p_user_id, p_scopes, p_refresh_token, p_access_token, p_expires_in)
    ON CONFLICT (user_id) 
    DO UPDATE SET 
        name = EXCLUDED.name,
        scopes = EXCLUDED.scopes,
        refresh_token = EXCLUDED.refresh_token,
        access_token = EXCLUDED.access_token,
        expires_in = EXCLUDED.expires_in,
        updated_at = CURRENT_TIMESTAMP;
    
    v_result.success = true;
    RETURN v_result;
EXCEPTION
    WHEN OTHERS THEN
        v_result.success = false;
        RETURN v_result;
END;
$$ LANGUAGE plpgsql;
