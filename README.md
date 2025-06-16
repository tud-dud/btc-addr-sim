# btc-addr-sim


This tool parses the output of `bitcoin-cli getrawaddrman` as of Bitcoin Core
v28.0.

```
EXPERIMENTAL warning: this call may be changed in future releases.

Returns information on all address manager entries for the new and tried tables.

Result:
{                                  (json object)
  "table" : {                      (json object) buckets with addresses in the address manager table ( new, tried )
    "bucket/position" : {          (json object) the location in the address manager table (<bucket>/<position>)
      "address" : "str",           (string) The address of the node
      "mapped_as" : n,             (numeric, optional) The ASN mapped to the IP of this peer per our current ASMap
      "port" : n,                  (numeric) The port number of the node
      "network" : "str",           (string) The network (ipv4, ipv6, onion, i2p, cjdns) of the address
      "services" : n,              (numeric) The services offered by the node
      "time" : xxx,                (numeric) The UNIX epoch time when the node was last seen
      "source" : "str",            (string) The address that relayed the address to us
      "source_network" : "str",    (string) The network (ipv4, ipv6, onion, i2p, cjdns) of the source address
      "source_mapped_as" : n       (numeric, optional) The ASN mapped to the IP of this peer's source per our current ASMap
    },
    ...
  },
  ...
}

```
