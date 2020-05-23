use crate::{codec, ControlPacketType, Error, Result as SageResult};
use async_std::io::{Read, Write};
use std::{convert::TryInto, marker::Unpin};

/// Writes the given `ControlPacketType` in one byte according to
/// MQTT5 specifications.
/// In case of success, returns `1`.
pub async fn write_control_packet_type<W: Write + Unpin>(
    cpt: ControlPacketType,
    writer: &mut W,
) -> SageResult<usize> {
    codec::write_byte(
        match cpt {
            ControlPacketType::RESERVED => 0b0000_0000,
            ControlPacketType::CONNECT => 0b0001_0000,
            ControlPacketType::CONNACK => 0b0010_0000,
            ControlPacketType::PUBLISH {
                duplicate,
                qos,
                retain,
            } => 0b0011_0000 | (duplicate as u8) << 3 | (qos as u8) << 2 | retain as u8,
            ControlPacketType::PUBACK => 0b0100_0000,
            ControlPacketType::PUBREC => 0b0101_0000,
            ControlPacketType::PUBREL => 0b0110_0010,
            ControlPacketType::PUBCOMP => 0b0111_0000,
            ControlPacketType::SUBSCRIBE => 0b1000_0010,
            ControlPacketType::SUBACK => 0b1001_0000,
            ControlPacketType::UNSUBSCRIBE => 0b1010_0010,
            ControlPacketType::UNSUBACK => 0b1011_0000,
            ControlPacketType::PINGREQ => 0b1100_0000,
            ControlPacketType::PINGRESP => 0b1101_0000,
            ControlPacketType::DISCONNECT => 0b1110_0000,
            ControlPacketType::AUTH => 0b1111_0000,
        },
        writer,
    )
    .await
}

/// Reads the given `reader` for a `ControlPacketType`.
/// In case of success, returns a `ControlPacketType` instance.
pub async fn read_control_packet_type<R: Read + Unpin>(
    reader: &mut R,
) -> SageResult<ControlPacketType> {
    let packet_type = codec::read_byte(reader).await?;
    let packet_type = match (packet_type >> 4, packet_type & 0b0000_1111) {
        (0b0000, 0b0000) => ControlPacketType::RESERVED,
        (0b0001, 0b0000) => ControlPacketType::CONNECT,
        (0b0010, 0b0000) => ControlPacketType::CONNACK,
        (0b0010, flags) => ControlPacketType::PUBLISH {
            duplicate: (flags & 0b0111) > 0,
            qos: ((flags & 0b0110) >> 1).try_into()?,
            retain: (flags & 0b0001) > 0,
        },
        (0b0100, 0b0000) => ControlPacketType::PUBACK,
        (0b0101, 0b0000) => ControlPacketType::PUBREC,
        (0b0110, 0b0010) => ControlPacketType::PUBREL,
        (0b0111, 0b0000) => ControlPacketType::PUBCOMP,
        (0b1000, 0b0010) => ControlPacketType::SUBSCRIBE,
        (0b1001, 0b0000) => ControlPacketType::SUBACK,
        (0b1010, 0b0010) => ControlPacketType::UNSUBSCRIBE,
        (0b1011, 0b0000) => ControlPacketType::UNSUBACK,
        (0b1100, 0b0000) => ControlPacketType::PINGREQ,
        (0b1101, 0b0000) => ControlPacketType::PINGRESP,
        (0b1110, 0b0000) => ControlPacketType::DISCONNECT,
        (0b1111, 0b0000) => ControlPacketType::AUTH,
        _ => return Err(Error::MalformedPacket),
    };
    Ok(packet_type)
}

#[cfg(test)]
mod unit {

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
                    Err(Error::MalformedPacket)
                );
            }
        }
    }
}
