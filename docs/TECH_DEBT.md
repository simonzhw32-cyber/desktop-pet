# TECH_DEBT.md

记录 Reviewer 标记的 minor 问题，后续顺手修复或单独 WI。

## PET-002 Reviewer 标记项

| # | 问题 | 来源 | 优先级 | 处理建议 |
|---|------|------|--------|----------|
| 1 | 路径遍历风险（`..` 未检查） | PET-002 Reviewer | 中 | ✅ PET-003 已修复（基础版） |
| 2 | CSP 仍为 null | PET-001/002 Reviewer | 低 | 发布前配置，PET-005 或 Release WI |
| 3 | 未校验 PNG 格式有效性 | PET-002 Reviewer | 低 | ✅ PET-005 已修复 |
| 4 | 未校验帧尺寸与 canvas 声明一致 | PET-002 Reviewer | 低 | ✅ PET-005 已修复 |

## PET-003 Reviewer 标记项

| # | 问题 | 来源 | 优先级 | 处理建议 |
|---|------|------|--------|----------|
| 5 | safe_path_join 未做 canonicalize 校验 | PET-003 Reviewer | 低 | 安全加固阶段 |
| 6 | 前端帧推进（frameIndexRef）与 Rust advance_frame 并存 | PET-003 Reviewer | 中 | 后续统一为一种模式 |
| 7 | playFrame async + setInterval 可能帧堆积 | PET-003 Reviewer | 中 | ✅ PET-004 Scope 中修复 |
| 8 | Loading 占位硬编码 512×512 | PET-003 Reviewer | 低 | ✅ PET-006 已修复 |

---

*Updated by Mochi @ 2026-05-10*