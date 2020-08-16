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
(used for nodes which are configured to provide basic information to bootstrap
access to the network).
Returns also a list with dictionary nodes.

### Dictionary node
A dictionary node responds with an address to a relay node or basic node
if supplied with an identifier.
There are multiple ways a dictionary mode can lookup such information.

- proxy to other dictionary node
- holding the global dictionary which is broadcasted between dictionary nodes
- holding a partial dictionary using a distributed hash map (later)
- hybrid: hold only a dictionary for current area, proxy others

### Relay node
Basic nodes who can't be reached directly over IP can use a relay node which
proxies packets.
NAT hole punching is used by the basic node so that the relay node can reach
the basic node.

Database
--------

Informations such as reachability information will be hold in a distributed
database by dictionary nodes.
