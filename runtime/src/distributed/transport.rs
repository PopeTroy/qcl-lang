use anyhow::Result;

pub struct UCXTransport;

impl UCXTransport {
    pub fn send_bytes(&self, target_endpoint: &str, payload: &[u8]) -> Result<()> {
        tracing::info!("Sending {} bytes to {}", payload.len(), target_endpoint);
        Ok(())
    }
}
