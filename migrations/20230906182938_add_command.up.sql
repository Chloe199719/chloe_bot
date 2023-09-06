-- Add up migration script here
CREATE Permission as ENUM ('Broadcaster', 'Super Moderator', 'Moderator', 'VIP', 'Subscriber', 'Everyone');

CREATE TABLE IF NOT EXISTS channel_commands(
    id SERIAL PRIMARY KEY,
    channel_id INT NOT NULL REFERENCES channels(user_id),
    PING BOOLEAN NOT NULL DEFAULT TRUE,
    PINGPERMS Permission NOT NULL DEFAULT 'Everyone',
    PINGCOOLDOWN INT NOT NULL DEFAULT 0,
    BALL8 BOOLEAN NOT NULL DEFAULT TRUE,
    BALL8PERMS Permission NOT NULL DEFAULT 'Everyone',
    BALL8COOLDOWN INT NOT NULL DEFAULT 0,
    BAN BOOLEAN NOT NULL DEFAULT TRUE,
    BANPERMS Permission NOT NULL DEFAULT 'Moderator',
    BANCOOLDOWN INT NOT NULL DEFAULT 0,
    UNBAN BOOLEAN NOT NULL DEFAULT TRUE,
    UNBANPERMS Permission NOT NULL DEFAULT 'Moderator',
    UNBANCOOLDOWN INT NOT NULL DEFAULT 0,
    KICK BOOLEAN NOT NULL DEFAULT TRUE,
    KICKPERMS Permission NOT NULL DEFAULT 'Moderator',
    KICKCOOLDOWN INT NOT NULL DEFAULT 0,
    COMMANDS BOOLEAN NOT NULL DEFAULT TRUE,
    COMMANDSPERMS Permission NOT NULL DEFAULT 'Everyone',
    COMMANDSCOOLDOWN INT NOT NULL DEFAULT 0,
    ADDCOMMAND BOOLEAN NOT NULL DEFAULT TRUE,
    ADDCOMMANDPERMS Permission NOT NULL DEFAULT 'Moderator',
    ADDCOMMANDCOOLDOWN INT NOT NULL DEFAULT 0,
    REMOVECOMMAND BOOLEAN NOT NULL DEFAULT TRUE,
    REMOVECOMMANDPERMS Permission NOT NULL DEFAULT 'Moderator',
    REMOVECOMMANDCOOLDOWN INT NOT NULL DEFAULT 0,
    FOLLOWAGE BOOLEAN NOT NULL DEFAULT TRUE,
    FOLLOWAGEPERMS Permission NOT NULL DEFAULT 'Everyone',
    FOLLOWAGECOOLDOWN INT NOT NULL DEFAULT 0,
    UPTIME BOOLEAN NOT NULL DEFAULT TRUE,
    UPTIMEPERMS Permission NOT NULL DEFAULT 'Everyone',
    UPTIMECOOLDOWN INT NOT NULL DEFAULT 0,
    CATEGORY BOOLEAN NOT NULL DEFAULT TRUE,
    CATEGORYPERMS Permission NOT NULL DEFAULT 'Everyone',
    CATEGORYCOOLDOWN INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)