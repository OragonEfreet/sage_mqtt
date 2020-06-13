use crate::{codec, PacketType, ReasonCode::MalformedPacket, Result as SageResult};
use futures::io::{AsyncRead, AsyncWrite};
use std::{convert::TryInto, marker::Unpin};

/// Write the given `PacketType` in one byte according to
/// MQTT5 specifications.
/// In case of success, returns `1`.
pub async fn write_control_packet_type<W: AsyncWrite + Unpin>(
    cpt: PacketType,
    writer: &mut W,
) -> SageResult<usize> {
    codec::write_byte(
        match cpt {
            PacketType::RESERVED => 0b0000_0000,
            PacketType::CONNECT => 0b0001_0000,
            PacketType::CONNACK => 0b0010_0000,
            PacketType::PUBLISH {
                duplicate,
                qos,
                retain,
            } => 0b0011_0000 | (duplicate as u8) << 3 | (qos as u8) << 2 | retain as u8,
            PacketType::PUBACK => 0b0100_0000,
            PacketType::PUBREC => 0b0101_0000,
            PacketType::PUBREL => 0b0110_0010,
            PacketType::PUBCOMP => 0b0111_0000,
            PacketType::SUBSCRIBE => 0b1000_0010,
            PacketType::SUBACK => 0b1001_0000,
            PacketType::UNSUBSCRIBE => 0b1010_0010,
            PacketType::UNSUBACK => 0b1011_0000,
            PacketType::PINGREQ => 0b1100_0000,
            PacketType::PINGRESP => 0b1101_0000,
            PacketType::DISCONNECT => 0b1110_0000,
            PacketType::AUTH => 0b1111_0000,
        },
        writer,
    )
    .await
}

/// Read the given `reader` for a `PacketType`.
/// In case of success, returns a `PacketType` instance.
pub async fn read_control_packet_type<R: AsyncRead + Unpin>(
    reader: &mut R,
) -> SageResult<PacketType> {
    let packet_type = codec::read_byte(reader).await?;
    let packet_type = match (packet_type >> 4, packet_type & 0b0000_1111) {
        (0b0000, 0b0000) => PacketType::RESERVED,
        (0b0001, 0b0000) => PacketType::CONNECT,
        (0b0010, 0b0000) => PacketType::CONNACK,
        (0b0011, flags) => PacketType::PUBLISH {
            duplicate: (flags & 0b0111) > 0,
            qos: ((flags & 0b0110) >> 1).try_into()?,
            retain: (flags & 0b0001) > 0,
        },
        (0b0100, 0b0000) => PacketType::PUBACK,
        (0b0101, 0b0000) => PacketType::PUBREC,
        (0b0110, 0b0010) => PacketType::PUBREL,
        (0b0111, 0b0000) => PacketType::PUBCOMP,
        (0b1000, 0b0010) => PacketType::SUBSCRIBE,
        (0b1001, 0b0000) => PacketType::SUBACK,
        (0b1010, 0b0010) => PacketType::UNSUBSCRIBE,
        (0b1011, 0b0000) => PacketType::UNSUBACK,
        (0b1100, 0b0000) => PacketType::PINGREQ,
        (0b1101, 0b0000) => PacketType::PINGRESP,
        (0b1110, 0b0000) => PacketType::DISCONNECT,
        (0b1111, 0b0000) => PacketType::AUTH,
        _ => return Err(MalformedPacket.into()),
    };
    Ok(packet_type)
}

#[cfg(test)]
mod unit {

    use crate::{Error, ReasonCode};
    use async_std::io::Cursor;

    use super::*;

    #[async_std::test]
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
                assert_matches!(
                    read_control_packet_type(&mut test_stream).await,
                    Err(Error::Reason(ReasonCode::MalformedPacket))
                );
            }
        }
    }
}
