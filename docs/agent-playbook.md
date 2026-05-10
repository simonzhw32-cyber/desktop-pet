# Agent 研发手册（OpenClaw / 多 Agent）

面向：**主要由 Agent 实现**，人类做裁决与合并。目标是通过 **阶段闸门 + 双子 Agent（至少一执一审）** 降低单会话改崩主线的风险。

---

## 1. 总体原则

| 原则 | 说明 |
|------|------|
| **契约优先** | `coreProfile=rich-v1`、`skin.json` 字段、状态机键名变更须走「设计评审」双子流程，禁止静默改名。 |
| **小步合并** | 默认 **一个 PR 一个可验证目标**；避免巨型 PR 导致 Reviewer 漏检。 |
| **证据驱动** | 合并前必须具备 **可机器校验证据**：测试命令输出、lint、构建日志等。 |
| **上下文隔离** | Reviewer 不与 Implementer 共用「我已搞定」的独白；复审须基于 **diff + 闸门结果**。 |
| **人类兜底** | 隐私裁决、**代码签名**、商店账号：**Human-only**；Agent 只准备脚本与说明，不持有密钥。 |

---

## 2. 角色定义

| 角色 | 职责 | 禁止 |
|------|------|------|
| **Implementer** | 按 Work Item 改代码/配置、跑闸门命令、开 PR、写变更说明 | 自称已审查并绕过第二 Agent；擅自扩大 Scope |
| **Reviewer** | 对照验收标准审查 diff；输出 **Approve** 或 **Request changes**；列风险 | 未看闸门证据就 Approve；直接推 `main` |
| **Owner Human** | 合并、产品取舍、证书与上架 | — |
| **Arbiter（可选）** | Implementer 与 Reviewer 冲突时，仅基于双方摘要与客观日志裁决 | 不写实现 |

### 推荐编排流程

```text
WorkItem → Implementer（分支 + PR）→ CI/本地闸门命令 → Reviewer（独立会话）
         ↑______________________________________|
                    Request changes
```

---

## 3. 全生命周期阶段表

「**强制双子**」= 必须先 Implementer PR，再独立 Reviewer 审阅；通过后方可合并或进入下一阶段。

| 阶段 | 主要产出 | Agent 易翻车点 | 闸门（放行条件） | 强制双子 |
|------|----------|----------------|------------------|----------|
| **1. 需求与设计** | PRD、`rich-v1` 冻结、里程碑 A→E | 需求漂移、键名不一致 | 文档一致；Owner 确认无矛盾 | 是 |
| **2. 脚手架** | Tauri/Electron 选型、`README`、CI 骨架 | 选型摇摆；与本仓库其他项目混淆 | **build + lint + test（可为 smoke）** CI 绿 | 是 |
| **3. 产品开发** | 里程碑增量 | 透明窗穿透、DPI、皮肤校验、密钥落盘 | 每里程碑独立 PR + 自动化测试（哪怕很少） | 每 PR：是 |
| **4. 测试** | 单测/集成/手动清单 | 未跑命令自称已测 | CI 全绿；Reviewer 核对契约回归 | 测试变更：是 |
| **5. 上线** | 版本号、CHANGELOG、安装包 | SmartScreen；隐私声明缺失 | Release PR + 产物哈希；**签名 Human-only** | 是 |
| **6. 维护** | 依赖更新、安全补丁 | major bump 破坏兼容 | 双子 PR；破坏性变更须 bump `schemaVersion` 或新 profile 文档 | 是 |

---

## 4. 必须双子的环节（摘要）

- `skin.json` schema、`rich-v1` 强制键、校验器逻辑  
- 安全与隐私：密钥存储、网络、插件白名单、前台窗口标题等敏感能力  
- 窗口与输入：透明层级、click-through、全局热键  
- **每个里程碑合并 PR**、**Release PR**、**CI/workflow 变更**

---

## 5. 硬约束（防「单 Agent 一条龙」）

- 禁止在同一线程内「实现 + 审查 + 合并」并自称完成。  
- **`main` 建议分支保护**：仅 PR 合并；CI 必须通过。  
- 破坏性变更：独立 PR + Release Note；影响皮肤兼容须更新 `schemaVersion` 或发布新 `coreProfile` 文档。

---

## 6. OpenClaw Work Item 模板（复制使用）

每个任务一份；Implementer 关闭后 **必须** 新建 Reviewer 任务。

### 6.1 元数据

```yaml
work_item_id: "PET-000"  # 递增
title: "一句话标题"
core_profile: "rich-v1"  # 冻结，勿改除非走设计评审
```

### 6.2 正文（必填）

```markdown
## Goal
（一句话可验证目标）

## Scope
- **In scope**（路径 glob）: `desktop-pet/...`
- **Out of scope**: ...

## Acceptance criteria
- [ ] 条目 1（可观测）
- [ ] 条目 2

## Commands（闸门 — Implementer 须在 PR 描述中粘贴输出摘要）
- （示例，脚手架落地后替换为真实命令）
  - `pnpm lint` / `pnpm test` / `pnpm build`
  - 或 `cargo test` / `cargo clippy`

## Implementer prompt（仅实现）
（粘贴给 Implementer：约束、契约引用、禁止事项）

## Reviewer prompt（仅审查）
仅基于 PR diff + CI/命令日志。输出：
1. **Approve** 或 **Request changes**
2. 最多 **5 条**风险（安全、兼容、性能、可测性）
3. 核对 Acceptance criteria 是否逐项有据

## Stop condition
未满足 Acceptance / 闸门未绿 → 不得标记完成。
```

### 6.3 Reviewer 专用检查清单（快速）

- [ ] 是否触碰 `rich-v1` 键名或 `skin.json` 契约却未更新文档？  
- [ ] 是否引入密钥硬编码或日志泄露？  
- [ ] 是否缺少失败路径（皮肤加载失败、网络错误）？  
- [ ] PR 是否过大？能否拆分？  
- [ ] 测试是否覆盖「缺强制键则加载失败」类契约？

---

## 7. 与本项目契约对齐速查

- **`coreProfile`**：`rich-v1`  
- **强制键**：`idle`，`walk`，`click`，`drag`，`sleep`，`wake`，`hover`，`happy`，`sad`，`surprised`，`wave`  
- **扩展键**：可选；缺失时回退 `idle`（或可配置）  
- **详情**：[skin-authoring.md](skin-authoring.md)

---

## 8. Playbook 自身缺口（后续填满）

| 缺口 | 处理方式 |
|------|----------|
| 具体 lint/test/build 命令 | 脚手架 PR 合并后写入 [DEVELOPMENT.md](DEVELOPMENT.md) 与本节 Commands |
| CI 配置文件路径 | 首个 CI PR 后在本节增加链接 |
| 版本号策略 | SemVer；皮肤 `schemaVersion` 独立文档 |
| Issue/PR 模板 | 维护阶段添加 `.github/` |

---

## 9. 文档落库清单（维护责任）

| 文件 | 状态 |
|------|------|
| `AGENTS.md` | 入口索引 |
| `docs/agent-playbook.md` | 本文 |
| `docs/DEVELOPMENT.md` | 脚手架就绪后更新命令 |
| `docs/skin-authoring.md` | 创作者与提示词 |
| `docs/SECURITY.md` | 数据流与漏洞上报 |
