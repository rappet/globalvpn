Global VPN Metadata Protocol
============================

Areas
-----

### Global Area
The global area is the area in which all nodes are in.
The global area holds public metadata, dictionary and relay nodes.


Architecture
------------

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
