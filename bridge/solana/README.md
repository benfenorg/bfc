# Solana 跨链桥智能合约

本项目在 Solana 上实现了一个安全、去中心化的跨链桥，实现了 Solana 与 Benfen 链之间的资产互操作性。它采用基于委员会的验证机制来确保跨链消息的安全性和有效性。

## 主要特性

### 1. 跨链资产传输
- **转出 (cross_token_to_bridge)**：用户可以在 Solana 上锁定资产，以发起向目标链的资产转移。
- **转入 (cross_out)**：处理来自Benfen的经过验证的消息，在 Solana 上释放资产。
- **精度处理**：自动处理不同链之间的精度转换（例如 Solana 上的 6 位精度 vs Benfen 上的 9 位精度）。

### 2. 安全与验证
- **委员会共识**：需要来自受信任委员会成员的签名阈值（例如 2/3）才能验证任何跨链消息。
- **防重放保护**：使用唯一的 Nonce 和追踪账户（`ProcessTransfer`）来防止交易重放攻击。
- **secp256k1签名**：在链上验证签名，确保消息认证的安全性。

### 3. 风险管理
- **每日限额**：对每条链实施 24 小时滚动转账限额，以控制潜在的安全风险。
- **单笔限额**：可配置单笔交易的最大金额。
- **紧急控制**：
  - **暂停/恢复**：委员会可以在紧急情况下全局冻结跨链桥操作。

### 4. 治理与升级
- **自升级**：跨链桥程序可以通过委员会签名的消息进行升级，确保协议演进的同时避免中心化风险。
- **配置管理**：
  - 更新委员会成员

## 核心指令

- `initialize_*`：设置初始的跨链桥配置、委员会和限流器
- `cross_token_to_bridge`：从 Solana 发起跨链转账
- `process_transfer`：在验证委员会签名后执行传入的转账（释放/铸造）。
- `upgrade_program`：通过 `bpf_loader_upgradeable` 更新跨链桥程序代码。
- `add_token_to_bridge`：添加新代币支持
- `update_emergency_op`：暂停/恢复跨链桥操作
- `update_token_price`：更新代币价格（需要委员会签名）
- `update_block_list`：更新黑名单（需要委员会签名）
- `extend_program`：扩展程序空间，用于存储新的配置或状态
- `cross_out`：处理来自Benfen的经过验证的消息，在 Solana 上释放资产。
- `update_bridge_limiter`：更新跨链桥的限额配置（需要委员会签名）
- `update_single_transfer_limit`：更新单笔交易限额（需要委员会签名）
- `transfer_upgrade_authority`：转移升级权限给合约

## 架构

该系统架构参照EVM合约方式来实现，并结合Solana链的特性来设计，系统依赖多个程序派生地址（PDA）来管理状态：
- **BridgeConfig**：存储全局设置（链 ID、支持目标链）。
- **Committee**：存储委员会成员和签名阈值。
- **ChainLimit**：跟踪每日交易量和限额。
- **MessageConfig**：存储消息配置。
- **MessageVerifier**：管理消息验证状态。
- **TokenConfigAccount**： 存储代币配置（精度、小数位等）
- **ProcessTransfer**：跟踪已处理的 Nonce，防止双花。
- **UpgradeAuthority**：存储升级权限，确保只有授权的地址才能升级合约


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
   3. 获得一个buffer地址 3MWj5y6LFUu89utM7jxF99tBNty6oxzXpQybwVwsGt4r
   4. 将权限地址给程序控制的地址
       solana program set-buffer-authority 3MWj5y6LFUu89utM7jxF99tBNty6oxzXpQybwVwsGt4r --new-buffer-authority 7nqbKtMwEBAZ4taNWrEcyHnLNaGxvoNKSrPF7czpuH6D
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










