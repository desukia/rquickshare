#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticatedShareRequest {
    pub request_id: String,
    pub from_device: AuthenticatedDevice,
    pub auth_token: String,        // JWT assinado
    pub timestamp: i64,
    pub files: Vec<FileMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticatedDevice {
    pub device_id: String,
    pub device_name: String,
    pub public_key: String,       // Para verificar assinatura
    pub is_trusted: bool,
}

/// Validar requisição autenticada
pub fn validate_authenticated_request(
    request: &AuthenticatedShareRequest,
    expected_signature: &str,
) -> Result<bool, String> {
    // Implementar verificação de assinatura usando a chave pública do dispositivo
    use ring::signature;
    use ring::primitives::decode_base64;

    let public_key_bytes = decode_base64(&request.from_device.public_key)
        .map_err(|_| "Invalid public key encoding")?;
    
    let public_key = signature::UnparsedPublicKey::new(
        &signature::ED25519,
        &public_key_bytes,
    );

    let message = format!(
        "{}{}{}",
        request.request_id,
        request.from_device.device_id,
        request.timestamp
    );

    public_key
        .verify(message.as_bytes(), decode_base64(expected_signature)?.as_ref())
        .map_err(|_| "Signature verification failed".to_string())?;

    Ok(true)
}
