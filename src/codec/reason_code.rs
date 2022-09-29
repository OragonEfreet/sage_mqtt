use crate::{codec, ReasonCode, Result as SageResult};
use std::marker::Unpin;
use tokio::io::AsyncWrite;

///Write the given `ReasonCode`in one byte, returning `1` in case of success.
pub async fn write_reason_code<W: AsyncWrite + Unpin>(
    code: ReasonCode,
    writer: W,
) -> SageResult<usize> {
    codec::write_byte(
        match code {
            ReasonCode::Success => 0x00,
            ReasonCode::GrantedQoS1 => 0x01,
            ReasonCode::GrantedQoS2 => 0x02,
            ReasonCode::DisconnectWithWillMessage => 0x04,
            ReasonCode::NoMatchingSubscribers => 0x10,
            ReasonCode::NoSubscriptionExisted => 0x11,
            ReasonCode::ContinueAuthentication => 0x18,
            ReasonCode::ReAuthenticate => 0x19,
            ReasonCode::UnspecifiedError => 0x80,
            ReasonCode::MalformedPacket => 0x81,
            ReasonCode::ProtocolError => 0x82,
            ReasonCode::ImplementationSpecificError => 0x83,
            ReasonCode::UnsupportedProtocolVersion => 0x84,
            ReasonCode::ClientIdentifierNotValid => 0x85,
            ReasonCode::BadUserNameOrPassword => 0x86,
            ReasonCode::NotAuthorized => 0x87,
            ReasonCode::ServerUnavailable => 0x88,
            ReasonCode::ServerBusy => 0x89,
            ReasonCode::Banned => 0x8A,
            ReasonCode::ServerShuttingDown => 0x8B,
            ReasonCode::BadAuthenticationMethod => 0x8C,
            ReasonCode::KeepAliveTimeout => 0x8D,
            ReasonCode::SessionTakenOver => 0x8E,
            ReasonCode::TopicFilterInvalid => 0x8F,
            ReasonCode::TopicNameInvalid => 0x90,
            ReasonCode::PacketIdentifierInUse => 0x91,
            ReasonCode::PacketIdentifierNotFound => 0x92,
            ReasonCode::ReceiveMaximumExceeded => 0x93,
            ReasonCode::TopicAliasInvalid => 0x94,
            ReasonCode::PacketTooLarge => 0x95,
            ReasonCode::MessageRateTooHigh => 0x96,
            ReasonCode::QuotaExceeded => 0x97,
            ReasonCode::AdministrativeAction => 0x98,
            ReasonCode::PayloadFormatInvalid => 0x99,
            ReasonCode::RetainNotSupported => 0x9A,
            ReasonCode::QoSNotSupported => 0x9B,
            ReasonCode::UseAnotherServer => 0x9C,
            ReasonCode::ServerMoved => 0x9D,
            ReasonCode::SharedSubscriptionsNotSupported => 0x9E,
            ReasonCode::ConnectionRateExceeded => 0x9F,
            ReasonCode::MaximumConnectTime => 0xA0,
            ReasonCode::SubscriptionIdentifiersNotSupported => 0xA1,
            ReasonCode::WildcardSubscriptionsNotSupported => 0xA2,
        },
        writer,
    )
    .await
}

#[cfg(test)]
mod unit {

    use super::*;

    #[tokio::test]
    async fn encode() {
        for (reason_code, byte) in vec![
            (ReasonCode::Success, 0x00_u8),
            (ReasonCode::GrantedQoS1, 0x01_u8),
            (ReasonCode::GrantedQoS2, 0x02_u8),
            (ReasonCode::DisconnectWithWillMessage, 0x04_u8),
            (ReasonCode::NoMatchingSubscribers, 0x10_u8),
            (ReasonCode::NoSubscriptionExisted, 0x11_u8),
            (ReasonCode::ContinueAuthentication, 0x18_u8),
            (ReasonCode::ReAuthenticate, 0x19_u8),
            (ReasonCode::UnspecifiedError, 0x80_u8),
            (ReasonCode::MalformedPacket, 0x81_u8),
            (ReasonCode::ProtocolError, 0x82_u8),
            (ReasonCode::ImplementationSpecificError, 0x83_u8),
            (ReasonCode::UnsupportedProtocolVersion, 0x84_u8),
            (ReasonCode::ClientIdentifierNotValid, 0x85_u8),
            (ReasonCode::BadUserNameOrPassword, 0x86_u8),
            (ReasonCode::NotAuthorized, 0x87_u8),
            (ReasonCode::ServerUnavailable, 0x88_u8),
            (ReasonCode::ServerBusy, 0x89_u8),
            (ReasonCode::Banned, 0x8A_u8),
            (ReasonCode::ServerShuttingDown, 0x8B_u8),
            (ReasonCode::BadAuthenticationMethod, 0x8C_u8),
            (ReasonCode::KeepAliveTimeout, 0x8D_u8),
            (ReasonCode::SessionTakenOver, 0x8E_u8),
            (ReasonCode::TopicFilterInvalid, 0x8F_u8),
            (ReasonCode::TopicNameInvalid, 0x90_u8),
            (ReasonCode::PacketIdentifierInUse, 0x91_u8),
            (ReasonCode::PacketIdentifierNotFound, 0x92_u8),
            (ReasonCode::ReceiveMaximumExceeded, 0x93_u8),
            (ReasonCode::TopicAliasInvalid, 0x94_u8),
            (ReasonCode::PacketTooLarge, 0x95_u8),
            (ReasonCode::MessageRateTooHigh, 0x96_u8),
            (ReasonCode::QuotaExceeded, 0x97_u8),
            (ReasonCode::AdministrativeAction, 0x98_u8),
            (ReasonCode::PayloadFormatInvalid, 0x99_u8),
            (ReasonCode::RetainNotSupported, 0x9A_u8),
            (ReasonCode::QoSNotSupported, 0x9B_u8),
            (ReasonCode::UseAnotherServer, 0x9C_u8),
            (ReasonCode::ServerMoved, 0x9D_u8),
            (ReasonCode::SharedSubscriptionsNotSupported, 0x9E_u8),
            (ReasonCode::ConnectionRateExceeded, 0x9F_u8),
            (ReasonCode::MaximumConnectTime, 0xA0_u8),
            (ReasonCode::SubscriptionIdentifiersNotSupported, 0xA1_u8),
            (ReasonCode::WildcardSubscriptionsNotSupported, 0xA2_u8),
        ] {
            let mut result = Vec::new();
            assert_eq!(
                write_reason_code(reason_code, &mut result).await.unwrap(),
                1
            );
            assert_eq!(result[0], byte);
        }
    }
}
