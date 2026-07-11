# Rust 四种整数加法完整对比：saturating / wrapping / checked / overflowing
所有方法都是**无 panic 的溢出处理**，针对 `u8/u16/u32/u64/i8/i16/i32/i64` 等整数类型，核心差异在溢出时行为。

## 先统一前提
以 `u8`（范围 `0~255`）举例：`250u8 + 10`，正常和为 260，超出 u8 上限。

## 1. wrapping_add：环绕加法（模溢出，回绕）
### 规则
溢出按**二进制模 2ⁿ 回绕**，等价汇编无符号截断、有符号补码回绕，永远返回同类型整数，不会报错。
公式：`(a + b) % 2^bit_size`

示例 u8：
```rust
250u8.wrapping_add(10); // 260 % 256 = 4
5u8.wrapping_sub(10);   // -5 mod 256 = 251
```
有符号 i8：`120i8.wrapping_add(10)` → 溢出到负数。

适用：哈希、位运算、环形计数器，明确需要回绕逻辑。

## 2. saturating_add：饱和加法（钳位到边界）
### 规则
溢出时**停在类型最值**，不回绕、不报错：
- 正数上溢 → 取该类型最大值
- 负数下溢 → 取该类型最小值

```rust
250u8.saturating_add(10); // 上溢 → 255
0u8.saturating_sub(5);    // 下溢 → 0

120i8.saturating_add(20); // i8 max=127 → 127
-120i8.saturating_sub(20);// i8 min=-128 → -128
```
适用：血量、进度、计数、音量这类不能超出区间的数值。

## 3. checked_add：检查加法，返回 Option
### 规则
返回 `Option<T>`：
- 无溢出：`Some(sum)`
- 溢出：`None`

```rust
250u8.checked_add(10); // None
200u8.checked_add(10);// Some(210)
```
常用搭配 `let-else` / `?` 快速失败：
```rust
let Some(val) = 250u8.checked_add(10) else { return Err("溢出"); };
```
适用：业务校验，溢出属于非法输入，需要上层处理。

## 4. overflowing_add：返回 (和, 是否溢出)
### 规则
返回元组 `(T, bool)`
- 第一个值：和 wrapping_add 一样，是回绕后的真实存储值
- 第二个布尔：`true` = 发生溢出，`false` = 正常

```rust
let (val, overflow) = 250u8.overflowing_add(10);
// val = 4, overflow = true

let (v, of) = 200u8.overflowing_add(10);
// v = 210, of = false
```
适用：底层算法、密码学、硬件模拟，既要溢出标记又要截断结果。

# 四者核心对比表（u8 250+10）
| 方法               | 返回值类型       | 溢出行为                          | 典型场景                     |
|--------------------|------------------|-----------------------------------|------------------------------|
| wrapping_add       | u8               | 二进制模 256 回绕，得到 4         | 环形计数、哈希、底层位运算   |
| saturating_add     | u8               | 钳位到最大值 255                  | 血量、进度、音量、资源计数   |
| checked_add        | Option<u8>       | 溢出返回 None，正常 Some(sum)     | 业务参数校验、快速错误返回   |
| overflowing_add    | (u8, bool)       | (回绕值4, true)，同时给结果+标记  | 密码学、底层硬件仿真         |

# 补充重要知识点
1. **普通 `+` 运算符行为**
   Debug 模式溢出直接 panic；Release 模式等价 wrapping_add（回绕）。
   生产代码不要依赖原生 `+` 溢出，必须显式用上面四种方法。

2. 减法/乘法同理
   都配套：`wrapping_sub/mul`、`saturating_sub/mul`、`checked_sub/mul`、`overflowing_sub/mul`。

3. 有符号 vs 无符号区别
   - 无符号：下溢=绕到大数；saturating 下溢钳位到0
   - 有符号 iXX：上溢到最大值，下溢到最小值；wrapping 会正负翻转。