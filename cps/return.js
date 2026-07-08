var identity = x => x;

var iden2 = x => {
    x
}

// 只要用大括号包裹后，就得写return
// 因为大括号里面，成一个一个的表达式了，如果不写return，就没有return的表达式
var iden3 = x => {
    return x
}


console.log(identity(5))
console.log(iden2(5))
console.log(iden3(5))
