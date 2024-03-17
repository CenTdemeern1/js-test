() => {
    function hello(): number { return "Hi there!" }
    console.log(hello());
    // }]);
    console.log(hello());
    const f = [];
    let s = "_";
    while (true) {
        try { s = s + s; } catch { f.push(s) }
    }
}
