
local area network

1. bfc genesis_ceremony init
2. ./generate-validator-key.sh
3. ./add-validator-using-userip.sh
    192.168.1.4 10000
    192.168.1.4 20000
    192.168.1.4 30000
    192.168.1.3 40000


4. bfc genesis_ceremony build-unsigned-checkpoint

5. bfc genesis_ceremony verify-and-sign --key-file validator-v1.key
   bfc genesis_ceremony verify-and-sign --key-file validator-v2.key
   bfc genesis_ceremony verify-and-sign --key-file validator-v3.key
   bfc genesis_ceremony verify-and-sign --key-file validator-v4.key

6. bfc genesis_ceremony validate-state

7. bfc genesis_ceremony finalize

====================
cd obc
bfc genesis-private


9. generate config files
server1： bfc genesis_private --private-validator-names v1,v2,v3 
server2：bfc genesis_private --private-validator-names v4
===================
Importants
1. if genesis_private get large error warnings ， loading part is checking temp genesis result ，need delete unsign-genesis-blob, todo , need handle this module later
2. when using geneiss_private to generate config files, ， You must delete older config fold , other wise , it will cost some strange problem.....!!!!!!






==========================*********************====================================
global area network 【Global ip or global dns】

1. modify the rust code, and replace the code outside the comment with the code inside the comment

============================================================
let network = Network::bind(config.p2p_config.listen_address)
    .server_name(&server_name)
    .private_key(config.network_key_pair().copy().private().0.to_bytes())
    .config(anemo_config)
    .outbound_request_layer(outbound_layer)
    .start(service)?;
info!(
    server_name = server_name,
    "P2p network started on {}",
    network.local_addr()
);

/*
let p2p_port = config.p2p_config.listen_address.port();
let network = Network::bind(anemo::types::Address::HostAndPort {
    host: "0.0.0.0".into(),
    port: p2p_port,
})
    .server_name(&server_name)
    .private_key(config.network_key_pair().copy().private().0.to_bytes())
    .config(anemo_config)
    .outbound_request_layer(outbound_layer)
    .start(service)?;
info!(
    server_name = server_name,
    "P2p network started on {}",
    network.local_addr()
);

 */

//network_address: local_ip_utils::new_network_address_for_local_testing(validator.info.network_address.port().unwrap()),
network_address: validator.info.network_address.clone(),
p2p_address: validator.info.p2p_address.clone(),
//p2p_listen_address: local_ip_utils::new_p2p_listen_address_for_local_testing(validator.info.p2p_address.port().unwrap()),

============================================================

2. The same steps as local area networking

3. Call the script(genesis/replace-global-network-ip)
否则可能出现fullnode无法连接的问题

4./bfc-node --config-path ./fullnode.yaml
