---
title: "Global VPN Metadata Protocol"
author: "Raphael Peters <rappet@rappet.de>"
abstract: |
	GlobalVPN is a decentral VPN meshed sollution.
lang: en-US
documentclass: article
geometry: margin=2cm
---

Introduction
============

Direct communication between enddevices on the internet is not allways possible.
Techniques such as NAT[^nat]
to mitigate IPv4 outage prevent direct connections whithout techniques
such as STUN[^stun] can bypass
those translation mechanisms on some cases.
IP Version 6[^ipv6] do not have this problems, but are not allways avaliable.
Customer Premise Equipment usually does also filter incomming connections
wihtout further configurations.
Direct connections are also not trivial if a device is roaming.

VPN Protocols such as Wireguard[^wireguard], OpenVPN[^openvpn] or Tinc[^tinc]
can bypass such problems by tunneling connections to a endpoint.
Tinc even offers meshing to allow two devices behind a NAT communicating with
eachother and having redundancy between gateways.
These protocols allways need a bit of confiuration and key exchange and don't
work out of the box.
Global VPN changes this problems by using a global mesh VPN with public relays
that can be used with default configuration to communicate with other Global VPN
connected enddevices.
For this the VPN deamon needs to be installed on each enddevice.
Nodes in the mesh can be configured to store metadata of the whole network,
which is broadcasted on change between all metadata nodes.
Such metadata is used to find a connection to a specific enddevice.

[^nat]: P. Srisuresh and M. Holdrege, IP Network Address Translator (NAT) Terminology and Considerations (RFC2663, <https://tools.ietf.org/html/rfc2663>)
[^stun]: J. Rosenberg et al., Session Traversal Utilities for NAT (RFC5389, https://tools.ietf.org/html/rfc5389)
[^ipv6]: S. Deering and R. Hinden, Internet Protocol, Version 6 (IPv6) Specification (RFC8200, <https://tools.ietf.org/html/rfc8200>)
[^wireguard]: Jason A. Donenfeld <jason@zx2c4.com>, WireGuard: Next Generation Kernel Network Tunnel (<https://www.wireguard.com/papers/wireguard.pdf>)
[^openvpn]: OpenVPN (<https://openvpn.net/>)
[^tinc]: Tinc VPN (<https://www.tinc-vpn.org/>)

Definitions
===========
Node
: A single instance. Each nodes can have different tasks.

Areas
=====

Global Area
-----------

The global area is the area in which all nodes are in.
The global area holds public metadata, dictionary and relay nodes.

Private Areas
-------------

Todo

Architecture
============

Node Types
----------

### Basic node
Connects to metadata node using encrypted TCP.
All nodes are also basic nodes.

### Metadata node
Has list with other metadata nodes.
Can close connection after providing list of other metadata nodes
(used for nodes which are confured in basic information to bootstrap
access to the network).
Returns also a list with dictionary nodes.

### Dictionary node
A dictionay node responds with an address to a relay node or basic node
if supplied with an identifier.
There are multiple ways a dictionary mode can lookup such information.

- proxy to other dictionary node
- holding the global dictionary which is broadcasted between dictionary nodes
- holding a partial dictionay using a distributed hash map (later)
- hybrid: hold only a dictionary for current area, proxy others

### Relay node
Basic nodes who can't be reached directly over IP can use a relay node which
proxies packets.
NAT hole punching is used by the basic node so that the relay node can reach
the basic node.

Protocol
========
There is the metadata and the packet protocol.
Metadata is shared over a TCP connection and encrypted using the private
key of the host to be reached.
The packet protocol consists of UDP packets with an encrypted payload.
The key for packet protocol is communicated over the metadata protocol.

Node States
-----------

### Update metadata/dictionary nodes list
(optional) connect to metadata node to update list of metadata/dictionary nodes.

### Get reachability information
Get information about the NAT type (similary as described in [^stun])
From the metadata/dictionary nodes.

### Register node to dictionary node.
Open a connection to a dictionary node and set how the node can be reached.
The dictionary node floods the reachability information to all neighbour nodes
(Update).

### Established state
Node can communicate with other nodes an can be reached globally.

### Close
Node floods a toombstone update packet and closes connections.

Protocol States
---------------

Encryption Initialization:

- Send packet with own public key information (both sides)
- Receive packet with foreign public key information
- Send encrypted symmetric key to other side.

Basic Initialization:

- Send node types.
- Do stuff

Packet
------

This section describes the encoding of the packets and structure of initial
defined packet types.

Format
------

Type  Name        Comment
----- ----------- -----------------------
u16   Length      describes payload length
u8    Packet Type
bytes Payload

The type must be defined in a specification.
Implementation specific packet types can be added via custom packet type.
A packet payload should not be larger than 65536 bytes.

### Packet types

Typeid Name
------ ----
0x01   OPEN
0x02   UPDATE
0x03   ERROR
0x04   KEEPALIVE
0x05   CUSTOM

### Open Packet

Type Name
---- -------
u8   Version
u8   Optional Parameters Count
~    Optional Parameters

### Update Packet

### Error Packet

Type  Name         
----- -------------
u8    Error code    
u8    Error subcode 
bytes Data

### Keepalive Packet

### Custom Packet

To allow custom additions to the protocol,
it is possible to add custom packet types without adding them to the standard.

Type Name
---- ---------
u32  Vendor ID
u16  subtype

The vendor ID should be randomly choosen and be published.[^1]
If a vendor ID is allready in use, it should not be reused by
a different vendor.

[^1]: A public list is to be announced. Contact developers if you need a vendor ID.

Security Considerations
=======================

