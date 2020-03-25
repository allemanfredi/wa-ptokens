#![allow(non_snake_case)]

#[derive(FromForm)]
pub struct Message {
    pub SmsMessageSid: String,
    pub NumMedia: String,
    pub SmsSid: String,
    pub SmsStatus: String,
    pub Body: String,
    pub To: String,
    pub NumSegments: String,
    pub MessageSid: String,
    pub AccountSid: String,
    pub From: String,
    pub ApiVersion: String,
}
