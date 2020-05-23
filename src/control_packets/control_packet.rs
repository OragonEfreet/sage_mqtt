use crate::{
    Auth, ConnAck, Connect, ControlPacketType, Disconnect, Error, FixedHeader, PubAck, PubComp,
    PubRec, PubRel, Publish, Result as SageResult, SubAck, Subscribe, UnSubAck, UnSubscribe,
};
use futures::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use std::marker::Unpin;

/// The standard type to manipulate a AsyncRead/AsyncWrite-able MQTT packet. Each packet
/// is an enum value with its own type.
#[derive(Debug, Clone)]
pub enum ControlPacket {
    /// CONNECT MQTT packet. Opens a connection request.
    Connect(Connect),
    /// CONNACK MQTT packet. Aknowledge a connectio request.
    ConnAck(ConnAck),

    /// PUBLISH MQTT packet. Delivery a message to or from a server.
    Publish(Publish),

    /// PUBACK MQTT packet. Ackowledge a QoS 1 or QoS 2 message.
    PubAck(PubAck),

    /// PUBREC MQTT packet. Ackowledge a QoS 2 message.
    PubRec(PubRec),

    /// PUBREL MQTT packet. Ackowledge a QoS 2 message.
    PubRel(PubRel),

    /// PUBCOMP MQTT packet. Ackowledge a QoS 2 message.
    PubComp(PubComp),

    /// SUBSCRIBE MQTT packet. Subscribe a client to topics.
    Subscribe(Subscribe),

    /// SUBACK MQTT packet. Acknowledge a client SUBSCRIBE packet.
    SubAck(SubAck),

    /// UNSUBSCRIBE MQTT packet. Unsubscribe a client from topics.
    UnSubscribe(UnSubscribe),

    /// UNSUBACK MQTT packet. Acknowledge a client UNSUBSCRIBE packet.
    UnSubAck(UnSubAck),

    /// PINGREQ MQTT packet. Send a ping request.
    PingReq,

    /// PINGRESP MQTT packet. Respond to a ping request.
    PingResp,

    /// DISCONNECT MQTT packet. Disconnect a connextion and optionally a session.
    Disconnect(Disconnect),

    /// AUTH MQTT packet. Performs authentication exchanges between clients and server.
    Auth(Auth),
}

impl ControlPacket {
    /// Writes the entire `ControlPacket` to `writer`, returning the number of
    /// bytes written.
    /// In case of failure, the operation will return any MQTT-related error, or
    /// `std::io::Error`.
    pub async fn encode<W: AsyncWrite + Unpin>(self, writer: &mut W) -> SageResult<usize> {
        let mut variable_and_payload = Vec::new();
        let (packet_type, remaining_size) = match self {
            ControlPacket::Connect(packet) => (
                ControlPacketType::CONNECT,
                packet.write(&mut variable_and_payload).await?,
            ),
            ControlPacket::ConnAck(packet) => (
                ControlPacketType::CONNACK,
                packet.write(&mut variable_and_payload).await?,
            ),
            ControlPacket::PingReq => (ControlPacketType::PINGREQ, 0),
            ControlPacket::PingResp => (ControlPacketType::PINGRESP, 0),
            ControlPacket::UnSubAck(packet) => (
                ControlPacketType::UNSUBACK,
                packet.write(&mut variable_and_payload).await?,
            ),
            ControlPacket::Auth(packet) => (
                ControlPacketType::AUTH,
                packet.write(&mut variable_and_payload).await?,
            ),
            ControlPacket::PubAck(packet) => (
                ControlPacketType::PUBACK,
                packet.write(&mut variable_and_payload).await?,
            ),
            ControlPacket::UnSubscribe(packet) => (
                ControlPacketType::UNSUBSCRIBE,
                packet.write(&mut variable_and_payload).await?,
            ),
            ControlPacket::PubRec(packet) => (
                ControlPacketType::PUBREC,
                packet.write(&mut variable_and_payload).await?,
            ),
            ControlPacket::Disconnect(packet) => (
                ControlPacketType::DISCONNECT,
                packet.write(&mut variable_and_payload).await?,
            ),
            ControlPacket::PubRel(packet) => (
                ControlPacketType::PUBREL,
                packet.write(&mut variable_and_payload).await?,
            ),
            ControlPacket::SubAck(packet) => (
                ControlPacketType::SUBACK,
                packet.write(&mut variable_and_payload).await?,
            ),
            ControlPacket::PubComp(packet) => (
                ControlPacketType::PUBCOMP,
                packet.write(&mut variable_and_payload).await?,
            ),
            ControlPacket::Subscribe(packet) => (
                ControlPacketType::SUBSCRIBE,
                packet.write(&mut variable_and_payload).await?,
            ),
            ControlPacket::Publish(packet) => (
                ControlPacketType::PUBLISH {
                    duplicate: packet.duplicate,
                    qos: packet.qos,
                    retain: packet.retain,
                },
                packet.write(&mut variable_and_payload).await?,
            ),
        };

        let mut fixed_header_buffer = Vec::new();

        let fixed_size = FixedHeader {
            packet_type,
            remaining_size,
        }
        .encode(&mut fixed_header_buffer)
        .await?;

        println!("{:?}", fixed_header_buffer);

        writer.write_all(&fixed_header_buffer).await?;
        writer.write_all(&variable_and_payload).await?;
        Ok(fixed_size)
    }

    /// Reads a control packet from `reader`, returning a new `ControlPacket`.
    /// In case of failure, the operation will return any MQTT-related error, or
    /// `std::io::Error`.
    pub async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<Self> {
        let fixed_header = FixedHeader::decode(reader).await?;

        let packet = match fixed_header.packet_type {
            ControlPacketType::CONNECT => ControlPacket::Connect(Connect::read(reader).await?),
            ControlPacketType::CONNACK => ControlPacket::ConnAck(ConnAck::read(reader).await?),
            ControlPacketType::PUBACK => {
                ControlPacket::PubAck(PubAck::read(reader, fixed_header.remaining_size == 2).await?)
            }
            ControlPacketType::PUBREC => {
                ControlPacket::PubRec(PubRec::read(reader, fixed_header.remaining_size == 2).await?)
            }
            ControlPacketType::PINGREQ => ControlPacket::PingReq,
            ControlPacketType::PINGRESP => ControlPacket::PingResp,
            ControlPacketType::SUBACK => {
                ControlPacket::SubAck(SubAck::read(reader, fixed_header.remaining_size).await?)
            }
            ControlPacketType::UNSUBSCRIBE => ControlPacket::UnSubscribe(
                UnSubscribe::read(reader, fixed_header.remaining_size).await?,
            ),
            ControlPacketType::AUTH => ControlPacket::Auth(Auth::read(reader).await?),
            ControlPacketType::PUBREL => {
                ControlPacket::PubRel(PubRel::read(reader, fixed_header.remaining_size == 2).await?)
            }
            ControlPacketType::DISCONNECT => {
                ControlPacket::Disconnect(Disconnect::read(reader).await?)
            }
            ControlPacketType::PUBCOMP => ControlPacket::PubComp(
                PubComp::read(reader, fixed_header.remaining_size == 2).await?,
            ),

            ControlPacketType::SUBSCRIBE => ControlPacket::Subscribe(
                Subscribe::read(reader, fixed_header.remaining_size).await?,
            ),

            ControlPacketType::UNSUBACK => {
                ControlPacket::UnSubAck(UnSubAck::read(reader, fixed_header.remaining_size).await?)
            }

            ControlPacketType::PUBLISH {
                duplicate,
                qos,
                retain,
            } => ControlPacket::Publish(
                Publish::read(
                    reader,
                    duplicate,
                    qos,
                    retain,
                    fixed_header.remaining_size as u64,
                )
                .await?,
            ),
            _ => return Err(Error::ProtocolError),
        };

        Ok(packet)
    }
}
