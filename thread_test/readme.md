1. https://www.bilibili.com/video/BV154dGYuEv6



Unpin informs the compiler that a given type does not need to uphold any guarantees about whether the value in question can be safely moved.

the compiler implements Unpin automatically for all types where it can prove it is safe

The notation for this is impl !Unpin for SomeType, where SomeType is the name of a type that does need to uphold those guarantees to be safe whenever a pointer to that type is used in a Pin.


First, Unpin is the “normal” case, and !Unpin is the special case. 
Second, whether a type implements Unpin or !Unpin only matters when you’re using a pinned pointer to that type like Pin<&mut SomeType>.
        
unPin 相当于 noNeedPin
即不需要pin，仍然是安全的
即可以自由移动

Unpin 是一个标记 trait（marker trait），就像我们在第十六章见过的 Send 和 Sync 一样，它本身没有任何功能。marker trait 的存在，只是为了告诉编译器：实现了该 trait 的类型，在某种特定上下文里可以被安全使用。Unpin 告诉编译器，某个类型不需要维护“这个值是否可以安全移动”方面的额外保证。