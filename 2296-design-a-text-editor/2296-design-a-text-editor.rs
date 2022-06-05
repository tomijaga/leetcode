class Node{
    public val: string;
    public next: Node | null = null;
    public prev: Node | null = null;

    constructor(val:string){
        this.val = val
    }
}

class TextEditor {
    public dummy: Node = new Node("");
    public node: Node;
    
    constructor(){
        this.node = this.dummy
    }

    addText(text: string): void {
        let node = null
        let prev = this.node
        let next = prev.next
        
        for (const c of text){
            node = new Node(c)
            node.prev = prev
            prev.next = node
            prev = node
        }
        
        
        node.next = next
        if (next){
            next.prev = node
        }
        
        this.node = node

    }

    deleteText(k: number): number {
        let i = 0;
        let next = this.node.next;
        
        while (i< k && this.node!=this.dummy){
            this.node = this.node.prev
            i+=1
        }
        
        
        this.node.next = next;
        if (next){
            next.prev = this.node
        }
        
        return i
    }

    cursorLeft(k: number): string {
        let i = 0;
        while (i < k && this.node.prev!=null && this.node!=this.dummy ){
            this.node = this.node.prev
            i+=1
        }
            
        
        let node = this.node
        let t = ""
        i = 0
        
        while(i < 10 && node != null) {
            i+=1
            t = node.val + t
            node = node.prev
        }
        // console.log(t)
        return t
    }

    cursorRight(k: number): string {
        let i = 0;
        while (i < k && this.node.next!=null){
            this.node = this.node.next
            i+=1
        }
        
        let node = this.node
        let t = ""
        i = 0
        
        while(i < 10 && node != null) {
            i+=1
            t = node.val + t
            node = node.prev
        }
        // console.log(t)
        return t
    }
}

/**
 * Your TextEditor object will be instantiated and called as such:
 * var obj = new TextEditor()
 * obj.addText(text)
 * var param_2 = obj.deleteText(k)
 * var param_3 = obj.cursorLeft(k)
 * var param_4 = obj.cursorRight(k)
 */