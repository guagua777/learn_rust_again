# CPS 转换（Continuation-Passing Style，续传风格）完整讲解
## 一、核心定义
**CPS 续传风格**：一种函数书写范式，**所有函数不直接返回结果**，而是多接收一个额外参数——**续体 continuation**（一个回调函数）；计算完成后，把结果传给这个续体，由续体决定下一步做什么。

普通直接风格（Direct Style）：函数 `return` 返回值
CPS 风格：函数把结果丢给传入的回调 `k(result)`，无 return。

### 最简对比示例（JS 伪代码）
1. 普通直接风格
```javascript
function add(a, b) {
  return a + b;
}
let res = add(1, 2);
console.log(res); // 3
```

2. CPS 改写
```javascript
// k = continuation 续体，接收计算结果
function addCps(a, b, k) {
  let sum = a + b;
  k(sum); // 不返回，把结果传给续体
}
// 调用：续体是打印逻辑
addCps(1, 2, res => console.log(res)); // 3
```

## 二、关键概念：Continuation 续体
**续体 = “剩下还没执行的代码”**
- 普通代码执行到 `add(1,2)` 时，续体是「拿到结果后打印」；
- CPS 把这段“后续逻辑”显式封装成函数，作为参数传入；
- 整个程序的执行流程完全由续体串联，不再依赖调用栈返回。

### 嵌套计算对比
普通风格：
```javascript
// ((1+2)*3)
let r1 = add(1,2);
let r2 = mul(r1,3);
console.log(r2);
```

CPS 链式嵌套（典型“回调地狱”形态）：
```javascript
addCps(1, 2, r1 => {
  mulCps(r1, 3, r2 => {
    console.log(r2);
  });
});
```
每一步的后续操作，都封装进下一层续体。

## 三、CPS 的核心特性
1. **没有返回语句**
   所有结果通过调用续体传递，`return` 完全消失。
2. **所有计算都是尾调用（Tail Call）**
   CPS 里每一步最后操作只有 `k(val)`，没有后续运算，天然是**尾调用**。
   配合尾调用优化 TCO，**程序不会栈溢出**，无限递归也能跑。
3. **程序控制流完全显式可控**
   续体只是普通函数，可以：
   - 调用 0 次：短路（类似异常、return）
   - 调用 1 次：正常流程
   - 调用多次：回溯、多分支、协程、异步
4. **消除栈帧依赖**
   普通语言靠调用栈保存上下文；CPS 把上下文全部打包进续体闭包，栈不再保存状态。

## 四、CPS 的典型用途（函数式语言核心场景）
### 1. 编译器中间表示（最重要用途）
OCaml、Scheme、Rust、ML 系列编译器都会做 **CPS 转换**，把源码统一转成 CPS IR：
- 统一处理控制流：if、循环、异常、return、break 全部用续体表达；
- 简化代码生成：所有调用都是尾调用，栈管理逻辑大幅简化；
- 优化更容易做：常量传播、内联、尾递归优化统一处理。

举个例子：用续体模拟 if 分支
```javascript
// 普通：if (x>0) a else b
// CPS版
function ifCps(cond, thenK, elseK, k) {
  cond(val => {
    if (val) thenK(k);
    else elseK(k);
  });
}
```

### 2. 实现异常、跳转、协程、回溯搜索
- **异常**：准备两套续体，正常续体 k / 异常续体 errK；出错时调用 errK 而非 k；
- **break/continue/goto**：提前保存外层循环续体，需要跳转时直接调用；
- **协程/生成器**：暂停时保存当前续体，恢复时重新调用；
- **回溯算法（Prolog）**：续体多次调用实现多解搜索。

### 3. 异步编程原生模型
所有异步回调本质就是 CPS：
```javascript
// Node.js 回调 = CPS
fs.readFile("a.txt", (err, data) => { /* continuation */ });
```
Promise / async-await 只是 CPS 的语法糖封装。

### 4. 实现无栈解释器、元循环求值器
Scheme 经典元循环解释器大量使用 CPS，不用硬件调用栈，手动管理所有执行上下文。

## 五、CPS 转换算法（把普通代码自动转 CPS）
通用转换规则（递归遍历表达式）：
1. **常量/变量**：不做计算，直接传给续体 `k(x)`
2. **函数调用 f(a,b)**：先把参数转 CPS，最后调用 fCps(a,b,k)
3. **lambda 函数**：新增续体参数 k，函数体内部全部 CPS 转换
4. **if 表达式**：条件求值后，分别给真假分支绑定各自续体

示例：递归阶乘直接风格转 CPS
直接版：
```ocaml
let rec fact n =
  if n = 0 then 1 else n * fact (n-1)
```
CPS 版：
```ocaml
let rec fact_cps n k =
  if n = 0 then k 1
  else fact_cps (n-1) (fun res -> k (n * res))
```
内层续体捕获 n，拿到子问题结果后相乘，再交给上层续体 k。

## 六、优缺点
### 优点
1. 完美统一所有控制流（分支、循环、异常、跳转）；
2. 天然尾调用，配合 TCO 杜绝栈溢出；
3. 控制流可自由操纵（暂停、回溯、异步）；
4. 编译器优化友好，IR 结构规整。

### 缺点
1. 手写可读性极差，多层嵌套形成“回调地狱”；
2. 代码膨胀严重，简单表达式会多出大量匿名续体；
3. 人类几乎不会手写大型 CPS 代码，仅编译器内部使用。

## 七、CPS 和其它函数式概念区分
1. **CPS vs 尾递归**
   尾递归只是“最后一步是调用自身”；CPS 是一种通用代码风格，所有调用都是尾调用，覆盖全部代码。
2. **CPS vs 单子(Monad)**
   - CPS：底层控制流变换，无副作用抽象；
   - Monad：抽象副作用（IO、状态、异常），可基于 CPS 实现；
   比如 Haskell 的 Cont 单子就是封装 CPS。
3. **CPS vs 回调函数**
   回调是 CPS 在异步场景的特例；CPS 是覆盖所有同步代码的通用范式。

## 八、Haskell 极简 Cont 单子示例（封装CPS）
```haskell
newtype Cont r a = Cont { runCont :: (a -> r) -> r }

-- 基础运算：把值丢给续体
pure a = Cont $ \k -> k a
(Cont f) >>= g = Cont $ \k -> f (\x -> runCont (g x) k)

-- 阶乘用Cont（CPS封装）
fact :: Int -> Cont Int Int
fact 0 = pure 1
fact n = fact (n-1) >>= \res -> pure (n * res)

runCont (fact 5) id -- 120
```
`Cont r a` 本质就是封装 `(a -> r) -> r`，也就是标准 CPS 类型签名。

## 总结
CPS（续传风格）是函数式语言底层核心技术：
1. 核心思想：不返回结果，把后续逻辑作为回调（续体）传入函数；
2. 本质：把程序隐式调用栈，全部转化为显式函数参数；
3. 主要落地：编译器中间表示、异步、异常、协程、无栈解释器；
4. 工程实践：人类极少手写，主要由编译器自动转换生成。





# 一、先理清两条核心关系
1. **CPS 本质**：把「后续要做的所有事」打包成回调函数（续体 `k`）传给函数，函数不 return，只调用 `k(结果)`。
   类型模板：`(value -> result) -> result`
2. **Cont Monad 本质**：把上面这个 CPS 类型包装成 Monad，复用 Monad 的 `pure / >>=` 统一串联 CPS 逻辑，不用手动嵌套回调。
3. **Monad 抽象副作用 vs CPS**
   - IO/State/Except 是**业务层面副作用抽象**：输入输出、全局状态、报错；
   - CPS / Cont 是**控制流底层抽象**：中断、回溯、提前返回、异步、跳转；
   - 所有带控制流的 Monad（异常、协程、异步）底层都能用 CPS 实现，Haskell `Cont` 就是直接封装 CPS 类型。

# 二、先用 JS 复刻 CPS 基础模型（无封装）
CPS 核心签名：`type ContR a = (k: (a) => R) => R`
一个值不直接暴露，而是接收一个回调，把值塞给回调。
```javascript
// 普通值：5
const normal = 5;

// CPS 包装后的 5：接收续体k，把5传给k
const cpsFive = (k) => k(5);

// 运行CPS：传入最终续体（打印）
cpsFive(x => console.log(x)); // 5
```

## 串联计算（不用Monad，纯手动嵌套，回调地狱）
计算 `(n-1)*n`，纯原生CPS：
```javascript
// 数字n的cps
const cpsNum = n => k => k(n);
// 乘法cps
const mulCps = (a, b, k) => k(a * b);
// 减1 cps
const sub1Cps = (x, k) => k(x - 1);

// 计算 5 * 4
cpsNum(5)(n => {
  sub1Cps(n, m => {
    mulCps(n, m, res => {
      console.log(res); // 20
    })
  })
})
```
嵌套层级爆炸，这就是手写CPS的痛点，**Cont Monad 就是用来抹平这种嵌套**。

# 三、JS 手动实现 Cont Monad（对应Haskell Cont）
Monad 需要两个接口：
1. `pure(a)`：把普通值包装成 Cont（CPS容器）
2. `bind(cont, f)`：串联两个CPS计算，对应Haskell `>>=`

### 完整实现
```javascript
// Cont 容器：保存 cps 函数 (k) => finalResult
class Cont {
  constructor(runCPS) {
    this.run = runCPS; // run = (k) => R
  }

  // 执行容器，传入最终续体k，拿到最终结果
  runCont(k) {
    return this.run(k);
  }
}

// pure：普通值 a 转 Cont a
// pure x = Cont(k => k(x))
const pure = (a) => new Cont(k => k(a));

// bind：对应 Haskell >>=
// 逻辑：
// 1. 先跑当前cont，拿到值x
// 2. 把x传给f，得到下一个Cont
// 3. 用外层续体k去跑下一个Cont
const bind = (cont, f) => {
  return new Cont(k => {
    cont.run(x => {
      const nextCont = f(x); // f: a => Cont b
      nextCont.run(k);
    })
  })
};
```

## 用 Cont Monad 改写上面乘法示例，消除嵌套
```javascript
// 减法：接收数字，返回 Cont
const sub1 = x => pure(x - 1);
// 乘法：接收两个数字，返回 Cont
const mul = (a, b) => pure(a * b);

// 计算 5 * 4
let computation = bind(pure(5), n => {
  return bind(sub1(n), m => mul(n, m));
});

// 执行，最终续体打印结果
computation.runCont(res => console.log(res)); // 20
```
多层计算只用链式bind，不用无限嵌套回调，这就是Monad的价值。

# 四、对应你贴的Haskell代码逐行翻译对照
## Haskell 原版
```haskell
newtype Cont r a = Cont { runCont :: (a -> r) -> r }

pure a = Cont $ \k -> k a
(Cont f) >>= g = Cont $ \k -> f (\x -> runCont (g x) k)

-- 阶乘
fact 0 = pure 1
fact n = fact (n-1) >>= \res -> pure (n * res)

runCont (fact 5) id -- 120
```

## 逐行 JS 映射
1. `newtype Cont r a = Cont { runCont :: (a -> r) -> r }`
   ↔ JS `class Cont { constructor(runCPS) { this.run = runCPS } }`
   内部存储的就是标准CPS函数 `(a => r) => r`

2. `pure a = Cont $ \k -> k a`
   ↔ JS `const pure = a => new Cont(k => k(a))`
   把普通值包成CPS容器

3. `(Cont f) >>= g = Cont $ \k -> f (\x -> runCont (g x) k)`
   ↔ JS `bind` 函数完全等价，`>>=` 就是bind中缀写法

4. 阶乘逻辑 JS 实现
```javascript
const fact = (n) => {
  if (n === 0) return pure(1);
  // bind：递归计算 n-1，拿到结果res后相乘
  return bind(fact(n - 1), res => pure(n * res));
};

// runCont 传 id 续体：x=>x，直接返回结果
fact(5).runCont(x => console.log(x)); // 120
```

# 五、关键：Cont Monad 和普通副作用Monad区别
## 1. IO / State / Except（业务副作用Monad）
- 解决**数据/外部副作用**：读写文件、修改状态、抛异常；
- 底层不一定依赖CPS，可直接用状态封装、错误标记实现。

## 2. Cont（控制流Monad，基于纯CPS）
- 解决**执行流程副作用**：提前返回、跳出循环、回溯、多分支、异步暂停；
- 底层**完全等价CPS转换**，所有逻辑都是传递续体；
- 可以用来实现异常、协程、goto、异步回调，是更底层的控制流工具。

### 例子：用Cont实现「提前return」（模拟异常/短路）
CPS续体可以选择不调用，直接短路逻辑，普通Monad很难做到：
```javascript
// 短路函数：直接忽略传入的k，返回固定值
const abort = (val) => new Cont(_k => val);

// 逻辑：n>3直接返回-1，否则返回n*2
const logic = (n) => bind(pure(n), x => {
  if (x > 3) return abort(-1);
  return pure(x * 2);
});

logic(5).runCont(console.log); // -1，提前终止，不执行后续续体
logic(2).runCont(console.log); // 4
```
这就是CPS独有的能力：**自由决定调用/不调用/多次调用续体**，普通顺序执行Monad做不到。

# 六、总结一句话
1. CPS 是一种代码书写范式：所有计算接收一个“后续操作回调（续体）”，无返回；
2. `Cont r a` 就是把 CPS 类型 `(a -> r) -> r` 封装成Monad容器；
3. Monad 的 `pure/bind` 提供统一接口串联多层CPS逻辑，消除原生CPS的回调嵌套；
4. IO/State/Except 是业务副作用抽象，Cont 是底层控制流抽象，后者基于CPS构建，还能用来实现前者的控制能力（异常、跳转、异步）。