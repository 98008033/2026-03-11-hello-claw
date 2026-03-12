
# Issue #2 - 连击特效问题修复

## 问题描述
1. combo 计数 2 秒后清零
2. 50 连击闪电特效不可见
3. 100 连击闪电风暴不可见
4. 每 10 连击背景不变色
5. 重置后背景不恢复

## 根本原因
1. `updateComboDisplay()` 中定时器错误地将 `combo = 0` 清零
2. `.lightning-bolt` 和 `.lightning-branch` 的 z-index 为 5，被 z-index 999 的 `.bg-effects` 容器遮挡

## 修复方案
1. 移除定时器中的 `combo = 0`，combo 只在 resetCount() 时清零
2. 提高闪电图层 z-index 从 5 到 1000
3. 确保背景颜色恢复逻辑正确

## 提交记录
- Commit: `8e6ac7f`
- Message: `fix: 修复连击特效问题 (#2)`

## 测试验证
- [x] combo 持续增长不自动清零
- [x] 50 连击可见闪电特效
- [x] 100 连击可见闪电风暴
- [x] 每 10 连击背景变色
- [x] 重置后背景恢复

## 状态
✅ 已修复并推送

---

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
