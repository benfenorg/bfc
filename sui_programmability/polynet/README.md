# aptos-contracts

bfc链再poly测试网注册的chainId是1200
goerli再poly测试网的chainId是502

1. change the move.toml published address to last upgrade address
2. find the upgrade capability. 
bfc client upgrade  --gas-budget 100000000 --upgrade-capability BFC765b525d9fb1b92d710972afbdae2ec0ddbaba2aeb0617fdd9220888a139db426080


========================

安全建议
1. 合约部署后的所有object需要review， cap类的对象需要转移给ob企业钱包托管，
2. token类别， 需要添加部分split逻辑， 把大部分的token转入到ob企业钱包托管， 保留少部分token用于合约运行，冷热钱包概念和逻辑
3. 管理权限的清理， pause函数的清理， 
4. 关于token的精度问题， 现在默认都是按照精度8处理， 需要对齐一下eth上的token精度