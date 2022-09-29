use crate::{codec, PacketType, ReasonCode::MalformedPacket, Result as SageResult};
use std::{convert::TryInto, marker::Unpin};
use tokio::io::{AsyncRead, AsyncWrite};

/// Write the given `PacketType` in one byte according to
/// MQTT5 specifications.
/// In case of success, returns `1`.
pub async fn write_control_packet_type<W: AsyncWrite + Unpin>(
    cpt: PacketType,
    writer: W,
) -> SageResult<usize> {
    codec::write_byte(
        match cpt {
            PacketType::Reserved => 0b0000_0000,
            PacketType::Connect => 0b0001_0000,
            PacketType::ConnAck => 0b0010_0000,
            PacketType::Publish {
                duplicate,
                qos,
                retain,
            } => 0b0011_0000 | (duplicate as u8) << 3 | (qos as u8) << 2 | retain as u8,
            PacketType::PubAck => 0b0100_0000,
            PacketType::PubRec => 0b0101_0000,
            PacketType::PubRel => 0b0110_0010,
            PacketType::PubComp => 0b0111_0000,
            PacketType::Subscribe => 0b1000_0010,
            PacketType::SubAck => 0b1001_0000,
            PacketType::UnSubscribe => 0b1010_0010,
            PacketType::UnSubAck => 0b1011_0000,
            PacketType::PingReq => 0b1100_0000,
            PacketType::PingResp => 0b1101_0000,
            PacketType::Disconnect => 0b1110_0000,
            PacketType::Auth => 0b1111_0000,
        },
        writer,
    )
    .await
}

/// Read the given `reader` for a `PacketType`.
/// In case of success, returns a `PacketType` instance.
pub async fn read_control_packet_type<R: AsyncRead + Unpin>(
    reader: R,
) -> SageResult<PacketType> {
    let packet_type = codec::read_byte(reader).await?;
    let packet_type = match (packet_type >> 4, packet_type & 0b0000_1111) {
        (0b0000, 0b0000) => PacketType::Reserved,
        (0b0001, 0b0000) => PacketType::Connect,
        (0b0010, 0b0000) => PacketType::ConnAck,
        (0b0011, flags) => PacketType::Publish {
            duplicate: (flags & 0b0111) > 0,
            qos: ((flags & 0b0110) >> 1).try_into()?,
            retain: (flags & 0b0001) > 0,
        },
        (0b0100, 0b0000) => PacketType::PubAck,
        (0b0101, 0b0000) => PacketType::PubRec,
        (0b0110, 0b0010) => PacketType::PubRel,
        (0b0111, 0b0000) => PacketType::PubComp,
        (0b1000, 0b0010) => PacketType::Subscribe,
        (0b1001, 0b0000) => PacketType::SubAck,
        (0b1010, 0b0010) => PacketType::UnSubscribe,
        (0b1011, 0b0000) => PacketType::UnSubAck,
        (0b1100, 0b0000) => PacketType::PingReq,
        (0b1101, 0b0000) => PacketType::PingResp,
        (0b1110, 0b0000) => PacketType::Disconnect,
        (0b1111, 0b0000) => PacketType::Auth,
        _ => return Err(MalformedPacket.into()),
    };
    Ok(packet_type)
}

#[cfg(test)]
mod unit {

    use crate::{Error, ReasonCode};
    use std::io::Cursor;

    use super::*;

    #[tokio::test]
    async fn mqtt_2_1_3_1() {
        let reserved_flags_per_type = [
            (0b0001, 0b0000),
            (0b0010, 0b0000),
            (0b0100, 0b0000),
            (0b0101, 0b0000),
            (0b0110, 0b0010),
            (0b0111, 0b0000),
            (0b1000, 0b0010),
            (0b1001, 0b0000),
            (0b1010, 0b0010),
            (0b1011, 0b0000),
            (0b1100, 0b0000),
            (0b1101, 0b0000),
            (0b1110, 0b0000),
            (0b1111, 0b0000),
        ];

        for (packet_type, flags) in &reserved_flags_per_type {
            for i in 0b0000..=0b1111 {
                if i == *flags {
                    continue;
                }
                let buffer = [*packet_type, *flags, 0x00];
                let mut test_stream = Cursor::new(buffer);
                assert!(matches!(
                    read_control_packet_type(&mut test_stream).await,
                    Err(Error::Reason(ReasonCode::MalformedPacket))
                ));
            }
        }
    }
}
