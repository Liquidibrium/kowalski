use crate::embeddings::transformer::{EmbeddingModelConfig, get_embeddings, load_model_bert_model, ModelAndTokenizer};

pub trait EmbeddingCalculator {
    fn new(model: &String) -> Self;
    fn get_embedding(&mut self, code: &str) -> Vec<f32>;
}

pub struct EmbeddingModelLocal {
    model_name: String,
    model: ModelAndTokenizer,
}

impl EmbeddingCalculator for EmbeddingModelLocal {
    fn new(model: &String) -> Self {
       // let config = EmbeddingModelConfig{
       //      model_repo: "BAAI/bge-small-en-v1.5".to_string(),
       //      config_file: "config.json".to_string(),
       //      tokenizer_file: "tokenizer.json".to_string(),
       //      weights_file: "pytorch_model.bin".to_string()
       // };
        let config = EmbeddingModelConfig{
            model_repo: "Salesforce/codet5p-110m-embedding".to_string(),
            config_file: "config.json".to_string(),
            tokenizer_file: "tokenizer.json".to_string(),
            weights_file: "pytorch_model.bin".to_string()
        };

        EmbeddingModelLocal {
            model_name: model.clone(),
            model: load_model_bert_model(&config).expect("Failed to load model"),
        }
    }

    fn get_embedding(&mut self, code: &str) -> Vec<f32> {
        println!("Getting embeddings for text: {}", code.len());
        return get_embeddings(code, &mut self.model.model, &self.model.tokenizer)
            .expect("Failed to get embeddings");
    }
}
