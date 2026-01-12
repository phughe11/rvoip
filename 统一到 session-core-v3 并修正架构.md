# 统一到 session-core-v3 并修正架构
## 问题分析
当前存在三个 session-core 版本（v1, v2, v3）并存，且组件使用了错误的抽象层：
* **call-engine** 使用 session-core (v1) - 但它应该使用 **b2bua-core**（服务端架构）
* **client-core** 使用 session-core (v1) - 可迁移到 **session-core-v3**
* **sip-client** 使用 session-core (v1) - 可迁移到 **session-core-v3**
## 正确的架构
根据 `LIBRARY_DESIGN_ARCHITECTURE.md`：
* **session-core (v3)**: 用于 SIP 端点（软电话、客户端）
* **b2bua-core**: 用于服务端（呼叫中心、B2BUA）- 直接使用 dialog-core
* **call-engine**: 业务层，使用 b2bua-core 进行呼叫桥接
## 实施步骤
### Phase 1: 修正 call-engine 使用 b2bua-core
1.1 更新 call-engine/Cargo.toml - 移除 session-core 依赖，保留 b2bua-core
1.2 重构 orchestrator/core.rs - 使用 B2buaEngine 替代 SessionCoordinator
1.3 重构 orchestrator/calls.rs - 使用 B2buaCall 进行呼叫管理
1.4 更新事件处理 - 适配 b2bua-core 事件系统
### Phase 2: 迁移 client-core 到 session-core-v3
2.1 更新 client-core/Cargo.toml - 改用 session-core-v3
2.2 重构 ClientManager - 使用 SimplePeer API
2.3 更新事件模型 - 适配 v3 的 Event 类型
### Phase 3: 迁移 sip-client 到 session-core-v3
3.1 更新 sip-client/Cargo.toml
3.2 适配 API 调用
### Phase 4: 清理和验证
4.1 更新 rvoip/Cargo.toml - 移除 session-core (v1) 依赖
4.2 运行编译测试
4.3 清理编译警告
4.4 更新文档
## 风险
* 这是一个破坏性变更，需要仔细测试
* call-engine 的重构最为复杂
* 需要确保事件系统兼容
## 预估工时
* Phase 1: 4-6 小时
* Phase 2: 2-3 小时
* Phase 3: 1-2 小时
* Phase 4: 1-2 小时
