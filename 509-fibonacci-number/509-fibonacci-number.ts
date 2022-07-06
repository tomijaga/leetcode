function fib(n: number): number {
    let fib = [0, 1]
    
    for (let i = 2; i<=n; i+=1){
        fib.push(fib[i - 1] + fib[i - 2])
    }
    
    return fib[n]
};