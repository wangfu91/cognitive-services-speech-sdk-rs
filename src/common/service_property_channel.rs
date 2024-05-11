/// Defines channels used to pass property settings to service.
/// Added in version 1.5.0.
#[derive(Debug)]
pub enum ServicePropertyChannel {
    /// Uses URI query parameter to pass property settings to service.
    UriQueryParameter = 0,

    /// Uses HttpHeader to set a key/value in a HTTP header.
    HttpHeader = 1,
}
