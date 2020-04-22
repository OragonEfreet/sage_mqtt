# Sage MQTT

> OASIS Message Queuing Telemetry Transport (MQTT) is a connectivity procol for machine-to-machine communication. It is mainly used for Internet of things (IoT) solutions.

**At the moment, this is a toy project. Feel free to use it but don't consider it fully functionnal until version 1.0.0.**

- [MQTT.org](http://mqtt.org/)
- [OASIS Standard](https://docs.oasis-open.org/mqtt/mqtt/v5.0/mqtt-v5.0.html)

Sage MQTT is a encode/decode library for MQTT protocol.

It defines a set of "pivot types" used to parse to and from streams:

- `Bits`
- `TwoByteInteger`, `FourByteInteger` and `VariableByteInteger`
- `UTF8String` and `UTF8StringPair`
- `BinaryData`

They all implement custom `Encode` and `Decode` traits which takes benefit from any standard [Write](https://doc.rust-lang.org/std/io/trait.Write.html) and [Read](https://doc.rust-lang.org/std/io/trait.Read.html) traits respectively. 

As well as standard Control Packet types as described in the OASIS standard:

- `Connect`
- `ConnAck`
- `Publish`
- `PubAck`
- `Pubrec`
- `Pubrel`
- `Pubcomp`
- `Subscribe`
- `Suback`
- `Unsubscribe`
- `Unsuback`
- `Pingreq`
- `Pingresp`
- `Disconnect`
- `Auth`

They are wrapped into a `ControlPacket` enum that allows you to manipulate every packet types into a single one.
