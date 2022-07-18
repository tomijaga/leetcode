function minCostClimbingStairs(cost: number[]): number {
    for (let i = 2; i< cost.length; i+=1){
        cost[i] = cost[i] + Math.min(cost[i-1], cost[i-2])
    }
    
    let len = cost.length
    return Math.min(cost[len - 1], cost[len - 2])
};