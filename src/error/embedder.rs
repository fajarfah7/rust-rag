#[derive(Debug)]
pub enum EmbedError{
    // empty input text
    EmptyInput,

    // request to provider failed
    Transport(String),

    // provider response error
    Provider {
        code: Option<u16>,
        message: String,
    },

    // response not expected
    InvalidResponse(String),

    // quota limit
    RateLimited,

    // unknown
    Unknown(String)
}