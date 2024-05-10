1. 运行
bfc-node --config-path ./validator/validator.yaml

2.create validator.info
bfc validator make-validator-info alex abc abc abc 127.0.0.1  100




2.1 stake
using the following page to stake
http://localhost:2946/object/BFC0000000000000000000000000000000000000000000000000000000000000003ac7e?module=sui_system

3. become validator 
bfc validator become-candidate ./validator.info


4.  bfc validator join-committee
