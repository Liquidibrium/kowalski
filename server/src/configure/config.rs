/// The configuration parameters for the application.
///
/// These can either be passed on the command line, or pulled from environment variables.
/// The latter is preferred as environment variables are one of the recommended ways to
/// get configuration from Kubernetes Secrets in deployment.
///
/// This is a pretty simple configuration struct as far as backend APIs go. You could imagine
/// a bunch of other parameters going here, like API keys for external services
/// or flags enabling or disabling certain features or test modes of the API.
///
/// For development convenience, these can also be read from a `.env` file in the working
/// directory where the application is started.
///
/// See `.env.sample` in the repository root for details.
#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env("PORT"), default_value = "8080")]
    pub port: u16,

    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env("POSTGRES_URL"))]
    pub postgres_url: String,

    #[clap(long, env("QDRANT_URL"))]
    pub qdrant_url: String,

    /// The HMAC signing and verification key used for login tokens (JWTs).
    ///
    /// There is no required structure or format to this key as it's just fed into a hash function.
    /// In practice, it should be a long, random string that would be infeasible to brute-force.
    #[clap(long, env("JWT_HMAC_KEY"))]
    pub jwt_hmac_key: String,
}