DROP FUNCTION IF EXISTS upsert_user(VARCHAR(255), VARCHAR(255), TEXT, VARCHAR(255), VARCHAR(255), INT);
CREATE OR REPLACE FUNCTION upsert_user(
    p_name VARCHAR(255),
    p_user_id VARCHAR(255),
    p_scopes TEXT,
    p_refresh_token VARCHAR(255),
    p_access_token VARCHAR(255),
    p_expires_in INT
)
RETURNS TABLE (
    old_name VARCHAR(255),
    old_scopes TEXT,
    old_refresh_token VARCHAR(255),
    old_access_token VARCHAR(255),
    old_expires_in INT,
    old_created_at TIMESTAMP,
    old_updated_at TIMESTAMP
) AS $$
BEGIN
    RETURN QUERY
    SELECT name, scopes, refresh_token, access_token, expires_in, created_at, updated_at
    FROM users
    WHERE user_id = p_user_id;
    
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
        
    RETURN;
END;
$$ LANGUAGE plpgsql;
