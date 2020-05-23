use crate::{Property, Result as SageResult};
use async_std::io::Write;
use std::marker::Unpin;

/// By default, `Connect` packets provide optional `user_name` and `password`
/// fields which can be used to provide basic authentication.
/// Enhanced authentication can be provided by using an `Authentication`
/// structure which will initialize a challenge / response style authentication.
/// Ii might imply the exchange of several `Auth` with reason code
/// `ContinueAuthentication` until eventually one is send with either `Success`
/// or any relevant error code and, in that case, close the connection.
/// The authentication method which is used as an agreement on how authentication
/// exchanges will perform. Authentication data can be sent at any moment
/// according to this agreement.
/// See the section 4.12 (Enhanced Authentication) of the MQTT 5 specifications
/// for examples.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Authentication {
    /// Specifies the authentication method, such as "SCRAM-SHA-1" or "GS2-KRB5".
    /// The actual support for a given authentication method is up to the server.
    /// If the server does not support the requested method, it will respond
    /// with a `Connack` packet with reason code `NotAuthorized` or
    /// `BadAuthenticationMethod` and close the connection.
    pub method: String,

    /// Authentication may contains data. The content depends on the
    /// authentication method.
    pub data: Vec<u8>,
}

impl Authentication {
    /// Write authentication data into `writer`, returning the written size
    /// in case of success.
    pub async fn write<W: Write + Unpin>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = Property::AuthenticationMethod(self.method)
            .encode(writer)
            .await?;
        if !self.data.is_empty() {
            n_bytes += Property::AuthenticationData(self.data)
                .encode(writer)
                .await?;
        }
        Ok(n_bytes)
    }
}

#[cfg(test)]
mod unit {

    use super::*;

    #[async_std::test]
    async fn encode() {
        let mut result = Vec::new();
        let test_data = Authentication {
            method: "Willow".into(),
            data: vec![0x0D, 0x15, 0xEA, 0x5E],
        };

        assert_eq!(test_data.write(&mut result).await.unwrap(), 16);
        assert_eq!(
            result,
            vec![21, 0, 6, 87, 105, 108, 108, 111, 119, 22, 0, 4, 13, 21, 234, 94]
        );
    }
}
