-- Tabela de usuários autenticados
CREATE TABLE authenticated_users (
    id TEXT PRIMARY KEY,              -- Google sub (unique ID)
    email TEXT UNIQUE NOT NULL,
    name TEXT,
    picture_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP
);

-- Tabela de dispositivos
CREATE TABLE devices (
    id TEXT PRIMARY KEY,              -- UUID ou MAC address
    user_id TEXT NOT NULL,
    device_name TEXT NOT NULL,        -- "PC de João", "Samsung Galaxy A54"
    device_type TEXT NOT NULL,        -- "pc", "android", "ios"
    os_info TEXT,                     -- Sistema operacional
    mac_address TEXT,
    ip_address TEXT,
    last_seen TIMESTAMP,
    is_trusted BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES authenticated_users(id),
    UNIQUE(user_id, mac_address)
);

-- Tabela de contatos/dispositivos conhecidos
CREATE TABLE device_contacts (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    contact_name TEXT NOT NULL,       -- Nome do contato (ex: "Meu Celular")
    device_id TEXT NOT NULL,
    is_favorite BOOLEAN DEFAULT FALSE,
    last_shared TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES authenticated_users(id),
    FOREIGN KEY (device_id) REFERENCES devices(id),
    UNIQUE(user_id, device_id)
);

-- Tabela de chaves criptográficas para autenticação entre dispositivos
CREATE TABLE device_keys (
    id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL,
    public_key TEXT NOT NULL,         -- Chave pública (RSA/Ed25519)
    private_key_encrypted TEXT,       -- Chave privada criptografada
    key_algorithm TEXT DEFAULT 'ed25519',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (device_id) REFERENCES devices(id)
);

-- Índices para performance
CREATE INDEX idx_user_devices ON devices(user_id);
CREATE INDEX idx_user_contacts ON device_contacts(user_id);
CREATE INDEX idx_device_mac ON devices(mac_address);
