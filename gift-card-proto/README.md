# DEMO app using protobuf

It expects synapse to be running in docker, and connected to Axon Server.
It has both `prost` amd `warp` as dependencies in addition the `synapse-client`.
The subject used is about gift cards. These can be created, redeemed and canceled.
With the http endpoint, either commands or queries can be fired via `JSON` over `http`.
The returned octet-stream from synapse is than serialized and deserialized to `JSON`.