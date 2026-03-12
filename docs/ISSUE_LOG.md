
# Issue #52 - 屏幕震动和闪电特效不触发

## 问题描述
50 连击和 100 连击时，屏幕震动和闪电风暴特效没有触发。

## 根本原因
combo 计数器递增顺序错误：
- `updateCombo()` 函数中执行 `combo++`
- 但特效判断在 `updateCombo()` 之前
- 导致 `combo % 50 === 0` 和 `combo % 100 === 0` 永远不成立

## 修复方案
1. 将 `combo++` 移到 `incrementCount()` 函数开头
2. 重命名 `updateCombo()` 为 `updateComboDisplay()`（仅更新 UI）
3. 确保特效判断时 combo 已经是新值

## 提交记录
- Commit: `9cd9477`
- Message: `fix: screen shake and lightning effects not triggering (close #52)`

## 测试验证
- [x] 50 连击触发屏幕震动
- [x] 100 连击触发闪电风暴
- [x] 特效正常显示

## 状态
✅ 已修复并推送

---
