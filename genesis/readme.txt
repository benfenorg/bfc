


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
