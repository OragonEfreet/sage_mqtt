use crate::{
    codec, Auth, ConnAck, Connect, Disconnect, PacketType, PingReq, PingResp, PubAck, PubComp,
    PubRec, PubRel, Publish, ReasonCode::ProtocolError, Result as SageResult, SubAck, Subscribe,
    UnSubAck, UnSubscribe,
};
use std::{fmt, marker::Unpin};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

#[derive(Debug)]
struct FixedHeader {
    pub packet_type: PacketType,
    pub remaining_size: usize,
}

impl FixedHeader {
    async fn encode<W: AsyncWrite + Unpin>(self, mut writer: W) -> SageResult<usize> {
        let mut n = codec::write_control_packet_type(self.packet_type, &mut writer).await?;
        n += codec::write_variable_byte_integer(self.remaining_size as u32, &mut writer).await?;
        Ok(n)
    }

    async fn decode<R: AsyncRead + Unpin>(mut reader: R) -> SageResult<Self> {
        let packet_type = codec::read_control_packet_type(&mut reader).await?;
        let remaining_size = codec::read_variable_byte_integer(&mut reader).await? as usize;
        Ok(FixedHeader {
            packet_type,
            remaining_size,
        })
    }
}

/// The standard type to manipulate a AsyncRead/AsyncWrite-able MQTT packet. Each packet
/// is an enum value with its own type.
#[derive(Debug, Clone)]
pub enum Packet {
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

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::Connect(_) => write!(f, "Connect"),
            Packet::ConnAck(connack) => write!(f, "ConnAck [{:?}]", connack.reason_code),
            Packet::Publish(_) => write!(f, "Publish"),
            Packet::PubAck(_) => write!(f, "PubAck"),
            Packet::PubRec(_) => write!(f, "PubRec"),
            Packet::PubRel(_) => write!(f, "PubRel"),
            Packet::PubComp(_) => write!(f, "PubComp"),
            Packet::Subscribe(_) => write!(f, "Subscribe"),
            Packet::SubAck(_) => write!(f, "SubAck"),
            Packet::UnSubscribe(_) => write!(f, "UnSubscribe"),
            Packet::UnSubAck(_) => write!(f, "UnSubAck"),
            Packet::PingReq => write!(f, "PingReq"),
            Packet::PingResp => write!(f, "PingResp"),
            Packet::Disconnect(disconnect) => {
                write!(f, "Disconnect [{:?}]", disconnect.reason_code)
            }
            Packet::Auth(_) => write!(f, "Auth"),
        }
    }
}

impl From<Connect> for Packet {
    fn from(control: Connect) -> Self {
        Packet::Connect(control)
    }
}
impl From<ConnAck> for Packet {
    fn from(control: ConnAck) -> Self {
        Packet::ConnAck(control)
    }
}
impl From<Publish> for Packet {
    fn from(control: Publish) -> Self {
        Packet::Publish(control)
    }
}
impl From<PubAck> for Packet {
    fn from(control: PubAck) -> Self {
        Packet::PubAck(control)
    }
}
impl From<PubRec> for Packet {
    fn from(control: PubRec) -> Self {
        Packet::PubRec(control)
    }
}
impl From<PubRel> for Packet {
    fn from(control: PubRel) -> Self {
        Packet::PubRel(control)
    }
}
impl From<PubComp> for Packet {
    fn from(control: PubComp) -> Self {
        Packet::PubComp(control)
    }
}
impl From<Subscribe> for Packet {
    fn from(control: Subscribe) -> Self {
        Packet::Subscribe(control)
    }
}
impl From<SubAck> for Packet {
    fn from(control: SubAck) -> Self {
        Packet::SubAck(control)
    }
}
impl From<UnSubscribe> for Packet {
    fn from(control: UnSubscribe) -> Self {
        Packet::UnSubscribe(control)
    }
}
impl From<UnSubAck> for Packet {
    fn from(control: UnSubAck) -> Self {
        Packet::UnSubAck(control)
    }
}
impl From<PingReq> for Packet {
    fn from(_: PingReq) -> Self {
        Packet::PingReq
    }
}
impl From<PingResp> for Packet {
    fn from(_: PingResp) -> Self {
        Packet::PingResp
    }
}
impl From<Disconnect> for Packet {
    fn from(control: Disconnect) -> Self {
        Packet::Disconnect(control)
    }
}
impl From<Auth> for Packet {
    fn from(control: Auth) -> Self {
        Packet::Auth(control)
    }
}

impl Packet {
    /// Write the entire `Packet` to `writer`, returning the number of
    /// bytes written.
    /// In case of failure, the operation will return any MQTT-related error, or
    /// `std::io::Error`.
    pub async fn encode<W: AsyncWrite + Unpin>(self, writer: &mut W) -> SageResult<usize> {
        let mut variable_and_payload = Vec::new();
        let (packet_type, remaining_size) = match self {
            Packet::Connect(packet) => (
                PacketType::Connect,
                packet.write(&mut variable_and_payload).await?,
            ),
            Packet::ConnAck(packet) => (
                PacketType::ConnAck,
                packet.write(&mut variable_and_payload).await?,
            ),
            Packet::PingReq => (PacketType::PingReq, 0),
            Packet::PingResp => (PacketType::PingResp, 0),
            Packet::UnSubAck(packet) => (
                PacketType::UnSubAck,
                packet.write(&mut variable_and_payload).await?,
            ),
            Packet::Auth(packet) => (
                PacketType::Auth,
                packet.write(&mut variable_and_payload).await?,
            ),
            Packet::PubAck(packet) => (
                PacketType::PubAck,
                packet.write(&mut variable_and_payload).await?,
            ),
            Packet::UnSubscribe(packet) => (
                PacketType::UnSubscribe,
                packet.write(&mut variable_and_payload).await?,
            ),
            Packet::PubRec(packet) => (
                PacketType::PubRec,
                packet.write(&mut variable_and_payload).await?,
            ),
            Packet::Disconnect(packet) => (
                PacketType::Disconnect,
                packet.write(&mut variable_and_payload).await?,
            ),
            Packet::PubRel(packet) => (
                PacketType::PubRel,
                packet.write(&mut variable_and_payload).await?,
            ),
            Packet::SubAck(packet) => (
                PacketType::SubAck,
                packet.write(&mut variable_and_payload).await?,
            ),
            Packet::PubComp(packet) => (
                PacketType::PubComp,
                packet.write(&mut variable_and_payload).await?,
            ),
            Packet::Subscribe(packet) => (
                PacketType::Subscribe,
                packet.write(&mut variable_and_payload).await?,
            ),
            Packet::Publish(packet) => (
                PacketType::Publish {
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

        writer.write_all(&fixed_header_buffer).await?;
        writer.write_all(&variable_and_payload).await?;
        Ok(fixed_size + remaining_size)
    }

    /// Read a control packet from `reader`, returning a new `Packet`.
    /// In case of failure, the operation will return any MQTT-related error, or
    /// `std::io::Error`.
    pub async fn decode<R: AsyncRead + Unpin>(mut reader: R) -> SageResult<Self> {
        let fixed_header = FixedHeader::decode(&mut reader).await?;

        let packet = match fixed_header.packet_type {
            PacketType::Connect => Packet::Connect(Connect::read(reader).await?),
            PacketType::ConnAck => Packet::ConnAck(ConnAck::read(reader).await?),
            PacketType::PubAck => {
                Packet::PubAck(PubAck::read(reader, fixed_header.remaining_size == 2).await?)
            }
            PacketType::PubRec => {
                Packet::PubRec(PubRec::read(reader, fixed_header.remaining_size == 2).await?)
            }
            PacketType::PingReq => Packet::PingReq,
            PacketType::PingResp => Packet::PingResp,
            PacketType::SubAck => {
                Packet::SubAck(SubAck::read(reader, fixed_header.remaining_size).await?)
            }
            PacketType::UnSubscribe => {
                Packet::UnSubscribe(UnSubscribe::read(reader, fixed_header.remaining_size).await?)
            }
            PacketType::Auth => Packet::Auth(Auth::read(reader).await?),
            PacketType::PubRel => {
                Packet::PubRel(PubRel::read(reader, fixed_header.remaining_size == 2).await?)
            }
            PacketType::Disconnect => Packet::Disconnect(Disconnect::read(reader).await?),
            PacketType::PubComp => {
                Packet::PubComp(PubComp::read(reader, fixed_header.remaining_size == 2).await?)
            }

            PacketType::Subscribe => {
                Packet::Subscribe(Subscribe::read(reader, fixed_header.remaining_size).await?)
            }

            PacketType::UnSubAck => {
                Packet::UnSubAck(UnSubAck::read(reader, fixed_header.remaining_size).await?)
            }

            PacketType::Publish {
                duplicate,
                qos,
                retain,
            } => Packet::Publish(
                Publish::read(
                    reader,
                    duplicate,
                    qos,
                    retain,
                    fixed_header.remaining_size as u64,
                )
                .await?,
            ),
            _ => return Err(ProtocolError.into()),
        };

        Ok(packet)
    }
}
