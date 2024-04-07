use anyhow::{Context, Error as E, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
// use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use candle_transformers::models::t5::{Config, T5EncoderModel};
use hf_hub::{api::sync::Api, Repo};
use tokenizers::{PaddingParams, Tokenizer};
const DTYPE: DType = DType::F32;

pub struct ModelAndTokenizer {
    pub model: T5EncoderModel,
    pub tokenizer: Tokenizer,
}

pub struct EmbeddingModelConfig {
    pub model_repo: String,
    pub config_file: String,
    pub tokenizer_file: String,
    pub weights_file: String,
}

pub fn load_model_bert_model(config: &EmbeddingModelConfig) -> Result<ModelAndTokenizer> {
    let api = Api::new()?.repo(Repo::model(config.model_repo.clone()));

    // Fetching the config, tokenizer and weights files
    let config_filename = api.get(config.config_file.as_str())?;
    let tokenizer_filename = api.get(config.tokenizer_file.as_str())?;
    let weights_filename = api.get(config.weights_file.as_str())?;

    let config = std::fs::read_to_string(config_filename)?;
    let config: Config = serde_json::from_str(&config)?;

    let mut tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;
    let vb = VarBuilder::from_pth(&weights_filename, DTYPE, &Device::Cpu)?;
    let model = T5EncoderModel::load(vb, &config)?;

    // Setting the padding strategy for the tokenizer
    if let Some(pp) = tokenizer.get_padding_mut() {
        pp.strategy = tokenizers::PaddingStrategy::BatchLongest
    } else {
        let pp = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            ..Default::default()
        };
        tokenizer.with_padding(Some(pp));
    }

    Ok(ModelAndTokenizer { model, tokenizer })
}

pub fn get_embeddings(
    sentence: &str,
    model: &mut T5EncoderModel,
    tokenizer: &Tokenizer,
) -> Result<Vec<f32>> {
    // println!("sentence: {}", sentence);
    // Tokenizing the sentence
    let tokens = tokenizer
        .encode_batch(vec![sentence], true)
        .map_err(E::msg)
        .context("Unable to encode sentence")?;

    // Getting the token ids from the tokens
    let token_ids = tokens
        .iter()
        .map(|tokens| {
            let tokens = tokens.get_ids().to_vec();
            Ok(Tensor::new(tokens.as_slice(), &Device::Cpu)?)
        })
        .collect::<Result<Vec<_>>>()
        .context("Unable to get token ids")?;

    // Stacking the token ids into a tensor
    let token_ids = Tensor::stack(&token_ids, 0).context("Unable to stack token ids")?;

    // Getting the embeddings from the model
    let embeddings = model
        .forward(&token_ids)
        .context("Unable to get embeddings from model")?;

    // Normalizing the embeddings
    let (_n_sentence, n_tokens, _hidden_size) = embeddings
        .dims3()
        .context("Unable to get embeddings dimensions")?;
    let embeddings =
        (embeddings.sum(1)? / (n_tokens as f64)).context("Unable to get embeddings sum")?;
    let embeddings = embeddings
        .broadcast_div(&embeddings.sqr()?.sum_keepdim(1)?.sqrt()?)
        .context("Unable to get embeddings broadcast div")?;

    let vec2 = embeddings
        .to_vec2()
        .context("Unable to get embeddings to vec2")?;
    Ok(vec2.first().context("vector is empty")?.clone())
}
