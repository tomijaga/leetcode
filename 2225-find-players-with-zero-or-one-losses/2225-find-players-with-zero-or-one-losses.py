class Solution:
    def findWinners(self, matches: List[List[int]]) -> List[List[int]]:
        losses = {}
        
        for [winner, loser] in matches:
            if winner not in losses:
                losses[winner] = 0
                
            if loser in losses:
                losses[loser] += 1
            else:
                losses[loser] = 1
                
        kings = []
        queens = []
        
        for (player, loss_cnt) in losses.items():
            if loss_cnt == 0:
                kings.append(player)
            elif loss_cnt == 1:
                queens.append(player)
                
        return [sorted(kings), sorted(queens)]
                