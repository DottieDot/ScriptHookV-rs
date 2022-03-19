use async_trait::async_trait;

#[async_trait]
pub trait Script {
  async fn start(&mut self);
  async fn update(&mut self);
  async fn cleanup(&mut self);
}
