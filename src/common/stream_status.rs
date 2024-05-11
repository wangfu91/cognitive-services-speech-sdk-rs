/// Defines the possible status of audio data stream.
/// Added in version 1.4.0
#[derive(Debug)]
pub enum StreamStatus {
    /// The audio data stream status is unknown
    Unknown = 0,

    //// The audio data stream contains no data
    NoData = 1,

    /// The audio data stream contains partial data of a speak request
    PartialData = 2,

    /// The audio data stream contains all data of a speak request
    AllData = 3,

    /// The audio data stream was canceled
    Canceled = 4,
}

impl StreamStatus {
    pub fn from_u32(status: u32) -> Self {
        match status {
            0 => StreamStatus::Unknown,
            1 => StreamStatus::NoData,
            2 => StreamStatus::PartialData,
            3 => StreamStatus::AllData,
            _ => StreamStatus::Canceled,
        }
    }

    pub fn from_i32(status: i32) -> Self {
        StreamStatus::from_u32(status as u32)
    }
}
