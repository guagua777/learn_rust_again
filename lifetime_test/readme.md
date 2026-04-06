1. 语义
```
// 生命周期的语义
var 'a; // 定义生命周期变量
'a = lifetime(m); // 'a 等于m的生命周期
if 'a > lifetime(n) { // 如果 'a 大于 n 的生命周期
    'a = lifetime(n);
}
return 'a;

```
2. struct中也可以定义生命周期
3. 

