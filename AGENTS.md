# Agent 指令（desktop-pet）

本目录为 Windows 桌宠应用。**人类 Owner 负责合并与证书；Agent 负责实现与文档，且关键路径必须双子协作（Implementer → Reviewer）。**

## 必读

| 文档 | 用途 |
|------|------|
| [docs/agent-playbook.md](docs/agent-playbook.md) | 全生命周期、门禁、多 Agent 流程、OpenClaw 任务模板 |
| [docs/skin-authoring.md](docs/skin-authoring.md) | `rich-v1` 皮肤契约与 AI 提示词矩阵 |
| [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) | 本地开发与命令（随脚手架更新） |
| [docs/SECURITY.md](docs/SECURITY.md) | 隐私、密钥、威胁建模占位 |

## 冻结契约（未经「设计评审」双子流程不得改）

- **`coreProfile`**：`rich-v1`
- **强制状态键（缺一不可）**：`idle`，`walk`，`click`，`drag`，`sleep`，`wake`，`hover`，`happy`，`sad`，`surprised`，`wave`
- **扩展状态**：可出现在 `skin.json` 的 `states` 中；未实现时引擎回退（默认 `idle`）

## 执行纪律

1. **单任务单 PR**：一个可验证目标；禁止巨型混合修改。
2. **闸门前不准宣告完成**：必须贴出约定命令的输出（测试/lint/build）。
3. **禁止单会话既写又审**：Reviewer 须独立上下文，只看 diff + CI/命令日志。
4. **Human-only**：代码签名、证书密码、商店账号、任何真实 API 密钥——Agent 只写占位符与文档。

开始任务前打开 [docs/agent-playbook.md](docs/agent-playbook.md) 对应阶段表与 Work Item 模板。
