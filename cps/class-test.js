class Dot {
    constructor(f) {
        this.f = f;
    }
}

var dot = new Dot(x => console.log(x))

dot.f(5)

