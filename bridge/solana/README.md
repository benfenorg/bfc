# Solana Bridge 

### 环境准备
1. 安装solana-cli
   - 参考文档：https://docs.solana.com/cli/install-solana-cli-tools
2. 安装anchor
   - 参考文档：https://www.anchor-lang.com/docs/installation

### 部署准备工作


0. 部署合约，并指定合约大小，Solana合约最大10MB
    solana program deploy target/deploy/benfen_bridge.so --max-len 10485760
    solana program deploy --program-id target/deploy/benfen_bridge-keypair.json --max-len 10485760 target/deploy/benfen_bridge.so
1. 初始化步骤
   - initialize_bridge_config 
   - initialize_committee_with_config 
   - initialize_message_verifier
   - initialize_limit
   - initialize_benfen_bridge
   - initialize_upgrade_authority
### 开始测试
2. 运行升级测试：
   - add_token_with_signature
   - update_price_with_signature
   - update_blocklist_with_signature
   - cross_in
   - cross_out_with_signatures
### 升级测试
   1. 改好代码 编译 合约
   ```
   anchor build
   ```
   2. 写入到buffer,
   ```
   solana program write-buffer target/deploy/benfen_bridge.so
   ```
   3. 获得一个buffer地址 4uab8JHfmouQK3jqWK8RPF7QRcUD1tdJTHwJ9DQ92oaG
   4. 将权限地址给程序控制的地址
       solana program set-buffer-authority HHRdRBvq6pjoPLVXsYj2BPe4bNW4CV5ZLbitAZF3JzkB --new-buffer-authority 7nqbKtMwEBAZ4taNWrEcyHnLNaGxvoNKSrPF7czpuH6D
   5. 获取bridge node 的签名 调用指令升级
     调用 upgrade_program 指令
### 部署脚本准备
3. 部署脚本准备
   - 部署配置
   - 部署脚本测试
### 部署验证测试
4. 部署测试
   - 部署测试
   - 部署测试结果










