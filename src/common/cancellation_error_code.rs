/// CancellationErrorCode defines error code in case that CancellationReason is Error.
#[derive(Debug)]
pub enum CancellationErrorCode {
    /// No error.
    /// If CancellationReason is EndOfStream, CancellationErrorCode
    /// is set to NoError.
    NoError = 0,
    /// Indicates an authentication error.
    /// An authentication error occurs if subscription key or authorization token is invalid, expired,
    /// or does not match the region being used.
    AuthenticationFailure = 1,
    /// Indicates that one or more recognition parameters are invalid or the audio format is not supported.
    BadRequest = 2,
    /// Indicates that the number of parallel requests exceeded the number of allowed concurrent transcriptions for the subscription.
    TooManyRequests = 3,
    /// Indicates that the free subscription used by the request ran out of quota.
    Forbidden = 4,
    /// Indicates a connection error.
    ConnectionFailure = 5,
    /// Indicates a time-out error when waiting for response from service.
    ServiceTimeout = 6,
    /// Indicates that an error is returned by the service.
    ServiceError = 7,
    /// Indicates that the service is currently unavailable.
    ServiceUnavailable = 8,
    /// Indicates an unexpected runtime error.
    RuntimeError = 9,
    /// Indicates the Speech Service is temporarily requesting a reconnect to a different endpoint.
    /// Note: Used internally
    ServiceRedirectTemporary = 10,
    /// Indicates the Speech Service is permanently requesting a reconnect to a different endpoint.
    /// Note: Used internally
    ServiceRedirectPermanent = 11,
    /// Indicates the embedded speech (SR or TTS) model is not available or corrupted.
    EmbeddedModelError = 12,
}

impl CancellationErrorCode {
    pub fn from_u32(code: u32) -> Self {
        match code {
            0 => CancellationErrorCode::NoError,
            1 => CancellationErrorCode::AuthenticationFailure,
            2 => CancellationErrorCode::BadRequest,
            3 => CancellationErrorCode::TooManyRequests,
            4 => CancellationErrorCode::Forbidden,
            5 => CancellationErrorCode::ConnectionFailure,
            6 => CancellationErrorCode::ServiceTimeout,
            7 => CancellationErrorCode::ServiceError,
            8 => CancellationErrorCode::ServiceUnavailable,
            9 => CancellationErrorCode::RuntimeError,
            10 => CancellationErrorCode::ServiceRedirectTemporary,
            11 => CancellationErrorCode::ServiceRedirectPermanent,
            _ => CancellationErrorCode::EmbeddedModelError,
        }
    }

    pub fn from_i32(code: i32) -> Self {
        CancellationErrorCode::from_u32(code as u32)
    }
}
