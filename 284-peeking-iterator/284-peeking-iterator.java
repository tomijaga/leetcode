// Java Iterator interface reference:
// https://docs.oracle.com/javase/8/docs/api/java/util/Iterator.html

class PeekingIterator implements Iterator<Integer> {
    Boolean peeked;
    Integer peekNext;
    Iterator<Integer> iterator;
    
	public PeekingIterator(Iterator<Integer> iterator) {
	    // initialize any member here.
	    this.iterator  = iterator;
        peeked = false;
        peekNext = null;
	}
	
    // Returns the next element in the iteration without advancing the iterator.
	public Integer peek() {
        if (peekNext== null){
            peekNext = iterator.next();
        }
        
        return peekNext;
	}
	
	// hasNext() and next() should behave the same as in the Iterator interface.
	// Override them if needed.
	@Override
	public Integer next() {
	    if (peekNext == null){
            return iterator.next();
        }
        
        Integer _next = peekNext;
        peekNext = null;
        
        return _next;
	}
	
	@Override
	public boolean hasNext() {
	    if(peekNext == null){
            return iterator.hasNext();
        }
        
        return true;
	}
}