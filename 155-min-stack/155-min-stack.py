
class Node: 
    next = None
    val = 0
    min = 0
    
    def __init__(self, val: int):
        self.val = val

class MinStack:

    def __init__(self):
        self.node = None

    def push(self, val: int) -> None:
        
        node = Node(val)
        node.next = self.node
        if self.node == None:
            node.min = node.val
        else:
            node.min = min(self.node.min, node.val)
        self.node = node

    def pop(self) -> None:
        self.node = self.node.next

    def top(self) -> int:
        return self.node.val

    def getMin(self) -> int:
        return self.node.min


# Your MinStack object will be instantiated and called as such:
# obj = MinStack()
# obj.push(val)
# obj.pop()
# param_3 = obj.top()X
# param_4 = obj.getMin()