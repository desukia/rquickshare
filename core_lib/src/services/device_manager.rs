use crate::models::device::{DeviceInfo, DeviceContact, DeviceType};
use sqlx::SqlitePool;
use chrono::Utc;

pub struct DeviceManager {
    db: SqlitePool,
}

impl DeviceManager {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    /// Registrar novo dispositivo para o usuário autenticado
    pub async fn register_device(&self, user_id: &str, device: DeviceInfo) -> Result<DeviceInfo, Box<dyn std::error::Error>> {
        sqlx::query(
            "INSERT INTO devices (id, user_id, device_name, device_type, os_info, mac_address, ip_address, last_seen, is_trusted)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&device.id)
        .bind(user_id)
        .bind(&device.device_name)
        .bind(format!("{:?}", device.device_type).to_lowercase())
        .bind(&device.os_info)
        .bind(&device.mac_address)
        .bind(&device.ip_address)
        .bind(device.last_seen)
        .bind(device.is_trusted)
        .execute(&self.db)
        .await?;

        Ok(device)
    }

    /// Listar todos os dispositivos do usuário
    pub async fn list_user_devices(&self, user_id: &str) -> Result<Vec<DeviceInfo>, Box<dyn std::error::Error>> {
        let devices = sqlx::query_as::<_, DeviceInfo>(
            "SELECT * FROM devices WHERE user_id = ? ORDER BY last_seen DESC"
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await?;

        Ok(devices)
    }

    /// Obter contatos de dispositivos (como aparecem no outro aparelho)
    pub async fn get_device_contacts(&self, user_id: &str) -> Result<Vec<DeviceContact>, Box<dyn std::error::Error>> {
        let contacts = sqlx::query_as::<_, DeviceContact>(
            "SELECT dc.*, d.* FROM device_contacts dc
             JOIN devices d ON dc.device_id = d.id
             WHERE dc.user_id = ? ORDER BY dc.is_favorite DESC, dc.created_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await?;

        Ok(contacts)
    }

    /// Marcar dispositivo como confiável
    pub async fn trust_device(&self, user_id: &str, device_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("UPDATE devices SET is_trusted = TRUE WHERE id = ? AND user_id = ?")
            .bind(device_id)
            .bind(user_id)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    /// Criar contato para um dispositivo descoberto
    pub async fn create_device_contact(
        &self,
        user_id: &str,
        device_id: &str,
        contact_name: &str,
    ) -> Result<DeviceContact, Box<dyn std::error::Error>> {
        let contact_id = uuid::Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO device_contacts (id, user_id, contact_name, device_id, created_at)
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&contact_id)
        .bind(user_id)
        .bind(contact_name)
        .bind(device_id)
        .bind(Utc::now())
        .execute(&self.db)
        .await?;

        // Retornar contato completo
        let contact = sqlx::query_as::<_, DeviceContact>(
            "SELECT * FROM device_contacts WHERE id = ?"
        )
        .bind(&contact_id)
        .fetch_one(&self.db)
        .await?;

        Ok(contact)
    }

    /// Atualizar último acesso ao dispositivo
    pub async fn update_device_last_seen(&self, device_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query("UPDATE devices SET last_seen = ? WHERE id = ?")
            .bind(Utc::now())
            .bind(device_id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
