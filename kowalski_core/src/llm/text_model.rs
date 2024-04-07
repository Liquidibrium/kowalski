pub trait TextModelApiAsync {
    async fn complete(&self, prompt: &str) -> anyhow::Result<String>;
    async fn complete_batch(&self, prompts: &Vec<&str>) -> anyhow::Result<Vec<String>>;
    async fn complete_streaming(&self, prompt: &str) -> anyhow::Result<String>;
}
